use hyper;
use hyper::header::{Headers, ContentType};
use std::sync::Arc;
use std::time::Duration;
use config;
use metric;
use std::io::prelude::*;
use std::fs;
use quick_error::ResultExt;

const API_VERSION: &'static str = "0.01";

quick_error! {
    #[derive(Debug)]
    pub enum ClientError {
        Hyper(err: hyper::Error) {
            from()
            display( "{}", err )
        }
        FileContext(filename: String, err: ::std::io::Error) {
            context(path: &'a str, err: ::std::io::Error)
                -> (path.to_owned(), err)
            context(path: &'a String, err: ::std::io::Error)
                -> (path.to_owned(), err)
            display( "plugin cache error [{} - {}]", filename, err )
        }
        Metric( err: metric::MetricError ) {
            from()
        }
    }
}

pub struct Client {
    config  : Arc<config::AgentConfig>,
    hyper   : Arc<hyper::Client>,
}

impl Client {
    pub fn new( config: Arc<config::AgentConfig> ) -> Client {
        let mut client = hyper::Client::new();
        client.set_read_timeout( Some( Duration::from_secs( 10 ) ) );
        client.set_write_timeout( Some( Duration::from_secs( 10 ) ) );

        Client {
            config  : config,
            hyper   : Arc::new(client),
        }
    }

    pub fn submit_metric( &self, metric: metric::Metric ) -> Result<(), ClientError> {
        match self._post_metric( &metric ) {
            Ok(res) => {
                info!( "{}: report {}", &metric.name, res.status );
                //XXX check the error type, we might need to handle other errors, such as 404 etc
                //301 ?
                //404/500?
            }
            Err(e) => {
                info!( "{} - failed to post metric [{}]", &metric.name, e );

                try!( self.cache_metric( metric ) );
            }
        };

        Ok( () )
    }

    pub fn _post_metric( &self, metric: &metric::Metric ) -> Result<hyper::client::Response, ClientError> {
        let url = format!( "{}/{}/metrics/{}/{}", self.config.base_url, API_VERSION, self.config.account_key, self.config.agent_key );
        trace!( "{}: posting to {}", &metric.name, &url );

        trace!( "{:?}", &metric );

        let mut headers = Headers::new();
        headers.set( ContentType( mime!(Application/Json; Charset=Utf8) ) );

        // there's no way to make this connection timeout sooner if the server isn't availble
        //   https://stackoverflow.com/questions/30022084/how-do-i-set-connect-timeout-on-tcpstream
        let res = self.hyper.post( &url )
            .headers( headers )
            .body( try!( metric.serialize() ).as_str() )
            .send();

        Ok( try!( res ) )
    }

    fn cache_metric( &self, metric: metric::Metric ) -> Result<(), ClientError> {
        let timespec = &metric.timestamp.to_timespec();
        let filename = format!( "{}/{}-{}.{}.metric", ::CLIENT_CACHE_DIR, &metric.name, &timespec.sec, &timespec.nsec );

        try!( fs::create_dir_all( ::CLIENT_CACHE_DIR ).context( ::CLIENT_CACHE_DIR ) );
        info!( "{} - writing to cache {}", &metric.name, &filename );

        let mut f = try!( fs::File::create( &filename ).context( filename.as_str() ) );
        try!( f.write_all( try!( metric.serialize() ).as_bytes() ).context( filename.as_str() ) );

        Ok( () )
    }
}
