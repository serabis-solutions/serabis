'use strict';
var decode = require('urldecode');

var dbModel = require('../../models/database');
var mqModel = require('../../models/mq');

module.exports = function (router) {
    var db = dbModel.new();
    var mq = mqModel.new();

    router.post('/:accountKey/:agentKey', function(req, res) {
        var items = req.body;
        if(!Array.isArray(items)) {
            items = [items];
        }

        db.saveDataPoints(items, req.params.accountKey, req.params.agentKey)
            .then(function(result) {
                if(result != undefined && result['error']) {
                    console.log(result['error']);
                    res.json ({ err: { code: 1003, msg: result['error'] }});
                } else {
                    res.json({ dataPointsSaved: items.length } );
                    items.forEach(function (item) {
                        item['agent'] = req.params.agentKey;
                        item['account'] = req.params.accountKey;
                        mq.publish(item, 'metric.new');
                    });
                }
            })
            .catch(function(err) {
                console.log(err);
                res.json ({ err: { code: 1001, msg: 'Failed to save data points'}});
            });
    });

    router.get('/:agentKey/:type', function(req, res) {
        db.getDataPoints(
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
            res.json({err: { code: 1002, msg: 'Failed to load data points'}});
        });
    });

    router.get('/load/:agentKey/:type/:dataKey', function(req, res) {
        db.getAggregateDataPoints(
            req.params.agentKey,
            decode(req.params.dataKey),
            decode(req.params.type),
            req.query.start,
            req.query.end
        )
        .then(function(data) {
            res.json({data: data});
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 1002, msg: 'Failed to load data points'}});
        });
    });
};
