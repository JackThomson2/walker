const {
    Worker,
    isMainThread,
    setEnvironmentData,
    getEnvironmentData,
} = require('node:worker_threads');

const Walker = require('..');

if (isMainThread) {
    setEnvironmentData('Hello', 'World!');
    const result = Walker.getWorkerId();

    Walker.initialisePoolForWorker(10000);
    console.log(`Result is ${result}`);
    const worker = new Worker(__filename);

    Walker.get(`/${result}`, (res) => {
        res.sendText(`Hello from main thread our id is ${result}`);
    })

} else {
    let result = Walker.getWorkerId();
    console.log(`Result is ${result}`);

    result = Walker.getWorkerId();
    console.log(`Result is ${result}`);

    result = Walker.getWorkerId();
    console.log(`Result is ${result}`);

    result = Walker.getWorkerId();
    console.log(`Result is ${result}`);


    Walker.get(`/${result}`, (res) => {
        res.sendText(`Hello from worker thread our id is ${result}`);
    })

    Walker.get(`/b`, (res) => {
        res.sendText(`Hello from worker thread our id is ${result}`);
    })

    Walker.get(`/c`, (res) => {
        res.sendText(`Hello from worker thread our id is ${result}`);
    })

    Walker.get(`/d`, (res) => {
        res.sendText(`Hello from worker thread our id is ${result}`);
    })

    const config = {
        url: "0.0.0.0:8081",
        workerThreads: 1,
        poolPerWorkerSize: 100,
        backlog: 1000,
        debug: true,
        tls: false,
    }
    Walker.initialisePoolForWorker(10000);
    Walker.startWithConfig(config);
}
