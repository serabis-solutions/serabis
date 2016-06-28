use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use serde::Deserialize;
use quick_error::ResultExt;
use std::path::{Path, PathBuf};
use std::error::Error;

quick_error! {
    #[derive(Debug)]
    pub enum ConfigLoadError {
        File(filename: PathBuf, err: ::std::io::Error) {
            context(path: &'a Path, err: ::std::io::Error)
                -> (path.to_path_buf(), err)
            display( "config load error [{} - {}]", filename.display(), err )
            cause( err )
        }
        Decode(filename: PathBuf, err: ::toml::DecodeError) {
            context(path: &'a Path, err: ::toml::DecodeError)
                -> (path.to_path_buf(), err)
            display( "config decode error [{} - {}]", filename.display(), err )
            cause( err )
        }
        Parse(filename: PathBuf, err: String) {
            display( "{} - {}", filename.display(), err )
        }
    }
}

pub trait Loader : Deserialize + Sized {
    fn new_from_file(path: &Path) -> Result<Self, ConfigLoadError> {
        let mut file = try!( File::open(path).context(path) );

        let mut config_toml = String::new();
        try!( file.read_to_string(&mut config_toml).context(path) );

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            //there could be more errors, but just report the first one
            let err = &parser.errors[0];
            let (loline, _) = parser.to_linecol(err.lo);
            return Err( ConfigLoadError::Parse(
                path.to_path_buf(),
                format!( "{} line {}", err.description(), loline + 1 )
            ) );
        }

        //unwrap here because it's an option and we know it didn't error
        //XXX is there a better way to do this?
        let mut toml_decoder = Decoder::new( Value::Table( toml.unwrap() ) );

        Ok( try!( Deserialize::deserialize( &mut toml_decoder ).context(path) ) )
    }
}


