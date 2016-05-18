'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();
    var uuid = require('node-uuid');

    router.post('/', function(req, res) {
        var details = req.body;
        details['key'] = generateKey(details);

        db.addAccount(details)
            .then(function() {
                res.json({ saved: 1, key: details['key'] } );
            })
            .catch(function(err) {
                console.log(err);
                res.json ({ err: { code: 2001, msg: 'Failed to add new account'}});
            });
    });

    function generateKey(details) {
        return uuid.v4();
    }
};
