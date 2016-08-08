'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();
    var uuid = require('node-uuid');

    router.post('/', function(req, res) {
        var details = req.body;
        details['key'] = generateKey(details);

        db.addAgent(details)
            .then(function() {
                res.json({ saved: 1, key: details['key'] } );
            })
            .catch(function(err) {
                console.log(err);
                res.json ({ err: { code: 2001, msg: 'Failed to add new agent'}});
            });
    });

    function generateKey(details) {
        return uuid.v4();
    }

    router.get('/:accountKey', function(req, res) {
        db.getAgentList(
            req.params.accountKey
        )
        .then(function(data) {
            res.json(data);
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 2003, msg: 'Failed to load agent list'}});
        });
    });
};
