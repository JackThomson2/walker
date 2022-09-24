const Walker = require('.');

const response = "Hello World"

Walker.get("/", (res) => {
    Walker.registerConst(res, response);
});

Walker.start("0.0.0.0:8081")

console.time("CallNapi");

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

setTimeout(async () => {
    while (true) {
        await timeout(1);
        setTimeout(Walker.eventLoop);
    }
}, 100);