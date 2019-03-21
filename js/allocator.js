export default function() {
  let allocations = [];
  let empty = [];
  return {
    //allocate
    a:function(value){
      let i = allocations.length;
      if(empty.length > 0) {
        i = empty.pop();
      }
      allocations[i] = value;
      return i;
    },
    //release
    r(handle){
        allocations[handle] = undefined;
        empty.push(handle);
    },
    //get
    g(handle){
      if(handle < 0){
        return undefined;
      }
      let ret =  allocations[handle];
      if(!ret){
        console.error(`Asked for ${handle} after it was released.`)
      }
      return ret;
    }
  }
}
