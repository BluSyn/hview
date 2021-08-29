hview
==========

A file viewer / image & video gallery.
Backend + Frontend built in rust with `actix-web` and `yew`.

## Usage

```sh
cd backend
cargo run -- -d <path_to_files> -p <port:8000> -h <host:localhost>
cd frontend
trunk serve --proxy-backend=http://<backend_host>:<backend_port> --proxy-rewrite=/api/
```

**NOTE**: This is just a hobby project for demo purposes.
Built as a learning experience with Rust, Yew, and similar frameworks.
As such, this is still mostly incomplete.
