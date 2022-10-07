const cluster = require('node:cluster');
const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

const buf = Buffer.from(response, 'utf8');

if (cluster.isPrimary) {
  console.log(`Primary ${process.pid} is running`);

  // Fork workers.
  for (let i = 0; i < 8; i++) {
    cluster.fork();
  }

  cluster.on('exit', (worker, code, signal) => {
    console.log(`worker ${worker.process.pid} died`);
  });
} else {
    Walker.get("/", (res) => {
        res.sendBytesText(buf);
    });

    Walker.get("/async", async (res) => {
        await timeout(1);
        res.sendText("Hello world");
    });
  
    Walker.start("0.0.0.0:8081")
}