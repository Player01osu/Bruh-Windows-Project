# Yuri Web Server

Web server source code for yuri website.

Built using actix framework and yew framework

## Requirements

- Rust 1.58+
- Trunk
- wasm32-unknown-unknown compilation target
- Mongodb

## Testing

```sh
$ cargo build --bin backend

$ ./link.sh

$ cd ./target/debug/

$ ./backend

```
New shell
```sh
$ cd ./frontend

$ trunk serve --proxy-backend=http://127.0.0.1:7878/api
```

Frontend binded on `localhost:8080`

Backend binded on `localhost:7878`

Make sure mongodb is running on port `27017`

## Development

Clone this repository

```sh
$ git clone https://github.com/player01osu/Bruh-Web-Server.git`
```

## TODO

- [TODO](TODO.md)
