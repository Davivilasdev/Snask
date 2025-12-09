# Blaze Web Framework (v1.1)

## Overview
Blaze is a lightweight web framework for the **Snask** programming language, inspired by Flask. It provides routing, middleware, template rendering, cookies, sessions, file uploads, and WebSocket support.

## Installation
The framework is shipped as part of the standard library. After building Snask (`cargo build --release`), the `blaze` module is available via `import blaze` (or automatically via `globals`).

## API Reference
| Function | Signature | Description |
|---|---|---|
| `blaze.create()` | `let app = blaze.create()` | Creates a new Blaze application instance. |
| `blaze.route(app, path, method, handler)` | `blaze.route(app, "/users/:id", "GET", fun(req, res) { … })` | Registers a route with dynamic parameters. |
| `blaze.use(app, middleware)` | `blaze.use(app, fun(req, res, next) { … })` | Adds a middleware function to the pipeline. |
| `blaze.request()` | `let req = blaze.request()` | Creates a new Request object (holds method, path, headers, body, query, params). |
| `blaze.response()` | `let res = blaze.response()` | Creates a new Response object (status, headers, body). |
| `blaze.set_status(res, code)` | `res = blaze.set_status(res, 200)` | Sets HTTP status code. |
| `blaze.set_header(res, key, value)` | `res = blaze.set_header(res, "Content-Type", "application/json")` | Adds a header. |
| `blaze.json(res, data)` | `let json_res = blaze.json(res, {"msg": "ok"})` | Serializes `data` to JSON and sets appropriate header. |
| `blaze.set_cookie(res, name, value)` | `res = blaze.set_cookie(res, "session", "abc123")` | Sets a cookie. |
| `blaze.create_session(app, id)` | `app = blaze.create_session(app, "sess1")` | Creates a session store identified by `id`. |
| `blaze.upload(req, field)` | `let info = blaze.upload(req, "file")` | Retrieves upload info for a form field. |
| `blaze.template(app, name, html)` | `app = blaze.template(app, "hello", "<h1>Hello {{name}}</h1>")` | Registers an HTML template with placeholders `{{var}}`. |
| `blaze.render(app, name, data)` | `let out = blaze.render(app, "hello", {"name": "World"})` | Renders a registered template with `data`. |
| `blaze.websocket(app, path)` | `let ws = blaze.websocket(app, "/ws")` | Creates a WebSocket endpoint. |
| `blaze.ws_send(ws, message)` | `blaze.ws_send(ws, "hello")` | Sends a message to all connected clients. |
| `blaze.listen(app, port)` | `blaze.listen(app, 3000)` | Starts the embedded HTTP server (async, runs until stopped). |

## Quick Start
```snask
let app = blaze.create()
app = blaze.route(app, "/", "GET", fun(req, res) {
    res = blaze.set_status(res, 200)
    res = blaze.set_header(res, "Content-Type", "text/plain")
    print("Hello from Blaze!")
})
blaze.listen(app, 8080)
```
Run with `snask my_app.snask`.

## Advanced Examples
### Routing with Parameters
```snask
app = blaze.route(app, "/user/:id", "GET", fun(req, res) {
    let user_id = req.params.id
    res = blaze.json(res, {"id": user_id})
})
```
### Middleware
```snask
app = blaze.use(app, fun(req, res, next) {
    print("Request received: " + req.path)
    next()
})
```
### Templates & Sessions
```snask
app = blaze.template(app, "profile", "<h1>Welcome {{name}}</h1>")
app = blaze.create_session(app, "sess1")
app = blaze.route(app, "/profile", "GET", fun(req, res) {
    let html = blaze.render(app, "profile", {"name": "Alice"})
    res = blaze.set_status(res, 200)
    res = blaze.set_header(res, "Content-Type", "text/html")
    print(html)
})
```
### WebSocket Chat
```snask
let ws = blaze.websocket(app, "/chat")
app = blaze.route(app, "/send", "POST", fun(req, res) {
    let msg = req.body.message
    blaze.ws_send(ws, msg)
    res = blaze.set_status(res, 200)
})
```

## FAQ & Troubleshooting
- **"Token inesperado" errors** – ensure you terminate statements with a newline, not a semicolon. The parser accepts function calls without `;`.
- **WebSocket not connecting** – make sure the client uses `ws://` and the server is listening on the same port.
- **Template placeholders not replaced** – placeholders must be wrapped in `{{` and `}}` and the data object must contain matching keys.

---
*Blaze* brings modern web capabilities to Snask while keeping the language’s simplicity.
