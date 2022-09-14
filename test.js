const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

Walker.newRoute("/", Walker.Methods.GET, (res) => {
    res.setResponse(response);
});

Walker.newRoute("/hello/:name", Walker.Methods.GET, (res) => {
    const params = res.getParams();
    res.setResponse(`Hello ${params.name}`);
});

Walker.newRoute("/async", Walker.Methods.GET, async (res) => {
    await timeout(1);
    res.setResponse("Hello world");
});

Walker.newRoute("/post", Walker.Methods.POST, (res) => {
    const body = res.getBody();
    res.setResponse(body.toString('utf8'));
});

Walker.start("0.0.0.0:8081")