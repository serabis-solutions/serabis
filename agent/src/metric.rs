use time;
use std::fs;
use serde;
use std::path;
use std::io::Read;
use serde_json;
use serde::Serialize;

quick_error! {
    #[derive(Debug)]
    pub enum MetricError {
        IO(err: ::std::io::Error) {
            from()
            description(err.description())
        }
        JSON( err: serde_json::Error ) {
            from()
            description(err.description())
        }
    }
}

trait SerializeWith: Sized {
    fn serialize_with<S>(&self, ser: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer;
}
trait DeserializeWith: Sized {
    fn deserialize_with<D>(de: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer;
}

impl SerializeWith for time::Tm {
    fn serialize_with<S>(&self, ser: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        try!( self.to_timespec().sec.serialize( ser ) );

        Ok( () )
    }
}
impl DeserializeWith for time::Tm {
    fn deserialize_with<D>(de: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        let seconds : i64 = try!(serde::Deserialize::deserialize(de));
        Ok( time::at( time::Timespec::new( seconds, 0 ) ) )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    #[serde(serialize_with="SerializeWith::serialize_with", deserialize_with="DeserializeWith::deserialize_with")]
    pub timestamp : time::Tm,
    #[serde(rename="type")]
    pub name      : String,
    pub data      : serde_json::Value,
}

impl Metric {
    pub fn new( name: &str, data: &str ) -> Result<Self, MetricError> {
        let metric = Metric {
            timestamp   : time::now_utc(),
            name        : name.to_owned(),
            data        : try!( serde_json::from_str( data ) ),
        };

        Ok( metric )
    }

    pub fn new_from_file( path: &path::PathBuf ) -> Result<Self, MetricError> {
        trace!( "loading metric from file {}", path.display() );
        let mut f = try!( fs::File::open( path ) );
        let mut s = String::new();
        try!( f.read_to_string( &mut s ) );
        let deserialized: Metric = try!( serde_json::from_str( &s ) );

        Ok( deserialized )
    }

    pub fn serialize(&self) -> Result<String, MetricError> {
        Ok( try!( serde_json::to_string( &self ) ) )
    }
}
