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

## Setting up

### Mongodb

Before starting the server, mongod must be running in order to
serve the posts. By default, mongod runs on port `27017`, with
the database name being 'yuri-web-server', and default collection
name being 'yuriPosts'.

For posting, a script has been created that will curl a POST request
to the backend, and move the file into the posts directory.

#### Debian

```sh
$ curl -sSL https://www.mongodb.org/static/pgp/server-6.0.asc  -o mongoserver.asc
```

```sh
$ gpg --no-default-keyring --keyring ./mongo_key_temp.gpg --import ./mongoserver.asc
```

```sh
$ gpg --no-default-keyring --keyring ./mongo_key_temp.gpg --export > ./mongoserver_key.gpg
```

```sh
$ sudo mv mongoserver_key.gpg /etc/apt/trusted.gpg.d/
```

```sh
$ sudo apt update && sudo apt install mongodb-org
```

```sh
$ sudo systemctl enable --now mongod
```

#### Ubuntu/Mint

```sh
$ wget -qO - https://www.mongodb.org/static/pgp/server-4.2.asc | sudo apt-key add -
```

```sh
$ echo "deb [ arch=amd64 ] https://repo.mongodb.org/apt/ubuntu bionic/mongodb-org/4.2 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-4.2.list
```

```sh
$ sudo apt-get update && sudo apt-get install -y mongodb-org
```

```sh
$ sudo service mongod start
```

#### Arch

Through the aur with [mongodb](https://aur.archlinux.org/packages/mongodb/) or the pre-build binary from the
ubuntu repo [mongodb-bin](https://aur.archlinux.org/packages/mongodb-bin)

> NOTE: BUILDING MONGODB FROM SCRATCH REQUIRES A LOT OF SPACE AND TIME
> From the [arch wiki](https://wiki.archlinux.org/title/MongoDB#Installation) -
> requiring 180GB+ free disk space, and may take several hours to build (i.e. 6.5 hours on Intel i7, 1 hour on 32 Xeon cores with high-end NVMe.)

### Starting server (Release)

## Development

Clone this repository

```sh
$ git clone https://github.com/player01osu/Bruh-Web-Server.git`
```

## TODO

- [TODO](TODO.md)
