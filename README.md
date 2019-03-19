# web-dom

DOM access for web assembly
* no magical code generation
* simple api
* technology agnostic

```rust
use web_dom::*;
#[no_mangle]
pub fn main() -> () {
    web_dom::console::log("hello world")
}
```
```console
cargo build --target wasm32-unknown-unknown --release
```
```html
<script src="https://unpkg.com/@web-dom/web-dom@latest/web-dom.min.js"></script>
<web-dom module="drawing.wasm"></web-dom>
```
