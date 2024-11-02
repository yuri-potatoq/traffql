#!/bin/sh

echo "removing old build artefact"
rm -rf pkg

wasm-pack build --target=no-modules || exit 1

echo "moving pkg stuff"
cp -r ./_pkg/** ./pkg
