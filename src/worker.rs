    //use std::process::Command;
    use std::time::Duration;
    use std::thread;

    pub struct Worker {
        command : String,
        args    : Vec<String>,
    }

    impl Worker {
        pub fn new( command: &str, args: Vec<&str> ) -> Worker {
            Worker {
                command: command.to_owned(),
                args   : args.iter().map( |&s| s.to_owned() ).collect::<Vec<String>>(),
            }
        }

        pub fn start( &self ) {
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
