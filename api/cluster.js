'use strict';

const http = require('http');
const fs = require('fs');
const cluster = require('cluster');

if (cluster.isMaster) {
    const workers = process.env.WORKERS || 1;

    // Fork workers.
    for (var i = 0; i < workers; i++) {
        cluster.fork();
    }
    console.log(`starting ${workers} workers...`);

    cluster.on('disconnect', (worker) => {
          console.log(`worker #${worker.id} has disconnected`);
    });
    cluster.on('exit', (worker, code, signal) => {
        console.log(`worker #${worker.id} exited`);
    });

    process.on('SIGTERM', () => {
        console.log('master caught SIGTERM, forwarding to workers');
        for (var id in cluster.workers) {
            cluster.workers[id].send('SIGTERM');
        }
    });

} else {
    var app = require('./index');
    var server = http.createServer(app);
    server.listen(process.env.PORT || 8000);

    process.on('message', (msg) => {
        if(msg === 'SIGTERM') {
            server.close(function () {
                process.exit();
            } );
        }
    });
}
