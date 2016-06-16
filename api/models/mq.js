'use strict';
var config;

class MQ{
    constructor() {
        this.mq = require('amqplib')
            .connect('amqp://inserter:reallysecure@dev.serapis/serapis');

        if( true || config.debug ) {
            console.log('Connecting to RabbitMQ');
        }
    };



    publish(data, key) {

      // do you really want to open and close a channel every time?
      // I was going to make it open at connect and stash it, but it
      // doesn't seem very nodejs
        this.mq.then(function(conn) {
            return conn.createChannel();
        }).then(function(ch) {
          // so it seems "config" is undefined?
          var exchange_name = "exchange"; //such a bad name. meh

          //same with this. there's no need to do it everytime do it once at startup
            return ch.assertExchange(exchange_name, 'topic', {durable: true})
                .then(function(ok) {
                    return ch.publish(exchange_name, key, new Buffer(JSON.stringify(data)));
                } );
        });
    };
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new MQ;
}


