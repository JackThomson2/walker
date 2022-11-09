const {
    Worker,
    isMainThread,
    setEnvironmentData,
    getEnvironmentData,
} = require('node:worker_threads');

const Walker = require('..');
const response = "Hello World!"

if (isMainThread) {
    setEnvironmentData('Hello', 'World!');
    const result = Walker.getWorkerId();

    Walker.initialisePoolForWorker(200_000);
    console.log(`Result is ${result}`);

    for (let i = 0; i < 10; i++) {
        const worker = new Worker(__filename);
    }

    Walker.get(`/${result}`, (res) => {
        res.sendText(`Hello from main thread our id is ${result}`);
    })

    Walker.get(`/`, (res) => {
        res.sendTextUnchecked(response);
    })

    setTimeout(() => {
        console.log('Starting server...')
        const config = {
            url: "0.0.0.0:8081",
            workerThreads: 12,
            poolPerWorkerSize: 10000,
            backlog: 10000,
            debug: false,
            tls: false,
        }
        Walker.startWithConfig(config);
    }, 5000);
} else {
    let result = Walker.getWorkerId();
    console.log(`Result is ${result}`);

    Walker.get(`/key`, (res) => {
        res.sendText(`Hello from worker thread our id is ${result}`);
    })

    Walker.get(`/`, (res) => {
        res.sendTextUnchecked(response);
    })

    Walker.initialisePoolForWorker(200_000);
}
