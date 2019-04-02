export default function() {
    let allocations = [null];
    let empty = [];
    return {
        //allocate
        a(value) {
            const i = empty.length > 0
                ? empty.pop()
                : allocations.length;
            allocations[i] = value;
            return i;
        },
    
        //release
        r(handle) {
            delete allocations[handle];
            empty.push(handle);
        },
      
        // get
        g(handle) {
            if (handle < 0) {
                return;
            }
            const ret = allocations[handle];
            if (handle !== 0 && !ret) {
                console.error(`Asked for ${handle} after it was released.`);
            }
            return ret;
        },
    };
}
