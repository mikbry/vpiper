all: www/app/videopipe.wasm

www/app/videopipe.wasm: target/wasm32-unknown-unknown/release/videopipe_wasm.wasm
	mkdir -p www/app
	cp target/wasm32-unknown-unknown/release/videopipe.wasm www/app/

target/wasm32-unknown-unknown/release/videopipe_wasm.wasm: src/lib.rs
	cargo build --release --target wasm32-unknown-unknown
	mv -f target/wasm32-unknown-unknown/release/videopipe_wasm.wasm target/wasm32-unknown-unknown/release/videopipe.wasm

clean:
	rm -R target www/app/*.wasm
