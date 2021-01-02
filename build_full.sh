#! /bin/sh

rm -rf dist
npm run build_web
npm run build_server
