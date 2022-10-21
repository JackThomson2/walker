### Walker Server

Walker server is a node HTTP server with a rust backend.

The key focus of this project is to provide a simple, easy to use, and fast HTTP server.

#### Installation

```bash
npm install walker
```

#### Usage

```javascript
const Walker = require('walker');

Walker.get("/", (req) => {
    req.sendText("Hello World!");
});

Walker.start("0.0.0.0:8081", 12);
```

