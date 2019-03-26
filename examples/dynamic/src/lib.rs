use web_dom::*;

fn add(a:i32,b:i32) -> i32{
  let calc = dynamic::load("calculator.wasm");
  let dyn_call = dynamic::begin(calc);
  dynamic::param_i32(dyn_call,a);
  dynamic::param_i32(dyn_call,b);
  dynamic::call(dyn_call,"add");
  let result = dynamic::result_i32(dyn_call);
  dynamic::unload(calc);
  result
}

#[no_mangle]
pub fn main() -> () {
  let result = add(2,2);
  console::log(&format!("{}",result));
}
