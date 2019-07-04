```
cargo install cargo-web
cargo web build --release

cd static/
ln -sf ../target/wasm32-unknown-unknown/release/game_of_life.wasm .
ln -sf ../target/wasm32-unknown-unknown/release/game_of_life.js .

python serve.py
```

then visit `localhost:8080`
