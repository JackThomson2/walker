import test from 'ava'
import fetch from 'node-fetch';

import * as Walker from '../index.js'

const config = {
    url: "0.0.0.0:8080",
    worker_threads: "1",
    backlog: "1000000",
    pool_per_worker_size: "1"
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


// Hit the cpu endpoint 10_000 times to saturate the object pool
test("Get /cpu 10_000 times", async t => {
    const promises = [];
    for (let i = 0; i < 10_000; i++) {
        promises.push(fetch(`http://0.0.0.0:8080/cpu/${i}`));
    }

    const responses = await Promise.all(promises);
    const texts = await Promise.all(responses.map((response) => response.text()));

    t.is(texts.length, 10_000);
    // check all the responses are correct
    texts.forEach((text, index) => {
        t.is(text, `Param: ${index}`);
    });
});