# LeafHttp Web Server
LeafHttp is a simple configurable web server.

```
⚠️ This project is for demo/educational purposes! It is *NOT* designed for production use.
```

# Features
- [X] Configurable IP/Port binding
- [X] Configurable WWW directory
- [X] Optional path traversal protection
- [X] Connection limiter
- [X] Can serve simple HTML/JS/CSS/etc. files
 - ⚠️ PHP is __not__ supported (will be served as plain text)!
- [ ] POST handling 🛠

# Compatibility
- ✅ Linux
- ✅ macOS
- ❌ Windows
    - Not supported due to `home_dir` using POSIX APIs.
    - This may be fixed in the future.

# Limitations/Notes
- The server strictly expects that requests are separated by `\r\n`. Using two newlines will cause parse errors (and thus 501s).
- Clients are handled on separate threads, so don't make `max-connections` too high (unless you have a ton of CPU cores).
- Only `GET` requests are supported.
- `max-request-size` counts the __entire__ request, not just the body.
- The request body __must__ contain valid UTF-8 characters. The server will respond with a 501 if you send a body with non-UTF-8 characters.

# Client handling
Every client is handled in a separate thread. When the connection limit is reached, the socket is immediately closed.

# Configuration
See [default_config.yml].

# Cross-compiling
Since `home_dir` depends on [`nix`](https://crates.io/crates/nix), you may get some linker errors.

If you get an error like `ld: unknown option: --as-needed` (especially on macOS), you can fix it by changing the linker. This can be done by creating a `.cargo/config.toml` config file, and specifying the following overrides:
```toml
[target.x86_64-unknown-linux-gnu] # macOS -> x86 Linux
linker = "x86_64-unknown-linux-gnu-gcc"
```
You'll need to adjust this to your needs.