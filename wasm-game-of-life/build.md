## prepare
```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install cargo-generate
```

## build
```
wasm-pack build
```
## test
```
wasm-pack test --chrome --headless
wasm-pack test --firefox --headless
```


## with a web page
```
npm init wasm-app www
```