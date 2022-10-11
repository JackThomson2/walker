const Walker = require('..');

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const do_thing = async () => {
    let conn = await Walker.connectDb("postgresql://localhost:5432?user=postgres&password=test");

    console.log('We have the connection...')

    let query_res = await conn.query("SELECT * FROM main ORDER BY id ASC LIMIT 10;");

    console.log(query_res);
}

do_thing();