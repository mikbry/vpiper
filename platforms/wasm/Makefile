all: www/app/vpiper_bg.wasm

www/app/vpiper_bg.wasm: pkg/vpiper_bg.wasm
	mkdir -p www/app
	cp pkg/vpiper_bg.wasm www/app
	cp pkg/vpiper.js www/app

pkg/vpiper_bg.wasm: src/lib.rs
	wasm-pack build --target web --out-name vpiper

clean:
	rm -Rf pkg target www/app/vpiper_bg.wasm www/app/vpiper.js
  