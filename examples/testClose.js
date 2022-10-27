const Walker = require('..');

const config = {
    url: "0.0.0.0:8081",
    worker_threads: "1",
    pool_per_worker_size: "100"
}

Walker.get("/", (res) => {
    res.sendText("Hello world");
});

Walker.startWithConfig(config);

setTimeout(() => {
    console.log("STOPPING THE SERVER!!");

    Walker.stop();
    console.log("JS thinks server is stopped");

    setTimeout(() => {
        console.log('Done waiting')
    }, 100000);
}, 10000)
