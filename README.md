# axum-mini-template

## Install

```shell
cargo install --locked bacon
```

## Prepare

```shell
# create sqlite database
mkdir -p data
touch data/db.sqlite
```

## Start

```shell
# Terminal1: watch web server
bacon dev

# Terminal2: watch the http test
bacon http-test
```
