#! /bin/sh

rm -rf dist
./node_modules/.bin/parcel build web/index.html web/404.html --public-url /web
cargo build --release --features include_site
strip ./target/release/duckception