import * as Walker from '../index.js'

const registerRoutes = () => {

    Walker.get("/", (res) => {
        res.sendText("Hello World");
    });

    Walker.get("/unchecked", (res) => {
        res.sendTextUnchecked("Hello World");
    });

    Walker.get("/async", async (res) => {
        res.sendText("Hello World");
    });

    Walker.get("/sleep", async (res) => {
        await new Promise((resolve) => setTimeout(resolve, 60));
        res.sendText("Hello World");
    });

    Walker.get("/hello/:name", (res) => {
        const params = res.getUrlParams();
        res.sendText(`Hello ${params.name}`);
    });

    Walker.get("/status/:code", (res) => {
        const params = res.getUrlParams();

        res.setStatusCode(parseInt(params.code));
        res.sendText(`Status code: ${params.code}`);
    });

    Walker.get("/headers", (res) => {
        let headers = res.getAllHeaders();
    
        res.sendObject(headers);
    });

    Walker.get("/params", (res) => {
        let headers = res.getQueryParams();
        res.sendObject(headers);
    });

    Walker.get("/json", (res) => {
        res.sendObject({
            hello: "world",
            json: "HERE"
        });
    });

    Walker.get("/fastJson", (res) => {
        res.sendFastObject({
            hello: "world",
            json: "HERE"
        });
    });

    Walker.get("/stringifiedJson", (res) => {
        const object = {
            hello: "world",
            json: "HERE"
        };

        res.sendStringifiedObject(JSON.stringify(object));
    });

    Walker.get("/customHeader", (res) => {
        res.addHeader("custom", "header");

        res.sendText("Custom header added");
    });


    Walker.get("/internalServerError", (res) => {
        res.sendInternalServerError();
    });
    

    Walker.get("/errorWithMessage", (res) => {
        res.sendInternalServerErrorWithMessage("This is an error");
    });

    // A slow running function blocking the cpu
    Walker.get("/cpu", (res) => {
        let i = 0;
        while (i < 100000000) {
            i++;
        }

        res.sendText("Hello World");
    });

    Walker.post("/return_text_body", (res) => {
        res.sendBytesText(res.getBody());
    });
};

export default registerRoutes;