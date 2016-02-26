'use strict';

var dbModel = require('../models/database');
module.exports = function (router) {
    var db = dbModel.new();

    router.get('/', function (req, res) {
        res.json({
            title: "Serapis API",
            latest_version: "0.01"
        });
    });

    router.post('/test', function(req, res) {
        res.json(db.saveDataPoint(req.body, 'someAgent'));
    });
};
