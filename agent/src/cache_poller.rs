use std::thread;
use std::time::Duration;
use std::sync::Arc;
use quick_error::ResultExt;
use client;
use plugin::PluginError;
use std::fs;
use metric;

pub struct CachePoller {
    client                : Arc<client::Client>,
}

impl CachePoller {
    pub fn new( client: Arc<client::Client> ) -> CachePoller {
        CachePoller {
            client                  : client,
        }
    }

    //XXX stop this using pluginerror
    pub fn run( self ) -> Result<(), PluginError> {
        loop {
            thread::sleep( Duration::from_secs( 60 * 10 ) );

            if let Err(err) = self.send_metrics() {
                info!( "posting cached metric failed, trying again later [{}]", err );
                continue;
            };
        }
    }

    fn send_metrics( &self ) -> Result<(), PluginError> {
        info!("looking for cached metrics");

        let files = try!( fs::read_dir( ::CLIENT_CACHE_DIR ).context( ::CLIENT_CACHE_DIR ) );

        for file in files {
            let cached_metric_path = try!( file ).path();

            let correct_filetype : bool = {
                match cached_metric_path.extension() {
                    Some(v) => v == "metric",
                    None    => false,
                }
            };

            if !cached_metric_path.is_file() || !correct_filetype {
                trace!("ignoring {:?}", &cached_metric_path );
                continue;
            }

            info!( "found cached metric {:?}", &cached_metric_path );

            let metric = match metric::Metric::new_from_file( &cached_metric_path ) {
                Ok(v)   => v,
                Err(e)  => {
                    error!( "failed to read cached metric [{}] - {}", e, &cached_metric_path.display() );
                    continue;
                }
            };

            match self.client._post_metric( &metric ) {
                Err(e)  => return Err( PluginError::ClientError(e) ),
                Ok(_)   => {
                    info!("successfully posted cached metric. deleting");
                    try!( fs::remove_file( &cached_metric_path ) );
                }
            }
        }

        Ok( () )
    }
}
