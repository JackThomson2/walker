import test from 'ava'
import fetch from 'node-fetch';

import registerRoutes from './standard_rig.mjs';

import * as Walker from '../index.js'


test.before(async (_) => {
	// This runs before all tests
  registerRoutes();
});

test.serial.before(async (_) => {
  Walker.start("0.0.0.0:8080", 1);

  // Sleeep for 100ms to let server start
  await new Promise((resolve) => setTimeout(resolve, 300));
});

test("Get / returns Hello World", async t => {
    const response = await fetch("http://0.0.0.0:8080/");
    const text = await response.text();
    t.is(text, "Hello World");
});

test("Get /async returns Hello World", async t => {
    const response = await fetch("http://0.0.0.0:8080/async");
    const text = await response.text();
    t.is(text, "Hello World");
});

test("Get /sleep returns Hello World", async t => {
    const response = await fetch("http://0.0.0.0:8080/sleep");
    const text = await response.text();
    t.is(text, "Hello World");
});

test("Get /hello/jack returns Hello jack", async t => {
    const response = await fetch("http://0.0.0.0:8080/hello/jack");
    const text = await response.text();
    t.is(text, "Hello jack");
});

test("Get /headers returns headers", async t => {
    const sentHeaders = {
      testing: "testing",
    };

    const response = await fetch("http://0.0.0.0:8080/headers", { method: 'GET', headers: sentHeaders});
    const headers = await response.json();

    t.not(headers.testing, undefined);
    t.is(headers.testing, sentHeaders.testing);
});

test("Get /params returns params", async t => {
    const response = await fetch("http://0.0.0.0:8080/params?testing=testing");
    const json = await response.json();
    const expecting = {
      testing: "testing",
    };

    t.deepEqual(json, expecting);
});

test("Get /json returns json", async t => {
    const response = await fetch("http://0.0.0.0:8080/json");
    const json = await response.json();

    const expecting = {
      hello: "world",
      json: "HERE"
    };

    t.deepEqual(json, expecting);
});

// Run this in serial as we're blocking the cpu here.
test.serial("Get /cpu returns Hello World", async t => {
    const response = await fetch("http://0.0.0.0:8080/cpu");
    const text = await response.text();
    t.is(text, "Hello World");
});

test("Post /return_text_body returns text body", async t => {
    const response = await fetch("http://0.0.0.0:8080/return_text_body", { method: 'POST', body: "testing" });
    const text = await response.text();

    t.is(text, "testing");
});