#!/bin/sh

if [ ! -d ./frontend/assets/posts ]; then
	echo posts directory does not exist\; Creating ./frontend/assets/posts/...
	mkdir -p ./frontend/assets/posts/
fi
if [ ! -d ./frontend/dist ]; then
	echo dist directory does not exist\; Creating ./frontend/dist...
	mkdir -p ./frontend/dist
fi

link_debug() {
	rm -rf ./target/**/static
	rm -rf ./target/**/static.json
	rm -rf ./target/**/dist
	rm -rf ./target/**/assets

	ln -s $(pwd)/backend/src/static ./target/debug/static
	ln -s $(pwd)/backend/static.json ./target/debug/
	ln -s $(pwd)/frontend/assets ./target/debug/
	ln -s $(pwd)/frontend/dist ./backend/
}

case $1 in
	"--release")
		rm -rf ./target/**/static
		rm -rf ./target/**/static.json
		rm -rf ./target/**/dist
		rm -rf ./target/**/assets

		ln -s $(pwd)/backend/src/static ./target/release/static
		ln -s $(pwd)/backend/static.json ./target/release/
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
