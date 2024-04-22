# Welcome to Rust Remix Starterkit!

There are no docs. You're on your own kid.

## Development

Run this to get your database up. Don't forget to rename `.env.sample` to `.env`

```sh
docker-compose up -d
```

Install dependencies in web folder.

```sh
cd web && yarn
```

Afterwards, you can run the respective servers.

```sh
cd api && cargo run
cd web && yarn dev
```
