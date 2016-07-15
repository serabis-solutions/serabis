'use strict';
var config;

class MQ{
    constructor() {
        this.mq = require('amqplib')
            .connect('amqp://inserter:reallysecure@dev.serapis/serapis');
        console.log('Connecting to RabbitMQ');

        var mq = this;
        
        this.mq.then(function(conn) {
          return conn.createChannel();
        }).then(function(ch) {
          ch.assertExchange("exchange", "topic", {durable: true}).then(function(ok) {
              mq.channel = ch;
          });
        });
    };



    publish(data, key) {
      return this.channel.publish("exchange", key, new Buffer(JSON.stringify(data)));
    };
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new MQ;
}


