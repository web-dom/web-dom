WebDOM.run("example.wasm",{
  global_sys_call:function(){
    console.log(arguments);
  }
})
