#!/bin/sh

if [ ! -d ./frontend/assets/posts ]; then
	echo posts directory does not exist\; Creating ./frontend/assets/posts/...
	mkdir -p ./frontend/assets/posts/
fi
if [ ! -d ./frontend/dist ]; then
	echo dist directory does not exist\; Creating ./frontend/dist...
	mkdir -p ./frontend/dist
fi

case $1 in
	"--release")
		rm -rf ./target/**/dist
		rm -rf ./target/**/assets

		ln -s $(pwd)/frontend/assets ./target/release/
		ln -s $(pwd)/frontend/dist ./backend/
		;;
	"--debug")
		link_debug
		;;
	"")
		link_debug
		;;
esac
