use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};

use std::process::{Command, Stdio};
use pine;
use pine::Line;
use serde::{Deserialize, Deserializer};

#[cfg(feature = "short_splay")]
const SPLAY_MAX : u64 = 6;

#[cfg(not(feature = "short_splay"))]
const SPLAY_MAX : u64 = 60;

use client;

pub struct Plugin {
    pub name : String,
    path     : PathBuf,
    splay    : Duration,
    config   : PluginConfig
}

trait MyDefault {
    fn default() -> Self;
}
trait MyDeserialize : Sized {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer;
}

type Timeout = Duration;
impl MyDefault for Timeout {
    fn default() -> Self {
        Duration::from_secs( 60 )
    }
}
impl MyDeserialize for Timeout {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let deserialized : u64 = try!(Deserialize::deserialize(de));

        Ok( Timeout::from_secs( deserialized ) )
    }
}

type CommandPath = PathBuf;
impl MyDeserialize for CommandPath {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let deserialized : String = try!(Deserialize::deserialize(de));

        Ok( CommandPath::from( deserialized ) )
    }
}

#[derive(Deserialize)]
struct PluginConfig {
    #[serde(default="MyDefault::default", deserialize_with="MyDeserialize::deserialize")]
    timeout  : Timeout,
    #[serde(deserialize_with="MyDeserialize::deserialize")]
    command  : CommandPath,
}

impl Plugin {
    pub fn new( name: &str, path: &Path ) -> Plugin {
        info!( "loading plugin {}", &name );

        let mut config_toml = String::new();

        let mut file = File::open(&path)
            .unwrap_or_else(|err| panic!("Error opening file: [{}]", err));

        file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{:?}:{}:{}-{}:{} error: {}",
                         path, loline, locol, hiline, hicol, err.desc);
            }
            panic!("Exiting");
        }

        let mut config = Decoder::new( Value::Table( toml.unwrap() ) );

        let mut rng = rand::thread_rng();
        let splay_range = Range::new( 0, SPLAY_MAX );
        let splay = splay_range.ind_sample( &mut rng );
        let splay_duration = Duration::from_secs( splay );

        Plugin {
            name    : name.to_owned(),
            path    : path.to_path_buf(),
            splay   : splay_duration,
            config  : Deserialize::deserialize( &mut config ).expect("helpful error")
        }
    }

    // take ownership of self and move it into the new thread
    pub fn run( self, client: Arc<client::Client> ) -> JoinHandle<()> {
        thread::Builder::new().name( self.name.to_string() ).spawn(move || {
            info!("{} splaying for {}s", &self.name, &self.splay.as_secs() );
            thread::sleep( self.splay );

            loop {
                info!("{} running {:?}", &self.name, &self.config.command);
                let mut process = Command::new( &self.config.command )
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap_or_else(|e| { die!("{} failed to execute process, {:?}: [{}]", &self.name, &self.config.command, e) });

                trace!("{} reading lines", &self.name);
                let lines = pine::lines(&mut process);
                for line in lines.iter() {
                    match line {
                        Line::StdOut(line) => client.report( &self.name, line.trim() ),
                        Line::StdErr(line) => die!("err -> '{}'", line.trim_right() )
                    }
                }

                trace!( "{} sleeping for {}s", &self.name, &self.config.timeout.as_secs() );

                thread::sleep( self.config.timeout );
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

        trace!("found {:?}", &file_path );
        //XXX check for executable
        //unwrap because there might be no name, but there can't be no name?!?
        let name = file_path.file_stem().unwrap().to_str().unwrap();
        plugins.push( Plugin::new( name, &file_path ) );
    }

    plugins
}
