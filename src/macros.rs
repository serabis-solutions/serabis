#[macro_use]
pub mod macros {
    macro_rules! die {
        () => ({
            use std;
            println!( "Died at {} line {}", file!(), line!() );
            std::process::exit(255);
        });
        ($fmt:expr) => ({
            use std;
            println!( $fmt );
            std::process::exit(255);
        });
        ($fmt:expr, $($arg:tt)+) => ({
            use std;
            println!( $fmt, $($arg)+ );
            std::process::exit(255);
        });
    }
}
