#! /bin/sh

rm -rf dist &&
./node_modules/.bin/parcel build web/index.html web/404.html --public-url /web &&
docker build -t duckception .