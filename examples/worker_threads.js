const {
    Worker,
    isMainThread,
    setEnvironmentData,
    getEnvironmentData,
} = require('node:worker_threads');

const Walker = require('..');
const response = "Hello World!"

if (isMainThread) {

    for (let i = 0; i < 6; i++) {
       // const worker = new Worker(__filename);
    }

    Walker.get(`/key`, (res) => {
        let i = 0; 

        for (i; i < 1_000_000; i++) {
            
        }

        res.sendTextUnchecked(`Result ${i}`)
    })
    

    setTimeout(() => {
        console.log('Starting server...')
        const config = {
            url: "0.0.0.0:8081",
            workerThreads: 6,
            poolPerWorkerSize: 200_000,
            backlog: 10000,
            debug: false,
            tls: false,
        }
        Walker.startWithConfig(config);
    }, 5000);
} else {
    Walker.get(`/`, (res) => {
        res.sendTextUnchecked(response);
    })

    Walker.get(`/key`, (res) => {
        let i = 0; 

        for (; i < 1_000_000; i++) {
            
        }

        res.sendTextUnchecked(`Result ${i}`)
    })
    Walker.registerThreadsPool(200_000);
}
