#!/bin/sh

rm -rf ./target/debug/static
rm -rf ./target/debug/static.json

ln -s $(pwd)/src/static ./target/debug/static
ln -s $(pwd)/static.json ./target/debug/
