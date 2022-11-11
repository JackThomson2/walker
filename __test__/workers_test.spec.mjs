import test from 'ava'
import axios from 'axios';
import { Worker } from 'node:worker_threads';
import { fileURLToPath } from 'url';
import path from 'path';

const Server = axios.create({
    baseURL: 'http://0.0.0.0:8080/'
  });

import * as Walker from '../index.js'

const config = {
    url: "0.0.0.0:8080",
    workerThreads: 8,
    backlog: 500,
    poolPerWorkerSize: 1_000,
    debug: false,
    tls: false,
}

test.serial.before(async (_) => {
    let dir = path.dirname(fileURLToPath(import.meta.url));
    for (let i = 0; i < 5; i++) {
        const worker = new Worker(`${dir}/worker_file.js`);
    }

    // We'll let the worker threads spin up
    await new Promise((resolve) => setTimeout(resolve, 1000));

    Walker.startWithConfig(config);

    // Sleeep for 100ms to let server start
    await new Promise((resolve) => setTimeout(resolve, 300));
});

// Hit the cpu endpoint 5_000 times to saturate the object pool
test.serial("Get /slowrunner multiple times in waves", async t => {
    const req_number = 100;
    const loop_number = 100_000;

    for (let c = 0; c < 100; c++) {
        const promises = [];
        for (let i = 0; i < req_number; i++) {
            promises.push(Server.get(`/slowrunner/${loop_number}`));
        }

        const responses = await Promise.all(promises);

        t.is(responses.length, req_number);
        // check all the responses are correct
        responses.forEach((resp, index) => {
            t.is(resp.data, `Result ${loop_number}`);
        });
    }
});


// Hit the endpoint with a continious load 
test.serial("Get /slowrunner multiple times with a consistent load", async t => {
    const promises = [];
    const req_number = 1000;
    const loop_number = 100_000;
    let sum_of_requests = 10_000;

    for (let i = 0; i < req_number; i++) {
        const p = Server.get(`/slowrunner/${loop_number}`).then(res => {
            promises.splice(promises.indexOf(p), 1);
            return res;
        });
        promises.push(p);
    }

    while (sum_of_requests > 0) {
        const complete = await Promise.race(promises);
        sum_of_requests -= 1;

        const p = Server.get(`/slowrunner/${loop_number}`).then(res => {
            promises.splice(promises.indexOf(p), 1);
            return res;
        });
        promises.push(p);

        t.is(complete.data, `Result ${loop_number}`);
    }

    const responses = await Promise.all(promises);

    // check all the responses are correct
    responses.forEach((resp, index) => {
        t.is(resp.data, `Result ${loop_number}`);
    });
});
