#!/bin/sh
set -x

if [ ! -d ./frontend/assets/posts ]; then
	echo posts directory does not exist\; Creating ./frontend/assets/posts/...
	mkdir -p ./frontend/assets/posts/
fi
if [ ! -d ./frontend/dist ]; then
	echo dist directory does not exist\; Creating ./frontend/dist...
	mkdir -p ./frontend/dist
fi

link_debug() {
	rm -rf ./target/debug/dist
	rm -rf ./target/debug/assets

	ln -s $(pwd)/frontend/assets ./target/debug/
	ln -s $(pwd)/frontend/dist ./target/debug/
}

case $1 in
	"--all")
		rm -rf ./target/**/dist
		rm -rf ./target/**/assets

		ln -s $(pwd)/frontend/assets ./target/release/
		ln -s $(pwd)/frontend/dist ./target/release/

		ln -s $(pwd)/frontend/assets ./target/debug/
		ln -s $(pwd)/frontend/dist ./target/debug/
		;;
	"--release")
		rm -rf ./target/release/dist
		rm -rf ./target/release/assets

		ln -s $(pwd)/frontend/assets ./target/release/
		ln -s $(pwd)/frontend/dist ./target/release/
		;;
	"--debug")
		link_debug
		;;
	"")
		link_debug
		;;
esac

rm -rf ./backend/dist
rm -rf ./frontend/dist/assets
ln -s $(pwd)/frontend/dist ./backend/
ln -s $(pwd)/frontend/assets ./frontend/dist/
