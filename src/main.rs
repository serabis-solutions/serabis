extern crate rustc_serialize;
extern crate toml;
#[macro_use] extern crate log;

mod config;

//use std::process::Command;
use std::thread;
use std::time::Duration;

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

macro_rules! exit {
    ($fmt:expr) => ({
        println!( $fmt );
        std::process::exit(1);
    });
    ($fmt:expr, $($arg:tt)+) => ({
        println!( $fmt, $($arg)+ );
        std::process::exit(1);
    });
}

fn main() {
warn!("here");
    let m = config::Monitor::parse( "/etc/serapis/monitor.toml" );

    println!( "{:?}", m );

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
