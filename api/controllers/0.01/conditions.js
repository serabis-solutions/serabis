'use strict';

var dbModel = require('../../models/database');

module.exports = function (router) {
    var db = dbModel.new();

    router.post('/new/:accountKey', function(req, res) {
        var condition = req.body;
        db.addCondition(req.params.accountKey, condition.name, condition.shortname)
        .then(function(data) {
            condition.condition.forEach(function(component) {
                db.addConditionComponent(data.id, component.type, Object.keys(component.value)[0], component.value[Object.keys(component.value)[0]], component.key)
                .then(function(data) {
                    console.log(data);
                })
                .catch(function(err) {
                    console.log(err);
                });
            });
            res.json({condition: data});
        })
        .catch(function(err) {
            console.log(err);
            res.json({err: { code: 4001, msg: 'Failed to add condition'}});
        });
    });

    router.post('/add_to_agent/:agent_id', function(req, res) {
      var data = req.body;
      db.addAgentCondition(req.params.agent_id, data.condition)
        .then(function(data) {
          console.log(data);
          res.json({success: 1});
        })
        .catch(function(err) {
          console.log(err);
          res.json({err: {code: 4002, msg: 'Failed to add condition to agent'}});
        });
    });

};
