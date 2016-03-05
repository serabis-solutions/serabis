//use std::process::Command;
use std::time::Duration;
use std::sync::Arc;
use std::thread;
use hyper;

pub struct Worker {
    command : String,
    args    : Vec<String>,
    hyper   : Arc<hyper::Client>,
}

impl Worker {
    pub fn new( command: &str, args: Vec<&str>, client: Arc<hyper::Client> ) -> Worker {
        Worker {
            command : command.to_owned(),
            args    : args.iter().map( |&s| s.to_owned() ).collect::<Vec<String>>(),
            hyper   : client,
        }
    }

    pub fn start( &self ) {
        loop {
            let body = [&self.command, "=", &self.args.join(",") ].concat();
            println!("{}", &body );

            let res = self.hyper.post("http://0:5000/")
                .body( "body" )
                .send()
                .unwrap();

            assert_eq!(res.status, hyper::Ok);
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
