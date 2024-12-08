# 🌊 Suez

Suez is a redis-like trivial key-value storage built with Rust standard library only.

The intent of this project is to learn basic networking with TCP and manage request with thread pool in Rust

**Disclaimer** Please don't use suez in production. This project is
intended to be a learning resource, and omits various parts of the Redis
protocol because implementing them would not introduce any new concepts. We will
not add new features because you need them in your project — use one of the
fully featured alternatives instead.

## Installation:

```sh-session
- clone the repo
- cargo run
- connect to port 31337 and you're set
```

## Commands:

All command is case-insensitive.

```sh-session
- GET <key>
- SET <key> <value>
- DEL / DELETE <key>
```
