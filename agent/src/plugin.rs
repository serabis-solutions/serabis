use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::time::Duration;

use std::process::{Command, Stdio};
use pine;
use pine::Line;

use config::PluginConfig;
use config_loader::{Loader, ConfigLoadError};

use client;
use metric;

use quick_error::ResultExt;

#[cfg(feature = "short_splay")]
const SPLAY_MAX : u64 = 6;

#[cfg(not(feature = "short_splay"))]
const SPLAY_MAX : u64 = 60;

//XXX these errors should have some more context, like the plugin name
quick_error! {
    #[derive(Debug)]
    pub enum PluginError {
        MetricError( err: metric::MetricError ) {
            from()
        }
        IO(err: ::std::io::Error) {
            from()
        }
        FileContext(filename: PathBuf, err: ::std::io::Error) {
            context(path: &'a Path, err: ::std::io::Error)
                -> (path.to_path_buf(), err)
            display( "plugin load error [{} - {}]", filename.display(), err )
        }
        FileStrContext(filename: String, err: ::std::io::Error) {
            context(path: &'a str, err: ::std::io::Error)
                -> (path.to_owned(), err)
            display( "plugin load error [{} - {}]", filename, err )
        }
        ConfigLoadError(err: ConfigLoadError) {
            from()
            display( "{}", err )
        }
        ClientError( err: client::ClientError ) {
            from()
            display( "{}", err )
        }
        CommandNotFound( err: String ) {
            from()
            display( "{}", err )
        }
    }
}

pub struct Plugin {
    pub name : String,
    splay    : Duration,
    config   : PluginConfig
}

impl Plugin {
    pub fn new( name: &str, plugin_config_path: &Path ) -> Result<Plugin, PluginError> {
        info!( "loading {}", &name );

        info!( "loading {} config", plugin_config_path.display() );
        let mut config = try!( PluginConfig::new_from_file( plugin_config_path ) );
        trace!( "{:?}", &config );

        let mut rng = rand::thread_rng();
        let splay_range = Range::new( 0, SPLAY_MAX );
        let splay = splay_range.ind_sample( &mut rng );
        let splay_duration = Duration::from_secs( splay );


        if config.command.is_relative() {
            trace!( "{} - command is relative", &name );

            let found_command_path = try!( Self::find_command( &name, &config.command, &plugin_config_path ) );
            config.command = found_command_path;
        }

        let plugin = Plugin {
            name    : name.to_owned(),
            splay   : splay_duration,
            config  : config,
        };

        Ok(plugin)
    }

    fn find_command( name: &str, command: &PathBuf, plugin_config_path: &Path )
        -> Result<PathBuf, PluginError>
    {
        let mut current_path = Some( plugin_config_path.to_path_buf() );

        while let Some( working_path ) = current_path {
            let parent_dir = working_path.parent();
            if let Some(parent_dir) = parent_dir {
                let new_path = &parent_dir.join( &command );
                if new_path.exists() {
                    trace!(
                        "{} - found {}/{}",
                        &name,
                        &parent_dir.display(),
                        &command.display()
                    );

                    return Ok( new_path.to_path_buf() );
                }
            }
            else {
                trace!("no parent dir. odd");
                let err = format!(
                    "{} - looking for {} failed. reached fs root. giving up",
                    &name,
                    &command.display()
                );
                return Err( PluginError::CommandNotFound( err ) );
            }

            current_path = working_path.read_link().ok();
        }

        let err = format!( "{} - looking for {}. not found. giving up", &name, &command.display() );
        Err( PluginError::CommandNotFound( err ) )
    }

    // take ownership of self
    pub fn run( self, client: Arc<client::Client> ) -> Result<(), PluginError> {
        info!("{} splaying for {}s", &self.name, &self.splay.as_secs() );
        thread::sleep( self.splay );

        loop {
            info!("{} running {:?}", &self.name, &self.config.command);

            let mut process = try!(
                Command::new( &self.config.command )
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context( self.config.command.as_path() )
            );

            trace!("{} reading lines", &self.name);

            let lines = pine::lines(&mut process);
            for line in lines.iter() {
                match line {
                    Line::StdOut(line) => {
                        let metric = try!( metric::Metric::new( &self.name, line.trim() ) );
                        try!( client.submit_metric( metric ) );
                    },
                    Line::StdErr(line) => error!("{} stderr: {}", &self.name, line.trim_right() ),
                };
            }

            //process should have finished running now, so read the exit status so we don't have
            //defunct proccess
            let exit_status = try!( process.wait() );
            if !exit_status.success() {
                //XXX this shouldn't be a die. it should be Err
                die!("{} {}", &self.name, exit_status );
            }

            trace!( "{} sleeping for {}s", &self.name, &self.config.timeout.as_secs() );

            thread::sleep( self.config.timeout );
        };
    }
}

pub fn load_all( plugin_path: &Path ) -> Result<Vec<Plugin>, PluginError> {
    info!("finding plugins in {:?}", &plugin_path );
    let files = try!( read_dir( &plugin_path ).context( plugin_path ) );

    let mut plugins = Vec::new();

    for file in files {
        let file_path = try!( file ).path();

        let correct_filetype : bool = {
            match file_path.extension() {
                Some(v) => v == "plugin",
                None    => false,
            }
        };

        if !file_path.is_file() || !correct_filetype {
            trace!("ignoring {:?}", &file_path );
            continue;
        }

        //how can there be no name?!?
        if let Some(name) = file_path.file_stem() {
            if let Some(str_name) = name.to_str() {
                trace!("found {}", str_name );

                let plugin = try!( Plugin::new( str_name, &file_path ) );
                plugins.push( plugin );
            }
        }
    }

    Ok(plugins)
}
