import test from 'ava'

import registerRoutes from './stess_rig.mjs';

import * as Walker from '../index.js'

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
        promises.push(fetch("http://0.0.0.0:8080/"));
    }

    const responses = await Promise.all(promises);
    const texts = await Promise.all(responses.map((response) => response.text()));

    t.is(texts.length, 1000);
    texts.forEach((text) => {
        t.is(text, "Hello World");
    });
});

// We'll send multiple requests to the server to see if it can handle it
test.serial("Get /cpu returns Hello World 500 times", async t => {
    const promises = [];
    for (let i = 0; i < 500; i++) {
        promises.push(fetch("http://0.0.0.0:8080/cpu"));
    }

    const responses = await Promise.all(promises);
    const texts = await Promise.all(responses.map((response) => response.text()));

    t.is(texts.length, 500);
    // check all the responses are correct
    texts.forEach((text) => {
        t.is(text, "Hello World");
    });
});

// Make 1000 post requests with incrementing body index and check the response
test("Post /return_text_body returns Hello World 10000 times", async t => {
    const promises = [];
    for (let i = 0; i < 1000; i++) {
        promises.push(fetch("http://0.0.0.0:8080/return_text_body", { method: 'POST', body: i.toString() }));
    }

    const responses = await Promise.all(promises);
    const texts = await Promise.all(responses.map((response) => response.text()));

    t.is(texts.length, 1000);
    // check all the responses are correct
    texts.forEach((text, index) => {
        t.is(text, index.toString());
    });
});