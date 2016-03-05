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

    addAgent(details) {
        return this.db.none(
            'INSERT INTO agents (key, hostname, shortname) VALUES ($1, $2, $3)',
            [
                details['key'],
                details['hostname'],
                details['shortname']
            ]);
    }

    getDataPoints(agentKey, type, start, end) {
        if(start === undefined) {
            start = '0';
        }
        if(end === undefined) {
            end = '253402214400'; //31/12/9999 #Bigger than 32 bit timestamp
        }
 
        return this.db.manyOrNone(
            'SELECT data FROM data_points WHERE agent_key = $1 AND data->>\'type\' = $2 AND data->>\'timestamp\' BETWEEN $3 AND $4 ORDER BY data->>\'timestamp\' LIMIT 1000',
            [
                agentKey,
                type,
                start,
                end
            ]);
    }
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new Database;
}


