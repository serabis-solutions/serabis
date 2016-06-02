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


    publish(data, queue) {
        this.mq.then(function(conn) {
            return conn.createChannel();
        }).then(function(ch) {
            return ch.assertQueue(queue)
                .then(function(ok) {
                    return ch.sendToQueue(
                        queue, 
                        new Buffer(JSON.stringify(data)
                    ));
                });
        });
    };
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new MQ;
}


