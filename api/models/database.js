'use strict';
var config;

class Database {
    constructor() {
        var pgp = require('pg-promise')({});
        this._value = require('pg-promise/lib/formatting').as.value;
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

    saveDataPoints(dataPoints, accountKey, agentKey) {
        var model = this;

        return this.db.one(
            'SELECT * FROM agents WHERE key = $1 AND account_key = $2',
            [ agentKey, accountKey ])
        .then(function(data) {
            var q = model._buildDataPointsQuery(dataPoints, agentKey);
            return model.db.none(q.text, q.values );
        })
        .catch(function(err) {
            return {error: "Unable to validate agent_key and/or account_key"};
        });
    }

    addAgent(details) {
        return this.db.none(
            'INSERT INTO agents (key, hostname, shortname, account_key) VALUES ($1, $2, $3, $4)',
            [
                details['key'],
                details['hostname'],
                details['shortname'],
                details['account_key']
            ]);
    }

    addAccount(details) {
        return this.db.none(
            'INSERT INTO accounts (key, name, shortname) VALUES ($1, $2, $3)',
            [
                details['key'],
                details['name'],
                details['shortname']
            ]);
    }

    /* I'm fairly sure this is the ugliest code I've ever written in my */
    /* life. Basically the split('').reverse().join('') bullshit is to  */
    /* reverse a string. This is needed because JS doesn't support      */
    /* look-behind regex, which we need to allow escaping of colons in  */
    /* key names. Pg-promise's format is called on everything we don't  */
    /* manually add, so it *should* be safe.                            */
    _parseDataKey(dataKey) {
        var keys = dataKey.split('').reverse().join('').split(/:(?!\\)/).reverse();
        var value = this._value;
        var formatted = [];

        keys.forEach(function(key) {
            key = key.split('').reverse().join('');
            key = key.replace('\\:', ':');
            formatted.push("'".concat(value(key).concat("'")));
        });

        var cleanKey = "->".concat(formatted.join("->"))
            .replace(new RegExp('(.*)->'), '$1->>');

        return cleanKey;
    }

    getAggregateDataPoints(agentKey, dataKey, type, start, end) {
        start = Number(start);
        end = Number(end);

        if(isNaN(start)) {
            start = 0;
        }
        if(isNaN(end)) {
            end = Math.round(Date.now() / 1000);
        }

        dataKey = this._parseDataKey(dataKey);

        var periodLength = 60; //1 minute
        if(end - start > 60*60*24*14) { //More than 2 weeks
            periodLength = 60*60*24; // 1day
        } else if (end - start > 60*60*24*48) { //More than 2 days
            periodLength = 60*60; //1 hour
        } else if (end - start > 60*60*12) { // More than 12 hours
            periodLength = 60*5; //5 minutes
        }

        return this.db.manyOrNone(
            'SELECT avg((data->\'data\'$5:raw)::numeric) AS value, (((data->>\'timestamp\')::int)/$6)*$6 ts, ((data->>\'timestamp\')::int)/$6 g FROM data_points WHERE agent_key = $1 AND data->>\'type\' = \'$2#\' AND (data->>\'timestamp\')::integer BETWEEN $3 AND $4 GROUP BY 3 ORDER BY g LIMIT 1000',
            [
                agentKey,
                type,
                start,
                end,
                dataKey,
                periodLength
            ]);
    }

    getDataPoints(agentKey, type, start, end) {
        start = Number(start);
        end = Number(end);

        if(isNaN(start)) {
            start = 0;
        }
        if(isNaN(end)) {
            end = Math.round(Date.now() / 1000);
        }

        return this.db.manyOrNone(
            'SELECT data FROM data_points WHERE agent_key = $1 AND data->>\'type\' = $2 AND (data->>\'timestamp\')::integer BETWEEN $3 AND $4 ORDER BY data->>\'timestamp\' LIMIT 1000',
            [
                agentKey,
                type,
                start,
                end
            ]);
    }

    //Add a new condition
    addCondition(accountKey, name, shortname) {
        return this.db.one(
            'INSERT INTO conditions (account_id, name, shortname) VALUES ((SELECT id FROM accounts WHERE key = $1), $2, $3) RETURNING id',
            [accountKey, name, shortname]
        );
    }


    addConditionComponent(condition_id, type, opperator, value, trigger_key) {
        return this.db.one(
            'INSERT INTO condition_components (condition_id, type, opperator, trigger_value, trigger_key) VALUES ($1, $2, $3, $4, $5) RETURNING id',
            [condition_id, type, opperator, value, trigger_key]
        );
    }
}

module.exports.init = function(modelConfig) {
    config = modelConfig;
};

module.exports.new = function() {
    return new Database;
}


