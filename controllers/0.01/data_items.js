'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();

    router.post('/:agentId', function(req, res) {
        var items = req.body;

        if(!Array.isArray(items)) {
            items = [items];
        }

        db.saveDataPoints(items, req.params.agentId)
            .then(function() {
                res.json({ dataPointsSaved: items.length } );
            })
            .catch(function(err) {
                console.log(err);
                res.json ({ err: { code: 1001, msg: 'Failed to save data points'}});
            });
    });
};
