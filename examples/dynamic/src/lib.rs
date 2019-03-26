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
