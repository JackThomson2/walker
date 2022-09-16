const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

let counter = 0;

const pool = Walker.DbPool.new("postgresql://localhost:5432?user=postgres&password=test", 16);

Walker.get("/", (res) => {
    res.sendText(response);
});

Walker.get("/counter", (res) => {
    res.sendText(`Counter is : ${++counter}`);
});

Walker.get("/json", (res) => {
    res.sendObject({
        hello: "world",
        json: "HERE",
        count: `Counter is : ${++counter}`
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

Walker.get('/db_call', async (res) => {
    const query = await pool.query("SELECT * FROM main LIMIT 2");
    res.sendObject(query);
})

Walker.get('/db_insert', async (res) => {
    await pool.query(`INSERT INTO main(name, age) VALUES('COUNTER', ${++counter});`);
    res.sendObject({ok: true});
})

Walker.post("/post", (res) => {
    const body = res.getBody();
    res.sendText(body.toString('utf8'));
});

Walker.start("0.0.0.0:8081")
