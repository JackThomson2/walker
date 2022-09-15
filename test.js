const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

const builder = Walker.ServerBuilder.newManager();

builder.get("/", (res) => {
    res.setResponse(response);
});

builder.get("/hello/:name", (res) => {
    const params = res.getParams();
    res.setResponse(`Hello ${params.name}`);
});

builder.get("/async", async (res) => {
    await timeout(1);
    res.setResponse("Hello world");
});

builder.post("/post", (res) => {
    const body = res.getBody();
    res.setResponse(body.toString('utf8'));
});

builder.start("0.0.0.0:8081")