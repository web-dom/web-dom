# web-dom

DOM access for web assembly
* no magic
* no abstractions
* no code generation
* api generated from webidl
* can be used with many languages that compile to web assembly

Features:
* [x] dom
* [x] events
* [x] canvas
* [x] localStorage
* [x] webgl
* [ ] audio
* [ ] networking

Documentation: https://docs.rs/web-dom/

```toml
web-dom = "0.1"
```

Want to create web components? Check out https://github.com/web-dom/webcomponent

```rust
use web_dom::*;

#[no_mangle]
pub fn main() -> () {
    console::log("hello world")
}
```
```html
<script src="http://unpkg.com/web-dom@latest/web-dom.min.js"></script>
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
web-dom = "0.1"
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
    canvas::fill_rect(ctx,0.0,0.0,50.0,50.0);
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


See it working [here](https://richardanaya.github.io/pong/)

# Don't like Rust?

web-dom can be used with any language that compiles to web assembly

```C
extern int global_window();
extern void window_alert(char*);

int main(void) {
  window_alert(global_window(),"hello world!");
}
```

```clojure
(extern global_get_window [])
(extern window_get_document [window])
(extern document_query_selector [document query])
(extern htmlcanvas_get_context [element context])
(extern drawing_set_fill_style [canvas color])
(extern drawing_fill_rect [canvas x y w h])

(def colors ("black" "grey" "red"))

(pub defn main []
  (let [window (global_get_window)
        document (window_get_document window)
        canvas (document_query_selector document "#screen")
        ctx (htmlcanvas_get_context canvas "2d")]
        (loop [x 0]
               (if (< x 3)
                   (do (drawing_set_fill_style ctx (mem32 (+ colors (* 4 x))))
                       (drawing_fill_rect ctx (* x 10) (* x 10) 50 50 )
                       (recur [x (+ x 1)]))))))
```

# Want to add your own functions?
Do you need a function I haven't provided? You can easily add your own function to your module context by calling WebDOM imperatively.

```
WebDOM.run("my.wasm",{
    my_function: function(){
    }
}
```

# Dynamic Calls
In some situations your application may need to utilize behavior in external web assembly modules. Imagine for instance a library called `calculator.wasm` that knows how to do calculations for me. I can dynamically call an exported function on the module like so.

```rust
use web_dom::*;

fn add(calc:DOMReference, a:i32,b:i32) -> i32{
  let dyn_call = dynamic::begin(calc);
  dynamic::param_i32(dyn_call,a);
  dynamic::param_i32(dyn_call,b);
  dynamic::call(dyn_call,"add");
  let result = dynamic::result_i32(dyn_call);
  dynamic::unload(calc);
  result
}

#[no_mangle]
pub fn callback(_listener: EventListener, event: Event) -> () {
    // get a handle to the module
    let calc = get_property(event,"module");
    // call add
    let result = add(calc,2,2);
    console::log(&format!("{}",result));
}

#[no_mangle]
pub fn main() -> () {
  // Load web module asynchronously
  let load_listener = create_event_listener();
  dynamic::load("calculator.wasm",load_listener);
}
```

This approach has quite a lot of ceremony to it, but can enable some very powerful multi-language
projects. It's intended to be used for big libraries you wouldn't want to compile in yourself, libraries written in another language, or proprietary libraries.
