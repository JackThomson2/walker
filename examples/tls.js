const Walker = require('..');

const config = {
    url: "0.0.0.0:8081",
    workerThreads: 1,
    poolPerWorkerSize: 100,
    backlog: 10000,
    debug: false,
    tls: true,
    certLocation: './certs/cert.pem',
    keyLocation: './certs/key.pem'
}

Walker.get("/", (res) => {
    res.sendTextUnchecked("Hello world!");
});

Walker.get("/test", (res) => {
    res.sendText("Hello world!");
});

Walker.startWithConfig(config);
