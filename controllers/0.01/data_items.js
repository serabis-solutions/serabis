'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();

    router.post('/:accountKey/:agentKey', function(req, res) {
        var items = req.body;

        if(!Array.isArray(items)) {
            items = [items];
        }

        db.saveDataPoints(items, req.params.accountKey, req.params.agentKey)
            .then(function(result) {
                if(result != undefined && result['error']) {
                    res.json ({ err: { code: 1003, msg: result['error'] }});
                } else {
                    res.json({ dataPointsSaved: items.length } );
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
            req.params.type,
            req.body.start,
            req.body.end
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
