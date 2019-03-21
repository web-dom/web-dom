# web-dom

DOM access for web assembly
* no magic
* no abstractions
* no code generation
* api generated from webidl
* technology agnostic

```toml
web-dom = "0.0.8"
```

Documentation: https://docs.rs/web-dom/

Want to create web components? Check out https://github.com/web-dom/webcomponent

```rust
use web_dom::*;

#[no_mangle]
pub fn main() -> () {
    console::log("hello world")
}
```
```html
<script src="http://unpkg.com/web-dom@latest/web-dom.js"></script>
<web-dom module="helloworld.wasm"></web-dom>
```
```toml
[package]
name = "helloworld"
version = "0.0.1"
edition = "2018"

[lib]
crate-type =["cdylib"]

[dependencies]
web-dom = "0.0.7"
```
```console
cargo build --target wasm32-unknown-unknown --release
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
    let ctx = htmlcanvas::get_context(canvas,"2d");
    drawing::fill_rect(ctx,0.0,0.0,50.0,50.0);
}
```

See it working [here](https://web-dom.github.io/web-dom/examples/canvas/)

# Events

```rust
use web_dom::*;

#[no_mangle]
pub fn callback(_listener:EventListener,_event:Event) -> () {
    let input = document::query_selector(document(),"input");
    let msg = htmlinput::get_value(input);
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

# Pong

https://github.com/richardanaya/pong/
