const Walker = require('..');
const response = "Hello World!"

Walker.get(`/`, (res) => {
    res.sendTextUnchecked(response);
})

Walker.get(`/slowrunner/:count`, (res) => {
    const params = res.getUrlParams();
    const count = parseInt(params.count);
    let i = 0; 

    for (; i < count; i++) {

    }

    res.sendTextUnchecked(`Result ${i}`)
})

Walker.registerThreadsPool(50_000);
