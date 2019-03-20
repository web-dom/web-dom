# web-dom

DOM access for web assembly
* no magic
* no abstractions
* no code generation
* api generated from webidl
* technology agnostic

```toml
[package]
name = "helloworld"
version = "0.0.1"
edition = "2018"

[lib]
crate-type =["cdylib"]

[dependencies]
web-dom = "0.0.3"
```
```rust
use web_dom::*;

#[no_mangle]
pub fn main() -> () {
    console::log("hello world")
}
```
```console
cargo build --target wasm32-unknown-unknown --release
```
```html
<script src="https://unpkg.com/@web-dom/web-dom@latest/web-dom.js"></script>
<web-dom module="helloworld.wasm"></web-dom>
```

See it working [here](https://web-dom.github.io/web-dom/examples/helloworld/)

# Alert

```rust
use web_dom::*;

#[no_mangle]
pub fn main() -> () {
    window::alert(window(),"hello world!");
}
```

See it working [here](https://web-dom.github.io/web-dom/examples/alert/)

# Canvas

```rust
use web_dom::*;

#[no_mangle]
pub fn main() -> () {
    let doc = window::get_document(window());
    let canvas = document::query_selector(doc,"#screen");
    let ctx = htmlcanvaselement::get_context(canvas,"2d");
    canvasrenderingcontext2d::fill_rect(ctx,0,0,50,50);
}
```

See it working [here](https://web-dom.github.io/web-dom/examples/canvas/)

# Events

```rust
use web_dom::*;

#[no_mangle]
pub fn callback(_listener:EventListener,_event:Event) -> () {
    let input = document::query_selector(document(),"input");
    let msg = htmlinputelement::get_value(input);
    window::alert(window(),&msg);
}

#[no_mangle]
pub fn main() -> () {
    let btn = document::query_selector(document(),"button");
    let listener = create_event_listener();
    eventtarget::add_event_listener(btn,"click",listener);
}
```

See it working [here](https://web-dom.github.io/web-dom/examples/events/)
