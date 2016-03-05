(function() {
    var app = angular.module('serapis', []);

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
                        console.log(res.data);
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

