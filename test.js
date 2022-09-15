const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

Walker.get("/", (res) => {
    res.setResponse(response);
});

Walker.get("/hello/:name", (res) => {
    const params = res.getParams();
    res.setResponse(`Hello ${params.name}`);
});

Walker.get("/async", async (res) => {
    await timeout(1);
    res.setResponse("Hello world");
});

Walker.post("/post", (res) => {
    const body = res.getBody();
    res.setResponse(body.toString('utf8'));
});

Walker.start("0.0.0.0:8081")