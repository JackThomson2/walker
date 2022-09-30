let counter = 0;

GET('/route', () => {
    log("Hello from route 1");

    log(`Counter is ${++counter}`);
});

GET('/route2', () => {
    log("Hello from route 2");
    log(`Counter is ${++counter}`);
});


GET('/route3', () => {
    log("Hello from route 3");
    log(`Counter is ${++counter}`);
});
