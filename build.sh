#! /usr/bin/env sh

BROWSER=${1:-'chrome'}

set -eux pipefall

echo "removing old build artfacte"
rm -rf pkg

wasm-pack build --target=web || exit 1

echo "moving pkg stuff"
echo $BROWSER
cp -r ./_pkg/$BROWSER/** ./pkg
cp -r ./vendor/sqlite-wasm/jswasm/** ./pkg

