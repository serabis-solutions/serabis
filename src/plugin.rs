use std::fs::read_dir;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use std::sync::Arc;
use std::thread;
use hyper;

#[derive(Debug)]
pub struct Plugin {
    name    : String,
    path    : String,
}
impl Plugin {
    pub fn new( name: &str, path: &Path ) -> Plugin {
        Plugin {
            name    : name.to_owned(),
            path    : path.to_str().unwrap().to_owned(),
        }
    }

    pub fn run( &self, client: Arc<hyper::Client> ) {
        loop {
//            let body = [ "data", "=", self.path.as_str() ].concat();

//
    //      https://stackoverflow.com/questions/26550962/how-would-you-stream-output-from-a-process-in-rust
            let output = Command::new( &self.path )
                .output()
                .unwrap_or_else(|e| { die!("failed to execute process, `{}`: {}", &self.path, e) });

            let encoded = String::from_utf8( output.stdout ).unwrap();
            print!("{}", encoded);
            let res = client.post("http://0:5000/")
                .body( &encoded )
                .send()
                .unwrap();
            assert_eq!(res.status, hyper::Ok);

            thread::sleep(Duration::from_millis(10000));
        };
    }
}

pub fn find_plugins( plugin_path: &Path ) -> Vec<Plugin> {
    let files = match read_dir( &plugin_path ) {
        Ok(e)  => e,
        Err(e) => die!( "foo{}", e ),
    };

    let mut plugins = Vec::new();

    for file in files {
        let file_path = file.unwrap().path();
        if !file_path.is_file() || ! file_path.extension().is_none() {
            continue;
        }

        //unwrap because there might be no name, but there can't be no name?!?
        let name = file_path.file_stem().unwrap().to_str().unwrap();
        plugins.push( Plugin::new( name, &file_path ) );
    }

    plugins
}
