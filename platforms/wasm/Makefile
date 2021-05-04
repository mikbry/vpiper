all: www/app/vpiper.wasm

www/app/vpiper.wasm: target/wasm32-unknown-unknown/release/vpiper_core.wasm
	mkdir -p www/app
	cp target/wasm32-unknown-unknown/release/vpiper.wasm www/app/

target/wasm32-unknown-unknown/release/vpiper_core.wasm: src/lib.rs
	cargo build --release --target wasm32-unknown-unknown
	mv -f target/wasm32-unknown-unknown/release/vpiper_core.wasm target/wasm32-unknown-unknown/release/vpiper.wasm

clean:
	rm -R target www/app/*.wasm
