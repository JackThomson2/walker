import test from 'ava'
import fetch from 'node-fetch';

import registerRoutes from './standard_rig.mjs';

import * as Walker from '../index.js'

test.serial.before(async (_) => {
	// This runs before all tests
  registerRoutes();

  Walker.startWithWorkerCount("0.0.0.0:8080", 1);

  // Sleeep for 100ms to let server start
  await new Promise((resolve) => setTimeout(resolve, 100));
});

test("Get / returns Hello World", async t => {
    const response = await fetch("http://0.0.0.0:8080/");
    const text = await response.text();
    t.is(text, "Hello World");
});

test("Get /unchecked returns Hello World", async t => {
    const response = await fetch("http://0.0.0.0:8080/unchecked");
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

test("Get /fastJson returns json", async t => {
  const response = await fetch("http://0.0.0.0:8080/fastJson");
  const json = await response.json();

  const expecting = {
    hello: "world",
    json: "HERE"
  };

  t.deepEqual(json, expecting);
});

test("Get /stringifiedJson returns json", async t => {
  const response = await fetch("http://0.0.0.0:8080/stringifiedJson");
  const json = await response.json();

  const expecting = {
    hello: "world",
    json: "HERE"
  };

  t.deepEqual(json, expecting);
});

test("Get /status/200 returns 200", async t => {
    const response = await fetch("http://0.0.0.0:8080/status/200");
    const text = await response.text();
    t.is(text, "Status code: 200");
    t.is(response.status, 200);
});

test("Get /status/201 returns 201", async t => {
    const response = await fetch("http://0.0.0.0:8080/status/201");
    const text = await response.text();
    t.is(text, "Status code: 201");
    t.is(response.status, 201);
});

test("Get /internalError returns 500", async t => {
    const response = await fetch("http://0.0.0.0:8080/internalServerError");
    const text = await response.text();

    t.is(text, "Internal Server Error");
    t.is(response.status, 500);
});

test("Get /errorWithMessage returns 500", async t => {
    const response = await fetch("http://0.0.0.0:8080/errorWithMessage");
    const text = await response.text();

    t.is(text, "This is an error");
    t.is(response.status, 500);
});

test("Get /notFound returns 404", async t => {
    const response = await fetch("http://0.0.0.0:8080/notFound");

    t.is(response.status, 404);
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