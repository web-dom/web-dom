let fs = require("fs");
let FUNCTIONS = [];
let WHITELIST = process.argv.slice(2);

function toInterfaceName(n) {
  return n.replace(" ", "");
}

function toSnake(s) {
  s = s.replace("HTML", "Html");
  s = s.replace("URL", "Url");
  s = s.replace("URI", "Uri");
  s = s.replace("ANGLE", "Angle");
  return s
    .replace(/(?:^|\.?)([A-Z])/g, function(x, y) {
      return "_" + y.toLowerCase();
    })
    .replace(/^_/, "");
}

function isPrimitive(n) {
  if (n == "object") {
    return false;
  }
  return true;
}

function finalNamespaceName(namespace) {
  namespace = namespace.toLowerCase();
  if (namespace == "canvasrenderingcontext2d") {
    return "canvas";
  }
  if (namespace == "webglrenderingcontext") {
    return "webgl";
  } else if (namespace != "element" && namespace.indexOf("element") != -1) {
    return namespace.replace("element", "");
  }
  return namespace.split("_").join("");
}

let namespaces = [];
function processNamespace(namespace) {
  namespace = finalNamespaceName(namespace);
  if (namespaces.includes(namespace)) {
    return namespace;
  }
  namespaces.push(namespace);
  fs.writeFileSync(
    "src/" + namespace + ".rs",
    "#[allow(unused_imports)]\nuse crate::*;\n"
  );
  return namespace;
}

function appendToNamespace(namespace, text) {
  namespace = finalNamespaceName(namespace);
  fs.appendFileSync("src/" + namespace + ".rs", text);
}

function toUnsafeRustType(returnType, returnNullable) {
  if (returnNullable) {
    return "DOMReference";
  }
  if (returnType == "string") {
    return "CString";
  }
  if (returnType == "number") {
    return "f32";
  }
  if (returnType == "object") {
    return "DOMReference";
  }
  return "i32";
}

function toRustParamType(returnType, returnNullable) {
  if (returnNullable) {
    return "DOMReference";
  }
  if (returnType == "string") {
    return "&str";
  }
  if (returnType == "boolean") {
    return "bool";
  }
  if (returnType == "number") {
    return "f32";
  }
  if (returnType == "object") {
    return "DOMReference";
  }
  return "i32";
}

function toRustReturnType(returnType, returnNullable) {
  if (returnNullable) {
    return "DOMReference";
  }
  if (returnType == "string") {
    return "String";
  }
  if (returnType == "boolean") {
    return "bool";
  }
  if (returnType == "number") {
    return "f32";
  }
  if (returnType == "object") {
    return "DOMReference";
  }
  return "i32";
}

let dom_api = JSON.parse(fs.readFileSync("dom_api.json", "utf8"));

for (i in dom_api) {
  let newNamespace = processNamespace(i);
  let seenMembers = {};
  for (f in dom_api[i]) {
    let member = dom_api[i][f];
    let trueName = member.name;
    let newName = toSnake(member.name);
    let returnType = member.return_type;
    let returnNullable = false; //member.return_nullable;
    let returnsString = returnType == "string";
    let returnsBool = returnType == "boolean";
    if (member.type == "property") {
      FUNCTIONS.push(`
        ${newNamespace}_get_${newName}: function(i) {
          let _i = A.g(i);
          return ${
            returnsString
              ? `this.ms(_i.${trueName})`
              : isPrimitive(returnType)
              ? `_i.${trueName}`
              : `A.a(_i.${trueName})`
          } ${returnsBool ? "? 1: 0" : ""};
        }`);
      FUNCTIONS.push(`
        ${newNamespace}_set_${newName}: function(i,v) {
          let _i = A.g(i);
          _i.${trueName} = ${
        returnsBool
          ? "1 == v"
          : returnsString
          ? `this.s(v)`
          : isPrimitive(returnType)
          ? `v`
          : ` A.g(v)`
      };
        }`);
      appendToNamespace(
        i,
        `extern \"C\" {
        fn ${newNamespace}_get_${newName}(instance:DOMReference) -> ${toUnsafeRustType(
          returnType,
          returnNullable
        )};
        fn ${newNamespace}_set_${newName}(instance:DOMReference,value:${toUnsafeRustType(
          returnType,
          returnNullable
        )});
    }\n
    pub fn get_${newName}(instance:DOMReference) -> ${toRustReturnType(
          returnType,
          returnNullable
        )} {\nunsafe{
      ${
        returnNullable
          ? ""
          : returnsBool
          ? "0 !="
          : returnsString
          ? "to_string("
          : ""
      } ${newNamespace}_get_${newName}(instance)
      ${returnsString ? ")" : ""}
      }\n}\n

      pub fn set_${newName}(instance:DOMReference,value:${toRustParamType(
          returnType,
          returnNullable
        )}){\nunsafe{
      ${newNamespace}_set_${newName}(instance,  ${
          returnsBool ? "if " : returnsString ? "to_cstring(" : ""
        }value${returnsString ? ")" : ""}${
          returnsBool ? " == true { 1 } else { 0 } " : ""
        });
            }\n}\n`
      );
    } else if (member.type == "function") {
      if (seenMembers[newName] == undefined) {
        seenMembers[newName] = 1;
      } else {
        let n = newName + "_" + seenMembers[newName];
        seenMembers[newName] = seenMembers[newName] + 1;
        newName = n;
      }

      let params = member.params;
      let returnType = member.return_type;
      let returnNullable = member.return_nullable;
      let hasReturn = member.return_type != null;
      let isStatic = member.is_static;
      FUNCTIONS.push(`
          ${newNamespace}_${newName}: function(${
        member.is_static ? "" : "i,"
      }${params.map(x => x.name).join(", ")}){
        ${member.is_static ? "" : "let _i = A.g(i);\n"}
              ${params
                .map(
                  x =>
                    `let _${x.name} = ${
                      x.type == "boolean"
                        ? `0 != ${x.name}`
                        : x.type == "string"
                        ? `this.s(${x.name})`
                        : x.type == "object"
                        ? ` A.g(${x.name})`
                        : x.name
                    };`
                )
                .join("\n")}
              ${
                hasReturn
                  ? returnsString
                    ? "return this.ms("
                    : returnType == "object"
                    ? "return A.a("
                    : "return ("
                  : "("
              }
              ${!isStatic ? "_i" : i}.${trueName}(${params
        .map(x => "_" + x.name)
        .join(", ")}))${returnsBool ? "?1:0" : ""};
          }`);
      appendToNamespace(
        i,
        `extern \"C\" {
        fn ${newNamespace}_${newName}(${
          isStatic ? "" : "instance:DOMReference,"
        }${params
          .map(x => newName + ":" + toUnsafeRustType(x.type))
          .join(", ")}) ${
          hasReturn ? ` -> ${toUnsafeRustType(returnType, returnNullable)}` : ""
        };
    }\n
    pub fn ` +
          newName +
          `(${isStatic ? "" : "instance:DOMReference,"}${params
            .map(x => toSnake(x.name) + ":" + toRustParamType(x.type))
            .join(", ")}) ${
            hasReturn
              ? ` -> ${toRustReturnType(returnType, returnNullable)}`
              : ""
          } {\nunsafe{
            ${
              returnNullable
                ? ""
                : returnsBool
                ? "0 !="
                : returnsString
                ? "to_string("
                : ""
            }
            ${newNamespace}_${newName}(${
            isStatic ? "" : "instance,"
          }${params
            .map(
              x =>
                (x.type == "boolean"
                  ? `if ${toSnake(x.name)}`
                  : x.type == "string"
                  ? `to_cstring(${toSnake(x.name)})`
                  : toSnake(x.name)) +
                (x.type == "boolean" ? " {1} else {0}" : "")
            )
            .join(", ")})
            ${returnNullable ? "" : returnsString ? ")" : ""}
          }\n}\n`
      );
    } else if (member.type == "const") {
      let constType = member.const_type;
      appendToNamespace(
        i,
        `pub const ${member.name.toUpperCase()}: f32 = ${
          member.value
        } as f32;\n`
      );
    }
  }
}

const template = `// THIS FILE IS GENERATED FROM tools/generate_webidl.js
import allocator from './allocator'

function createWebIDLContext(){
  let A = allocator();
  const webidl = {
    allocator: function () {return A;},

    global_create_f32array: function(start,length){
      return A.a(new Float32Array((Uint8Array.from(new Uint8Array(this.memory.buffer).subarray(start,start+length))).buffer));
    },

    global_create_uint8array: function(start,length){
      return A.a(new Uint8Array(this.memory.buffer).subarray(start, start + length));
    },

    global_is_null: function(o){
      return A.g(o) == null;
    },

    global_convert_to_ref: function(o){
      return A.g(o);
    },

    global_convert_to_number: function(o){
      return A.g(o);
    },

    global_convert_to_bool: function(o){
      return A.g(o) ? 1 : 0;
    },

    global_convert_to_string: function(o){
      return this.ms(A.g(o));
    },

    global_debugger: function(){
      debugger;
    },

    global_get_window: function(){
      return A.a(window);
    },

    global_release: function(handle) {
      A.r(handle);
    },

    global_create_event_listener: function() {
      let handle = A.a((e) => this.executeCallback(handle,e,A));
      return handle;
    },

    global_get_property: function(handle,name) {
      let o = A.g(handle);
      let p = o[this.s(name)];
      if(typeof p == "string"){
        return this.ms(p);
      } else if(typeof p == "boolean"){
        return p?1:0;
      } else if(typeof p == "number"){
        return p;
      }
      return A.a(p);
    },

    date_now: function() {
      return Date.now();
    },
    date_now_seconds: function() {
      return Date.now()/1000;
    },
    date_get_timezone_offset: function(){
      return new Date().getTimezoneOffset();
    },
    math_random: function() {
      return Math.random();
    },
    math_random_n: function(n) {
      return Math.floor(Math.random()*n);
    },
    customelement_attach_shadow: function(instance) {
      let _instance = A.g(instance);
      return A.a(_instance.attachShadow({mode:"open"}));
    },
    customelement_define: function(componentName) {
      componentName = this.s(componentName);
      let createElement = this.elementCreated;
      window.setTimeout(()=>{
        customElements.define(
          componentName,
          class extends HTMLElement {
            constructor() {
              super();
              var e = new CustomEvent("customelementcreated",{detail:A.a(this)});
              window.dispatchEvent(e);
            }
            connectedCallback() {
              var e = new CustomEvent("connected");
              this.dispatchEvent(e);
            }
            disconnectedCallback() {
              debugger;
              var e = new CustomEvent("disconnected");
              this.dispatchEvent(e);
            }
            adoptedCallback() {
              var e = new CustomEvent("adopted");
              this.dispatchEvent(e);
            }
            attributeChangedCallback(name, oldValue, value) {
              var e = new CustomEvent("attributechanged", {
                detail: { name, oldValue, value }
              });
              this.dispatchEvent(e);
            }
          }
        );
      },1);
    },
    customelement_define_with_attributes: function(componentName, attributes) {
      componentName = this.s(componentName);
      attributes = this.s(attributes);
      let createElement = this.elementCreated;
      let observedAttributes = attributes.split(",").map(x => x.trim());
      window.setTimeout(()=>{
        customElements.define(
          componentName,
          class extends HTMLElement {
            constructor() {
              super();
              var e = new CustomEvent("customelementcreated",{detail:A.a(this)});
              window.dispatchEvent(e);
            }
            static get observedAttributes() {
              return observedAttributes;
            }
            connectedCallback() {
              var e = new CustomEvent("connected");
              this.dispatchEvent(e);
            }
            disconnectedCallback() {
              debugger;
              var e = new CustomEvent("disconnected");
              this.dispatchEvent(e);
            }
            adoptedCallback() {
              var e = new CustomEvent("adopted");
              this.dispatchEvent(e);
            }
            attributeChangedCallback(name, oldValue, value) {
              var e = new CustomEvent("attributechanged", {
                detail: { name, oldValue, value }
              });
              this.dispatchEvent(e);
            }
          }
        );
      },1);
    },

    WasmWorker_onWorkerLoaded: function(instance,listener){
      let _instance = A.g(instance);
      let _listener = A.g(listener);
      if(_instance.loaded){
          _listener(new CustomEvent("load",{detail:{id:_instance.workerId}}))
      } else {
          _instance.addEventListener("load", _listener);
      }
    },
    WasmWorker_onWorkerMessage: function(instance,listener){
      let _instance = A.g(instance);
      let _listener = A.g(listener);
      _instance.addEventListener("message", _listener);
    },
    WasmWorker_sendWorkerMessage: function(instance,start,len){
      let _instance = A.g(instance);
      const data = new Uint8Array(this.memory.buffer)
      _instance.sendMessage(data.subarray(start,start+len))
    },
    WasmWorkerLoadEvent_getWorkerId: function(ev){
      let e = A.g(ev)
      return e.detail.id;
    },
    WasmWorkerMessageEvent_get_length: function(ev){
      let e = A.g(ev)
      return e.detail.length;
    },
    WasmWorkerMessageEvent_get_data: function(ev){
      let e = A.g(ev)
      let start = this.m(e.length);
      const data = new Uint8Array(this.memory.buffer)
      data.set(e.detail, start);
      return start
    },

    dynamic_load(moduleName,listener) {
      let _listener = A.g(listener);
      let handle = A.a({module:WebDOM.run(this.s(moduleName),{},function(){
        _listener({module:handle});
      },true),params:[],result:null});
    },
    dynamic_unload(dynHandle) {
      A.r(dynHandle);
    },
    dynamic_begin(dynHandle) {
      let call = A.g(dynHandle);
      call.params = [];
      call.result = null;
      return dynHandle;
    },
    dynamic_param_cstring(call, cstr) {
      window.alert("todo");
    },
    dynamic_param_memory(call, start, len) {
      window.alert("todo");
    },
    dynamic_param_i32(call, p) {
      let c = A.g(call);
      c.params.push(p);
    },
    dynamic_param_i64(call, p) {
      let c = A.g(call);
      c.params.push(p);
    },
    dynamic_param_f32(call, p) {
      let c = A.g(call);
      c.params.push(p);
    },
    dynamic_param_f64(call, p) {
      let c = A.g(call);
      c.params.push(p);
    },
    dynamic_call(call, name) {
      let c = A.g(call);
      let _name = this.s(name);
      c.result = c.module.exports[_name].apply(c.module.exports,c.params);
      c.params = [];
    },
    dynamic_result_i32(call) {
      let c = A.g(call);
      return c.result;
    },
    dynamic_result_i64(call) {
      let c = A.g(call);
      return c.result;
    },
    dynamic_result_f32(call) {
      let c = A.g(call);
      return c.result;
    },
    dynamic_result_f64(call) {
      let c = A.g(call);
      return c.result;
    },
    dynamic_result_cstring(call) {
      window.alert("todo");
    },
    dynamic_result_memory(call) {
      window.alert("todo");
    },
    dynamic_result_memory_len(call) {
      window.alert("todo");
    },

    //TODO: get rid of one day when this isn't required by tinygo
    io_get_stdout: function() {
      return 1;
    },

    //TODO: get rid of one day when this isn't required by tinygo
    resource_write: function(){
    },

    FUNCTIONS
  };
  return webidl;
}

export default createWebIDLContext;
`;

fs.writeFileSync(
  "js/webidl.js",
  template.replace("FUNCTIONS", FUNCTIONS.join(",\n"))
);

fs.writeFileSync(
  "src/lib.rs",
  `
  ${namespaces.map(x => `pub mod ${x};`).join("\n")}
  pub mod date;
  pub mod math;
  pub mod customelement;
  pub mod dynamic;
  mod global;
  pub use global::*;
  `
);
