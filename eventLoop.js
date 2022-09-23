const Walker = require('.');

let count = 1;

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

function testOtherFunNow() {
    return `hello world ${count++}`
}

const callMeNow = async () => {
    await timeout(1);
    console.log(`hello world ${count++}`);
}

Walker.registerConst(testOtherFunNow);

console.time("CallNapi");
for (let i = 0; i < 10_000_000; i++) {
    Walker.eventLoop();
}
console.timeEnd("CallNapi");