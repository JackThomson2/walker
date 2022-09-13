const Walker = require('.');

let callCount = 0;

console.log('Adding route');

Walker.newRoute("/", (res) => {
    res.setResponse("Hello world");
});

console.log('Done');

Walker.start("0.0.0.0:8081")