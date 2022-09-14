const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

Walker.newRoute("/", (res) => {
    res.setResponse(response);
});

Walker.newRoute("/hello/:name", (res) => {
    const params = res.getParams();
    res.setResponse(`Hello ${params.name}`);
});

Walker.newRoute("/async", async (res) => {
    await timeout(1);
    res.setResponse("Hello world");
});

console.log('Done');

Walker.start("0.0.0.0:8081")