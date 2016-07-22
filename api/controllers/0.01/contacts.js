'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();

    router.post('/new/:accountKey', function(req, res) {
        var details = req.body;

        db.addContact(req.params.accountKey, details['fname'], details['lname'], details['email'])
            .then(function() {
                res.json({ saved: 1 } );
            })
            .catch(function(err) {
                console.log(err);
                res.json ({ err: { code: 4001, msg: 'Failed to add new contact'}});
            });
    });
};
