'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();

    router.post('/:agentId', function(req, res) {
        var items = req.body;

        if(!Array.isArray(items)) {
            items = [items];
        }

        res.json(db.saveDataPoints(items, req.params.agentId));
    });
};
