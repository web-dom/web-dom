import webidl from "./webidl";

class WebDOMExecutor {
  constructor(wasmSrc, funcs) {
    fetch(wasmSrc)
      .then(response => response.arrayBuffer())
      .then(bytes => {
        let webidlContext = webidl();
        let env = {};
        let i;
        for (i in webidlContext) {
          env[i] = webidlContext[i].bind(this);
        }
        for (i in funcs) {
          env[i] = funcs[i].bind(this);
        }
        this.env = env;
        return WebAssembly.instantiate(bytes, { env });
      })
      .then(results => {
        this.memory = results.instance.exports.memory;
        this.exports = results.instance.exports;
        results.instance.exports.main();
        this.loaded = true;
      })
      .catch(e => {
        console.error(e);
      });
  }

  executeCallback(handle, ev, allocator) {
    let h = this.exports[this.callbackHandler];
    if (h) {
      if (ev) {
        // give the opportunity for event handler to grab what it needs
        let eventHandle = allocator.a(ev);
        h(handle, eventHandle);
        // then release event
        allocator.r(eventHandle);
      } else {
        h(handle, -1);
      }
    } else {
      throw new Error(
        "cannot call back without implementation of callback(source:i32,callback:i32)"
      );
    }
  }

  readStringFromMemory(start) {
    this.s(start);
  }

  s(start) {
    const data = new Uint8Array(this.memory.buffer);
    const str = [];
    let i = start;
    while (data[i] !== 0) {
      str.push(data[i]);
      i++;
    }
    return this.utf8dec.decode(new Uint8Array(str));
  }

  makeString(str) {
    this.ms(str);
  }

  ms(str) {
    if (!this.exports.malloc) {
      throw new Error(
        "Cannot return string to wasm with an implementation of malloc(size:i32)->i32 exposed on exports"
      );
    }
    let bytes = this.utf8enc.encode(str + String.fromCharCode(0));
    let len = bytes.length;
    let start = this.exports.malloc(len);
    const memory = new Uint8Array(this.memory.buffer);
    memory.set(bytes, start);
    return start;
  }

  malloc(len) {
    this.m(len);
  }

  m(len) {
    if (!this.exports.malloc) {
      throw new Error(
        "Cannot call malloc to wasm with an implementation of malloc(size:i32)->i32 exposed on exports"
      );
    }
    let start = this.exports.malloc(len);
    return start;
  }
}

export function run(wasmSrc, funcs) {
  let w = new WebDOMExecutor(wasmSrc, funcs);
}

const memories = {};

class WebIDLLoader extends HTMLElement {
  connectedCallback() {
    this.utf8dec = new TextDecoder("utf-8");
    this.utf8enc = new TextEncoder("utf-8");
    let wasmSrc = this.getAttribute("module");
    if (!wasmSrc) {
      console.error("no wasm module specified for wasm-module");
      return;
    }
    this.callbackHandler = this.getAttribute("callback") || "callback";
    this.mallocHandler = this.getAttribute("malloc") || "malloc";
    let exec = this.getAttribute("execute") || "main";
    let memory = this.getAttribute("memory") || "memory";
    let isWorker = this.getAttribute("worker");
    let workerId = parseInt(this.getAttribute("worker-id") || 0);
    this.workerId = workerId;
    let messageHandler = this.getAttribute("worker-message") || "message";
    let sharedMemory = this.getAttribute("shared-memory") || false;

    if (isWorker) {
      var response = `
      let utf8dec = new TextDecoder("utf-8");
      let memory = null;
      let instance = null;

      function fromCString(start) {
        const data = new Uint8Array(memory.buffer);
        const str = [];
        let i = start;
        while (data[i] !== 0) {
          str.push(data[i]);
          i++;
        }
        return utf8dec.decode(new Uint8Array(str));
      }

      self.onmessage=function(e){
        if(instance){
          let handler = instance.exports["${messageHandler}"];
          if(handler){
            if (!instance.exports["${this.mallocHandler}"]) {
              throw new Error(
                "Cannot postMessage to wasm without an implementation of malloc(size:i32)->i32 exposed on exports"
              );
            }
            let bytes = e.data;
            let len = bytes.length;
            let start = instance.exports["${this.mallocHandler}"](len);
            const m = new Uint8Array(memory.buffer);
            m.set(bytes, start);
            handler(start,len)
          }
        } else {
          fetch("${wasmSrc}")
            .then(response => response.arrayBuffer())
            .then(bytes => {
              let env = {
                global_postMessage:function(m,len){
                  const data = new Uint8Array(memory.buffer)
                  postMessage(data.subarray(m,m+len))
                },

                console_debug: function(message_start) {
                  let _message = fromCString(message_start);
                  console.debug(_message);
                },

                console_error: function(message_start) {
                  let _message = fromCString(message_start);
                  console.error(_message);
                },

                console_info: function(message_start) {
                  let _message = fromCString(message_start);
                  console.info(_message);
                },

                console_log: function(message_start) {
                  let _message = fromCString(message_start);
                  console.log(_message);
                },
              };
              return WebAssembly.instantiate(bytes, { env });
            })
            .then(results => {
              memory = results.instance.exports["${memory}"];
              instance = results.instance;
              results.instance.exports["${exec}"](${workerId});
              postMessage({type:"load",id:${workerId}});
            });
        }
      }`;
      var blob;
      try {
        blob = new Blob([response], { type: "application/javascript" });
      } catch (e) {
        window.BlobBuilder =
          window.BlobBuilder ||
          window.WebKitBlobBuilder ||
          window.MozBlobBuilder;
        blob = new BlobBuilder();
        blob.append(response);
        blob = blob.getBlob();
      }
      var worker = new Worker(URL.createObjectURL(blob));
      worker.onmessage = e => {
        if (!Array.isArray(e.data) && e.data.type == "load") {
          this.dispatchEvent(new CustomEvent("load", { detail: e.data }));
          this.loaded = true;
          return;
        }
        this.dispatchEvent(new CustomEvent("message", { detail: e.data }));
      };
      this.sendMessage = function(data) {
        worker.postMessage(data);
      };

      // start worker with a message
      worker.postMessage("start");
      return;
    }

    fetch(wasmSrc)
      .then(response => response.arrayBuffer())
      .then(bytes => {
        let webidlContext = webidl();
        let env = {};
        let i;
        for (i in webidlContext) {
          env[i] = webidlContext[i].bind(this);
        }
        if (sharedMemory) {
          throw new Error("Not supported yet");
        } else {
          return WebAssembly.instantiate(bytes, { env });
        }
      })
      .then(results => {
        this.memory = results.instance.exports[memory];
        this.exports = results.instance.exports;
        results.instance.exports[exec]();
        this.dispatchEvent(new CustomEvent("load"));
        this.loaded = true;
      })
      .catch(e => {
        console.error(e);
      });
  }

  executeCallback(handle, ev, allocator) {
    let h = this.exports[this.callbackHandler];
    if (h) {
      if (ev) {
        // give the opportunity for event handler to grab what it needs
        let eventHandle = allocator.a(ev);
        h(handle, eventHandle);
        // then release event
        allocator.r(eventHandle);
      } else {
        h(handle, -1);
      }
    } else {
      throw new Error(
        "cannot call back without implementation of callback(source:i32,callback:i32)"
      );
    }
  }

  //readStringFromMemory
  s(start) {
    const data = new Uint8Array(this.memory.buffer);
    const str = [];
    let i = start;
    while (data[i] !== 0) {
      str.push(data[i]);
      i++;
    }
    return this.utf8dec.decode(new Uint8Array(str));
  }

  //makeString
  ms(str) {
    if (!this.exports.malloc) {
      throw new Error(
        "Cannot return string to wasm with an implementation of malloc(size:i32)->i32 exposed on exports"
      );
    }
    let bytes = this.utf8enc.encode(str + String.fromCharCode(0));
    let len = bytes.length;
    let start = this.exports.malloc(len);
    const memory = new Uint8Array(this.memory.buffer);
    memory.set(bytes, start);
    return start;
  }

  m(len) {
    if (!this.exports.malloc) {
      throw new Error(
        "Cannot call malloc to wasm with an implementation of malloc(size:i32)->i32 exposed on exports"
      );
    }
    let start = this.exports.malloc(len);
    return start;
  }
}
window.customElements.define("web-dom", WebIDLLoader);
