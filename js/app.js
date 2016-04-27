(function() {
    var app = angular.module('serapis', ['chart.js']);

    app.directive('agentHeader', function() {
        return {
            restrict: 'E',
            templateUrl: 'ng-components/agents/header.html',
            controller: function (){
                this.host = agent;
            },
            controllerAs: 'agent'
        };
    });

    app.directive('graph', function() {
        return {
            restrict: 'E',
            templateUrl: 'ng-components/graphs/default.html',
            controller: function ($scope, $http){
                var start = Date.now() - (24 * 60 * 1000 * 60);
                start = Math.round(start/1000); //We don't need milisecond accuracy!
                $scope.data = [[], [], []];
                $scope.labels = [];
                $scope.series = [];
                update_graph_data(agent, 'load', '1min', $scope, $http, start, 0);
                update_graph_data(agent, 'load', '5min', $scope, $http, start, 1);
                update_graph_data(agent, 'load', '15min', $scope, $http, start, 2);

                this.host = agent;
            },
            controllerAs: 'agent'
        };
    });

    function update_graph_data(agent, type, key, scope, http, start, series_index) {
        http.get('http://dev.serapis:8000/0.01/data_items/load/' + agent.key + '/' + type + '/' + key + '?start=' + start)
            .then(function(res) {
                console.log('MASSIVE SUCCESS2');
                var data = res.data.data;
                data.forEach(function(obj) {
                    scope.data[series_index].push(obj.value);
                    if(series_index == 0) {
                        scope.labels.push(new Date(obj.ts * 1000));
                    }
                });
                scope.series.push(key);

            })
            .catch(function(err) {
                console.log('MASSIVE ERROR2');
                console.log(err);
            });
    }

    var agent = {
        shortName: 'test',
        name: 'test.lwtn.org',
        key: '1497e439-87f3-4784-b949-e7c9487d888b'
    };

})();

