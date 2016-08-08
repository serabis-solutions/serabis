'use strict';
var decode = require('urldecode');

var dbModel = require('../../models/database');
var mqModel = require('../../models/mq');

module.exports = function (router) {
    var db = dbModel.new();
    var mq = mqModel.new();

    //new metric
    router.post('/:accountKey/:agentKey', function(req, res) {
        var items = req.body;
        if(!Array.isArray(items)) {
            items = [items];
        }

        db.saveMetrics(items, req.params.accountKey, req.params.agentKey)
            .then(function(result) {
                if(result != undefined && result['error']) {
                    console.log(result['error']);
                    res.json ({ err: { code: 1003, msg: result['error'] }});
                } else {
                    res.json({ metricsSaved: items.length } );
                    items.forEach(function (item) {
                        item['agent'] = req.params.agentKey;
                        item['account'] = req.params.accountKey;
                        mq.publish(item, 'metric.new');
                    });
                }
            })
            .catch(function(err) {
                console.log(err);
                res.json ({ err: { code: 1001, msg: 'Failed to save metrics'}});
            });
    });

    //gets sample of keys from metric.type
    router.get('/sample/:agentKey/:type', function(req, res) {
        db.getMetricsLimited(
            req.params.agentKey,
            req.params.type,
            20
        )
        .then(function(data) {
            res.json(data);
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 1004, msg: 'Failed to load metrics'}});
        });
    });

    //has to be above the more generic match below
    //gets list of metric types for agent
    router.get('/types/:agentKey', function(req, res) {
        db.getMetricTypes(
            req.params.agentKey
        )
        .then(function(data) {
            res.json(data);
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 1003, msg: 'Failed to load metric types'}});
        });
    });

    //gets all metrics of :type for :agent
    router.get('/:agentKey/:type', function(req, res) {
        db.getMetrics(
            req.params.agentKey, 
            decode(req.params.type),
            req.query.start,
            req.query.end
        )
        .then(function(data) {
            res.json({data: data});
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 1002, msg: 'Failed to load metrics'}});
        });
    });

    //gets metrics of :type and [?key]
    router.get('/load/:agentKey/:type', function(req, res) {
        let key = req.query.key;

        if ( !(key instanceof Array) ) {
            key = [key]
        }

        let keys = [];
        key.forEach( (key) => {
            keys.push( decode(key) );
        } );

        db.getAggregateMetrics(
            req.params.agentKey,
            keys,
            decode(req.params.type),
            req.query.start,
            req.query.end
        )
        .then(function(data) {
            res.json(data);
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 1002, msg: 'Failed to load metrics'}});
        });
    });
};
