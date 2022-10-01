let counter = 0;

GET('/', (req) => {
    return "Hello world"
})

GET('/counter', (req) => {
    return `Count is ${++counter}`
});

GET('/route2', () => {
    log("Hello from route 2");
    log(`Counter is ${++counter}`);
});


GET('/route3', () => {
    log("Hello from route 3");
    log(`Counter is ${++counter}`);
});
