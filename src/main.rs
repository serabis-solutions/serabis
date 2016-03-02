use std::process::Command;
use std::thread;
use std::time::Duration;

extern crate toml_loader;
use toml_loader::Loader;
use toml_loader::LoadError;
use std::path::Path;

struct Worker {
    command : String,
    args    : Vec<String>,
}

impl Worker {
    fn new( command: &str, args: Vec<&str> ) -> Worker {
        Worker {
            command: command.to_owned(),
            args   : args.iter().map( |&s| s.to_owned() ).collect::<Vec<String>>(),
        }
    }

    fn start( &self ) {
        println!("command is '{}'", self.command);
        println!("args are {:?}", self.args);

        loop {

            thread::sleep(Duration::from_millis(10000));
//
//    //      https://stackoverflow.com/questions/26550962/how-would-you-stream-output-from-a-process-in-rust
//            let output = Command::new( &self.command )
//                .args( &self.args )
//                .output()
//                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
//
//            let encoded = String::from_utf8( output.stdout ).unwrap();
//            print!("{}", encoded);
        };
    }
}

fn main() {
    let config_file = Path::new( "/etc/serapis/monitor.toml" );

    // XXX this is terrible, but it works and i'll learn how to improve it somewhere along the line
    // XXX handle the Io error better, probably with a default config
    let config = match Loader::from_file( &config_file ) {
        Ok(s) => { s },
        Err(LoadError::Parse(e)) => panic!("failed to parse config, {}", e[0]),
        Err(LoadError::Io(e)) => panic!("failed to open config file {}, {}", &config_file.to_str().unwrap(), e.to_string()),
    };
    println!( "{}", &config );

    let workers = vec![
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
    ];

    let handles: Vec<_> = workers.into_iter().map(|p| {
        thread::spawn(move || {
            p.start();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
