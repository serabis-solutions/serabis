'use strict';

//longer stack traces in dev
var node_env = process.env.NODE_ENV;
if ( ( node_env === undefined ) || node_env === 'development' ) {
    Error.stackTraceLimit = Infinity;
}

var express = require('express');
var kraken = require('kraken-js');


var options, app;

/*
 * Create and configure application. Also exports application instance for use by tests.
 * See https://github.com/krakenjs/kraken-js#options for additional configuration options.
 */
options = {
    onconfig: function (config, next) {
        /*
         * Add any additional config setup or overrides here. `config` is an initialized
         * `confit` (https://github.com/krakenjs/confit/) configuration object.
         */

        //this is dumb. can it be automated?
        require('./models/mq.js').init( config.get('model/mq') );
        require('./models/database.js').init( config.get('model/database') );

        next(null, config);
    }
};

app = module.exports = express();
app.use(kraken(options));
app.disable('etag');
app.on('start', function () {
    console.log('Application ready to serve requests.');
    console.log('Environment: %s', app.kraken.get('env:env'));
});
