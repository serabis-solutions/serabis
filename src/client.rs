use hyper;
use std::sync::Arc;
use std::time::Duration;

const API_VERSION: &'static str = "0.01";

pub struct Client {
    config  : Arc<config::Monitor>,
    hyper   : Arc<hyper::Client>,
}
use config;

impl Client {
    pub fn new( config: Arc<config::Monitor> ) -> Client {
        let mut client = hyper::Client::new();
        client.set_read_timeout( Some( Duration::from_secs( 10 ) ) );
        client.set_write_timeout( Some( Duration::from_secs( 10 ) ) );

        Client {
            config  : config,
            hyper   : Arc::new(client),
        }
    }

    //XXX better name
    pub fn report( &self, name: &str, data: &str ) {
        let url = format!( "{}/{}/data_items/{}/{}", self.config.base_url, API_VERSION, self.config.account_key, self.config.agent_key );
        trace!( "{}: posting to {}", &name, &url );

        let report = format!( r#"{{ "name": "{}", "data": {} }}"#, name, data );
        debug!( "{}", report );

        // there's no way to make this connection timeout sooner if the server isn't availble
        //   https://stackoverflow.com/questions/30022084/how-do-i-set-connect-timeout-on-tcpstream
        let res = self.hyper.post( &url )
            .body( &report )
            .send()
            .unwrap();
        debug!( "{}: report {}", &name, res.status );
    }
}
