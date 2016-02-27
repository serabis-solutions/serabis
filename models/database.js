'use strict';
var config;

class Database {
    constructor() {
        var pgp = require('pg-promise')({});
        this.db = pgp({
            host: 'localhost',
            port: 5432,
            database: 'serapis_dev',
            user: 'serapis',
            password: 'reallysecure',
        });
        
        //'postgres://serapis:reallysecure@localhost:5432/serapis_dev');

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
        var q = this._buildDataPointsQuery(dataPoints, agentId);
        return this.db.none(q.text, q.values );
    }
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new Database;
}


