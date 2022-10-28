import test from 'ava'
import axios from 'axios';

import registerRoutes from './stess_rig.mjs';

import * as Walker from '../index.js'

const Server = axios.create({
    baseURL: 'http://0.0.0.0:8080/'
  });

test.serial.before(async (_) => {
    // This runs before all tests
    registerRoutes();

    Walker.startWithWorkerCount("0.0.0.0:8080", 4);

    // Sleeep for 100ms to let server start
    await new Promise((resolve) => setTimeout(resolve, 300));
});

// Send 1000 requests to the root of the server
test.serial("Get / returns Hello World", async t => {
    const promises = [];
    for (let i = 0; i < 1000; i++) {
        promises.push(Server.get("/"));
    }

    const responses = await Promise.all(promises);

    t.is(responses.length, 1000);
    responses.forEach((resp) => {
        t.is(resp.data, "Hello World");
    });
});

// We'll send multiple requests to the server to see if it can handle it
test.serial("Get /cpu returns Hello World 500 times", async t => {
    const promises = [];
    for (let i = 0; i < 500; i++) {
        promises.push(Server.get("/cpu"));
    }

    const responses = await Promise.all(promises);

    t.is(responses.length, 500);
    // check all the responses are correct
    responses.forEach((resp) => {
        t.is(resp.data, "Hello World");
    });
});

// Make 1000 post requests with incrementing body index and check the response
test("Post /return_text_body returns Hello World 10000 times", async t => {
    const promises = [];
    for (let i = 0; i < 1000; i++) {
        promises.push(Server.post("/return_text_body", i.toString()));
    }

    const responses = await Promise.all(promises);

    t.is(responses.length, 1000);
    // check all the responses are correct
    responses.forEach((resp, index) => {
        t.is(resp.data, index);
    });
});