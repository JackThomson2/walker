const Walker = require('..');

const config = {
    url: "0.0.0.0:8081",
    worker_threads: "1",
    pool_per_worker_size: "100",
    tls: "true",
    cert_location: './certs/cert.pem',
    key_location: './certs/key.pem'
}

Walker.get("/", (res) => {
    res.sendTextUnchecked("Hello world!");
});

Walker.startWithConfig(config);
