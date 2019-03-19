# web-dom

DOM access for web assembly
* no magical
* no abstractions
* no code generation
* simple api
* technology agnostic

```toml
[package]
name = "helloworld"
version = "0.0.0"
edition = "2018"

[lib]
crate-type =["cdylib"]

[dependencies]
web-dom = "0.1"
```
```rust
#[no_mangle]
pub fn main() -> () {
    web_dom::console::log("hello world")
}
```
```console
cargo build --target wasm32-unknown-unknown --release
```
```html
<script src="https://unpkg.com/@web-dom/web-dom@latest/web-dom.js"></script>
<web-dom module="helloworld.wasm"></web-dom>
```
