const Walker = require('.');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const response = "Hello World"

const buf = new Uint8Array(Buffer.from(response, 'utf8'));

let counter = 0;

const pool = {}; // Walker.DbPool.new("postgresql://localhost:5432?user=postgres&password=test", 16);

function do_resp(resp) {
    resp.sendText(response);
}

Walker.get("/", (res) => {
    res.sendText(response);
});

Walker.get("/normalFunc", do_resp);

Walker.get("/next_tick", (res) => {
    process.nextTick(() => res.sendText(response));
});

Walker.get("/setImmediate", (res) => {
    setImmediate(() => res.sendText(response));
});

Walker.get("/return text", (res) => {
    res.sendText(response);
});

Walker.get("/t", (res) => {
    res.sendText(response);
});

Walker.get("/b", (res) => {
    res.sendBytesText(buf);
});

Walker.get("/allHeaders", (res) => {
    let headers = res.getAllHeaders();

    res.sendObject(headers);
});

Walker.get("/template.html", (res) => {
    const data = {
        username: "Bob",
        numbers: [1,2,3,4,5,6,7,8],
        show_all: true,
        bio: "<script>alert('test')</script>",
        my_var: `Page visitors ${++counter}`
    };

    res.sendTemplateResp(data);
});



Walker.get("/counter", (res) => {
    res.sendText(`Counter is : ${++counter}`);
});

Walker.post("/body", async (res) => {
    let bytes = res.getBody();

    res.sendBytesText(bytes);
});

Walker.get("/no_resp", async (_) => {

});

Walker.get("/headers", (res) => {
    let count = res.headerLength();
    let found = res.getHeader("Accept");
    res.sendText(`We have ${count} headers accept header is ${found}`);
});

Walker.get("/params", (res) => {
    let headers = res.getParams();
    res.sendObject(headers);
});


Walker.get("/json", (res) => {
    res.sendObject({
        hello: "world",
        json: "HERE",
        count: `Counter is : ${++counter}`
    });
});

Walker.get("/sjson", (res) => {
    res.sendStringifiedObject(JSON.stringify({
        hello: "world",
        json: "HERE",
        count: `Counter is : ${++counter}`
    }));
});


Walker.get("/hello/:name", (res) => {
    const params = res.getParams();
    res.sendText(`Hello ${params.name}`);
});

Walker.get("/async", async (res) => {
    await timeout(1);
    res.sendText("Hello world");
});

Walker.get("/timeout", (res) => {
    setTimeout(() => {
        res.sendText(response);
    }, 1);
});

Walker.get('/db_call', async (res) => {
    const query = await pool.query("SELECT (name, age) FROM main LIMIT 2");
    res.sendObject(query);
})

Walker.get('/db_multi_call', async (res) => {
    const queries = [];

    for (let i = 0; i < 10; i++) {
        const query = pool.query("SELECT (name, age) FROM main LIMIT 2");
        queries.push(query);
    }

    const result = await Promise.all(queries);

    res.sendObject(result);
})

Walker.get('/db_multi_call_sync', async (res) => {
    const queries = [];

    for (let i = 0; i < 10; i++) {
        const query = await pool.query("SELECT (name, age) FROM main LIMIT 2");
        queries.push(query);
    }

    res.sendObject(queries);
})

Walker.get('/db_multi_call_native', async (res) => {
    const queries = [];

    for (let i = 0; i < 10; i++) {
        const query = "SELECT (name, age) FROM main LIMIT 10;";
        queries.push(query);
    }

    const result = await pool.multiQuery(queries);

    res.sendObject(result);
})

Walker.get('/db_insert', async (res) => {
    await pool.query(`INSERT INTO main(name, age) VALUES('COUNTER', ${++counter});`);
    res.sendObject({ok: true});
})

Walker.get('/db_count', async (res) => {
    const result = await pool.query(`SELECT reltuples AS estimate FROM pg_class WHERE relname = 'main';`);
    const value = parseInt(result[0][0][0]);
    res.sendObject({value});
})

Walker.post("/post", (res) => {
    const body = res.getBody();
    res.sendText(body.toString('utf8'));
});

Walker.start("0.0.0.0:8081")