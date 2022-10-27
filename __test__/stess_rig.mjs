import * as Walker from '../index.js'

const registerRoutes = () => {

    // Simple hello world endpoint
    Walker.get("/", (res) => {
        res.sendText("Hello World");
    });

    // Cpu blocking function
    Walker.get("/cpu", (res) => {
        let i = 0;
        while (i < 100000) {
            i++;
        }

        res.sendText("Hello World");
    });

    // Returns the body of the request
    Walker.post("/return_text_body", (res) => {
        res.sendBytesText(res.getBody());
    });
};

export default registerRoutes;