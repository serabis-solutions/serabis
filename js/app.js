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
                $http.get('http://dev.serapis:8000/0.01/data_items/3fce4f1d-b74a-40fa-934f-9dd6c2717e6f/system.load')
                    .then(function(res) {
                        console.log('MASSIVE SUCCESS!');
                        console.log(res.status);
                        console.log(res.data.data);
                        var data = res.data.data;
                        var labels = [];
                        var graph_data = [[], [], []];
                        data.forEach(function(obj) {
                            labels.push(new Date(obj.data.timestamp * 1000));
                            graph_data[0].push(obj.data['1min']);
                            graph_data[1].push(obj.data['5min']);
                            graph_data[2].push(obj.data['15min']);
                        });
console.log(labels);
console.log(graph_data);
                        $scope.labels = labels;
  $scope.series = ['1min', '5min', '15min'];
  $scope.data = graph_data; 
  $scope.onClick = function (points, evt) {
    console.log(points, evt);
  };


                    })
                    .catch(function(err) {
                        console.log('MASSIVE ERROR');
                        console.log(err);
                    });
                this.host = agent;
            },
            controllerAs: 'agent'
        };
    });

    var agent = {
        shortName: 'test',
        name: 'test.lwtn.org',
        key: '3fce4f1d-b74a-40fa-934f-9dd6c2717e6f'
    };

})();

