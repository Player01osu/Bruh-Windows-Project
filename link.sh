#!/bin/sh

rm -rf ./target/debug/static
rm -rf ./target/debug/static.json

ln -s $(pwd)/backend/src/static ./target/debug/static
ln -s $(pwd)/backend/static.json ./target/debug/
