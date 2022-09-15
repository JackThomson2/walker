const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

Walker.get("/", (res) => {
    res.sendText(response);
});

Walker.get("/json", (res) => {
    res.sendObject({
        hello: "world",
        json: "HERE",
    });
});

Walker.get("/hello/:name", (res) => {
    const params = res.getParams();
    res.sendText(`Hello ${params.name}`);
});

Walker.get("/async", async (res) => {
    await timeout(1);
    res.sendText("Hello world");
});

Walker.post("/post", (res) => {
    const body = res.getBody();
    res.sendText(body.toString('utf8'));
});

Walker.start("0.0.0.0:8081")