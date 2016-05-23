use std::fs::read_dir;
use std::path::Path;
use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use rand;
use rand::distributions::{IndependentSample, Range};

use std::process::{Command, Stdio};
use pine;
use pine::Line;

use client;

pub struct Plugin {
    pub name : String,
    path     : String,
}
impl Plugin {
    pub fn new( name: &str, path: &Path ) -> Plugin {
        info!( "loaded {}", &name );
        Plugin {
            name    : name.to_owned(),
            path    : path.to_str().unwrap().to_owned(),
        }
    }

    // take ownership of self and move it into the new thread
    pub fn run( self, client: Arc<client::Client> ) -> JoinHandle<()> {
        thread::Builder::new().name( self.name.to_string() ).spawn(move || {
            let mut rng = rand::thread_rng();
            let splay_range = Range::new( 0, 59 );
            let splay = splay_range.ind_sample( &mut rng );
            info!("{} splay {}s", &self.name, &splay );

            thread::sleep( Duration::from_secs( splay ) );

            loop {
                let mut process = Command::new( &self.path )
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap_or_else(|e| { die!("failed to execute process, `{}`: {}", &self.path, e) });

                let lines = pine::lines(&mut process);
                for line in lines.iter() {
                    match line {
                        Line::StdOut(line) => client.report( &self.name, line.trim() ),
                        Line::StdErr(line) => die!("err -> '{}'", line.trim_right() )
                    }
                }

                //XXX this should be configurable
                thread::sleep( Duration::from_secs( 60 ) );
            };
        } ).unwrap()
    }
}

pub fn load_all( plugin_path: &Path ) -> Vec<Plugin> {
    info!("finding plugins in {:?}", &plugin_path );
    let files = match read_dir( &plugin_path ) {
        Ok(e)  => e,
        Err(e) => die!( "foo{}", e ),
    };

    let mut plugins = Vec::new();

    for file in files {
        let file_path = file.unwrap().path();
        if !file_path.is_file() || ! file_path.extension().is_none() {
            info!("ignoring {:?}", &file_path );
            continue;
        }

        trace!("found {:?}", &file_path );
        //XXX check for executable
        //unwrap because there might be no name, but there can't be no name?!?
        let name = file_path.file_stem().unwrap().to_str().unwrap();
        plugins.push( Plugin::new( name, &file_path ) );
    }

    plugins
}
