I probably wouldn't do this entire thing from the `HTML` down in Rust without WebAssembly again. I was making pizza for a
get together and had a few hours before it kicked off. I decided to build arguably the most minimal LAN pizza ordering app possible for fun. 

## Run
Either throw it in a docker container (optionally sync container timezone with local) or just run it on your local machine, doesn't take much.
You'll need to set a few env variables:
```
SERVER_ADDR=X.X.X.X # Address for web server.
SERVER_PORT=XXXX # Port for the web server.
```

## Misc
I have never gotten into frontend web development. It probably would have been more fun if I had taken the time to seperate
the components of this lil' app and not hacked together HTML rendering-ish utilities in Rust! Handlebars didn't work out
of the box for something I wanted so...like I said, only a few hours to crank it out!

