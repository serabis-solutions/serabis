use std::fs::read_dir;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use rand;
use rand::distributions::{IndependentSample, Range};

use client;

pub struct Plugin {
    pub name : String,
    path     : String,
}
impl Plugin {
    pub fn new( name: &str, path: &Path ) -> Plugin {
        debug!( "loaded {}", &name );
        Plugin {
            name    : name.to_owned(),
            path    : path.to_str().unwrap().to_owned(),
        }
    }

    // take ownership of self and move it into the new thread
    pub fn run( self, client: Arc<client::Client> ) -> JoinHandle<()> {
        thread::spawn( move || {
            let mut rng = rand::thread_rng();
            let splay_range = Range::new( 0, 59 );
            let splay = splay_range.ind_sample( &mut rng );
            debug!("{} splay {}s", &self.name, &splay );

            thread::sleep( Duration::from_secs( splay ) );

            loop {
                let output = Command::new( &self.path )
                    .output()
                    .unwrap_or_else(|e| { die!("failed to execute process, `{}`: {}", &self.path, e) });

                // XXX streaming output (eg inotify watcher)
                let stdout = String::from_utf8( output.stdout ).unwrap().trim().to_owned();
                client.report( &self.name, &stdout );

                //XXX should be configurable?
                thread::sleep( Duration::from_secs( 60 ) );
            };
        } )
    }
}

pub fn load_all( plugin_path: &Path ) -> Vec<Plugin> {
    debug!("finding plugins in {:?}", &plugin_path );
    let files = match read_dir( &plugin_path ) {
        Ok(e)  => e,
        Err(e) => die!( "foo{}", e ),
    };

    let mut plugins = Vec::new();

    for file in files {
        let file_path = file.unwrap().path();
        if !file_path.is_file() || ! file_path.extension().is_none() {
            debug!("ignoring {:?}", &file_path );
            continue;
        }

        debug!("loading {:?}", &file_path );
        //unwrap because there might be no name, but there can't be no name?!?
        let name = file_path.file_stem().unwrap().to_str().unwrap();
        plugins.push( Plugin::new( name, &file_path ) );
    }

    plugins
}
