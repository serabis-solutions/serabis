'use strict';
var config;

class Database {
    constructor() {
        this.pg = require('pg');
        this.db = new this.pg.Client('postgres://serapis:reallysecure@localhost:5432/serapis_dev');
        this.db.connect();

        if( true || config.debug ) {
            console.log('Connecting to Database');
        }
    }

    saveDataPoints(dataPoints, agentId) {
        var i;
        for (i = 0; i < dataPoints.length; i++) {
            var q = this.db.query(
                "INSERT INTO data_points (data, agent_id) VALUES ($1, $2)", 
                [dataPoints[i], agentId]
            );
        }

        return { dataPointsSaved: i };
    }
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new Database;
}


