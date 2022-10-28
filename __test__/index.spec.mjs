import test from 'ava'
import axios from 'axios';

import registerRoutes from './standard_rig.mjs';

import * as Walker from '../index.js'

const JSON_MIME = 'application/json; charset=UTF-8';
const TEXT_MIME = 'text/plain; charset=UTF-8';

const Server = axios.create({
  baseURL: 'http://0.0.0.0:8080/'
});

test.serial.before(async (_) => {
  // This runs before all tests
  registerRoutes();

  Walker.startWithWorkerCount("0.0.0.0:8080", 1);

  // Sleeep for 100ms to let server start
  await new Promise((resolve) => setTimeout(resolve, 100));
});

test("Get / returns Hello World", async t => {
  const response = await Server.get("/");
  
  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Hello World");
});

test("Get /unchecked returns Hello World", async t => {
  const response = await Server.get("/unchecked");
  
  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Hello World");
});

test("Get /async returns Hello World", async t => {
  const response = await Server.get("/async");
  
  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Hello World");
});

test("Get /sleep returns Hello World", async t => {
  const response = await Server.get("/sleep");

  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Hello World");
});

test("Get /hello/jack returns Hello jack", async t => {
  const response = await Server.get("/hello/jack");

  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Hello jack");
});

test("Get /headers returns headers", async t => {
  const sentHeaders = {
    testing: "testing",
  };

  const response = await Server.get("/headers", { method: 'GET', headers: sentHeaders });
  const headers = response.data;

  t.is(response.headers['content-type'], JSON_MIME);
  t.not(headers.testing, undefined);
  t.is(headers.testing, sentHeaders.testing);
});

test("Get /params returns params", async t => {
  const response = await Server.get("/params?testing=testing");
  const json = response.data;
  const expecting = {
    testing: "testing",
  };

  t.is(response.headers['content-type'], JSON_MIME);
  t.deepEqual(json, expecting);
});

test("Get /json returns json", async t => {
  const response = await Server.get("/json");
  const json = response.data;

  const expecting = {
    hello: "world",
    json: "HERE"
  };

  t.is(response.headers['content-type'], JSON_MIME);
  t.deepEqual(json, expecting);
});

test("Get /fastJson returns json", async t => {
  const response = await Server.get("/fastJson");
  const json = response.data;

  const expecting = {
    hello: "world",
    json: "HERE"
  };

  t.is(response.headers['content-type'], JSON_MIME);
  t.deepEqual(json, expecting);
});

test("Get /stringifiedJson returns json", async t => {
  const response = await Server.get("/stringifiedJson");
  const json = response.data;

  const expecting = {
    hello: "world",
    json: "HERE"
  };

  t.is(response.headers['content-type'], JSON_MIME);
  t.deepEqual(json, expecting);
});

test("Get /status/200 returns 200", async t => {
  const response = await Server.get("/status/200");

  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Status code: 200");
  t.is(response.status, 200);
});

test("Get /status/201 returns 201", async t => {
  const response = await Server.get("/status/201");

  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.data, "Status code: 201");
  t.is(response.status, 201);
});

test("Get /customHeader returns custom headers", async t => {
  const response = await Server.get("/customHeader");

  t.is(response.headers['content-type'], TEXT_MIME);
  t.is(response.headers['custom'], "header");

  t.is(response.data, "Custom header added");
});

test("Get /internalError returns 500", async t => {
  try {
    const _ = await Server.get("/internalServerError");
    t.fail();
  } catch (error) {
    t.is(error.response.data, "Internal Server Error");
    t.is(error.response.status, 500);
  }
});

test("Get /errorWithMessage returns 500", async t => {
  try {
    const _ = await Server.get("/errorWithMessage");
    t.fail();
  } catch (error) {
    t.is(error.response.data, "This is an error");
    t.is(error.response.status, 500);
  }
});

test("Get /notFound returns 404", async t => {
  try {
    const _ = await Server.get("/notFound");
    t.fail();
  } catch (error) {
    t.is(error.response.status, 404);
  }
});

// Run this in serial as we're blocking the cpu here.
test.serial("Get /cpu returns Hello World", async t => {
  const response = await Server.get("/cpu");

  t.is(response.data, "Hello World");
});

test("Post /return_text_body returns text body", async t => {
  const response = await Server.post("/return_text_body", "testing");

  t.is(response.data, "testing");
});