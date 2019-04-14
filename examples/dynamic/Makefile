build:
	@cargo +nightly build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/example.wasm .
serve:
	http-server -p 8080
