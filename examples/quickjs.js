const Walker = require('..');

const gethandler = async (req) => {
    // store all the keys of the request object
    let keys = Object.keys(req);



    return `Global is ${JSON.stringify(request)}   Keys are ${JSON.stringify(keys)} value is ${JSON.stringify(req)}`;
}

Walker.registerQuickJsHandler(gethandler);