(function() {
    var app = angular.module('serapis', ['chart.js']); //.config(function($locationProvider) { $locationProvider.html5Mode(true); }); 

    app.directive('agentHeader', function() {
        return {
            restrict: 'E',
            templateUrl: 'ng-components/agents/header.html',
            controller: function (){
                this.host = agent;
            },
            controllerAs: 'agent',
            scope: {},
        };
    });

    app.directive('graphDiskSpace', function() {
        return {
            restrict: 'E',
            templateUrl: 'ng-components/graphs/default.html',
            controller: function ($scope, $http, $location){
                var start = Date.now() - (60 * 1000 * 60);
                start = Math.round(start/1000); //We don't need milisecond accuracy!
                agent['key'] = $location.search()['agent'];
                $scope.data = [[], [], [], []];
                $scope.labels = [];
                $scope.series = [];
                update_graph_data(agent, 'disk_space', '%2Fdev%2Fsda1:size', $scope, $http, start, 0);
                update_graph_data(agent, 'disk_space', '%2Fdev%2Fsda1:used', $scope, $http, start, 1);
                update_graph_data(agent, 'disk_space', '%2Fdev%2Fsda1:available', $scope, $http, start, 2);

                this.host = agent;
                this.type = 'DiskStats';
            },
            scope: {},
            controllerAs: 'agent'
        };
    });

    app.directive('graphNetstats', function() {
        return {
            restrict: 'E',
            templateUrl: 'ng-components/graphs/default.html',
            controller: function ($scope, $http, $location){
                var start = Date.now() - (60 * 1000 * 60);
                start = Math.round(start/1000); //We don't need milisecond accuracy!
                $scope.data = [[], [], [], []];
                agent['key'] = $location.search()['agent'];
                $scope.labels = [];
                $scope.series = [];
                update_graph_data(agent, 'netstats', 'eth0:rx', $scope, $http, start, 0);
                update_graph_data(agent, 'netstats', 'eth0:tx', $scope, $http, start, 1);
                update_graph_data(agent, 'netstats', 'eth1:rx', $scope, $http, start, 2);
                update_graph_data(agent, 'netstats', 'eth1:tx', $scope, $http, start, 3);

                this.host = agent;
                this.type = 'Netstats';
            },
            scope: {},
            controllerAs: 'agent'
        };
    });

   app.directive('graph', function() {
        return {
            restrict: 'E',
            templateUrl: 'ng-components/graphs/default.html',
            controller: function ($scope, $http, $location){
                var start = Date.now() - (60 * 1000 * 60);
                agent['key'] = $location.search()['agent'];
                start = Math.round(start/1000); //We don't need milisecond accuracy!
                $scope.data = [[], [], []];
                $scope.labels = [];
                $scope.series = [];
                update_graph_data(agent, 'load', '1min', $scope, $http, start, 0);
                update_graph_data(agent, 'load', '5min', $scope, $http, start, 1);
                update_graph_data(agent, 'load', '15min', $scope, $http, start, 2);

                this.host = agent;
                this.type = 'Load';
            },
            scope: {},
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
                        var date = new Date(obj.ts * 1000);
                        var hours = date.getHours();
                        var minutes = "0" + date.getMinutes();
                        var seconds = "0" + date.getSeconds();

                        var formattedTime = hours + ':' + minutes.substr(-2) + ':' + seconds.substr(-2);
                        scope.labels.push(formattedTime);
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
        key: ''
    };
})();

