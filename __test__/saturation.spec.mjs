import test from 'ava'
import axios from 'axios';

const Server = axios.create({
    baseURL: 'http://0.0.0.0:8080/'
  });

import * as Walker from '../index.js'

const config = {
    url: "0.0.0.0:8080",
    workerThreads: 1,
    backlog: 1000000,
    poolPerWorkerSize: 1,
    debug: false,
    tls: false,
}

test.serial.before(async (_) => {
    Walker.get("/cpu/:id", (res) => {
        let i = 0;
        while (i < 10_000_000) {
            i++;
        }
        const params = res.getUrlParams();

        res.sendText(`Param: ${params.id}`);
    });

    Walker.startWithConfig(config);

    // Sleeep for 100ms to let server start
    await new Promise((resolve) => setTimeout(resolve, 300));
});


// Hit the cpu endpoint 5_000 times to saturate the object pool
test("Get /cpu 5_000 times", async t => {
    const promises = [];
    for (let i = 0; i < 5_000; i++) {
        promises.push(Server.get(`/cpu/${i}`));
    }

    const responses = await Promise.all(promises);

    t.is(responses.length, 5_000);
    // check all the responses are correct
    responses.forEach((resp, index) => {
        t.is(resp.data, `Param: ${index}`);
    });
});
