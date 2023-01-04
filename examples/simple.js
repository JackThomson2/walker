const Walker = require('..');

const response = "Hello world!";

Walker.get("/", (res) => {
    res.sendTextUnchecked(response);
});


Walker.get("/blank", (res) => {
    res.uncheckedSendEmptyText();
});

const config = {
    url: "0.0.0.0:8081",
    workerThreads: 8,
    poolPerWorkerSize: 200_000,
    backlog: 10000,
    debug: false,
    tls: false,
}

Walker.startWithConfig(config)
