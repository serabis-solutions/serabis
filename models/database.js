'use strict';
var config;

class Database {
    constructor() {
        this.pg = require('pg').native;
        this.db = new this.pg.Client('postgres://serapis:reallysecure@localhost:5432/serapis_dev');
        this.db.connect();

        if( true || config.debug ) {
            console.log('Connecting to Database');
        }
    }


    _buildDataPointsQuery(dataPoints, agentId) {
        var params = []
        var chunks = []
        for(var i = 0; i < dataPoints.length; i++) {
            var dataPoint = dataPoints[i]
            var valueClause = []
            params.push(dataPoint)
            valueClause.push('$' + params.length)
            params.push(agentId)
            valueClause.push('$' + params.length)
            chunks.push('(' + valueClause.join(', ') + ')')
         }

        return {
            text: 'INSERT INTO data_points (data, agent_key) VALUES ' + chunks.join(', '),
            values: params
        } 
    }

    saveDataPoints(dataPoints, agentId) {
        var q = this.db.query(this._buildDataPointsQuery(dataPoints, agentId) );
        return { dataPointsSaved: dataPoints.length };
    }
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new Database;
}


