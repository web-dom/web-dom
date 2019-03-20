let fs = require("fs");
let webidlParser = require("webidl2");

let FUNCTIONS = [];

let WHITELIST = process.argv.slice(2);

function toInterfaceName(n) {
  return n.replace(" ", "");
}

function toSnake(s) {
  return s
    .replace(/(?:^|\.?)([A-Z])/g, function(x, y) {
      return "_" + y.toLowerCase();
    })
    .replace(/^_/, "");
}

function isPrimitive(n) {
  if (n == "DOMString") {
    return true;
  }
  if (Array.isArray(n)) {
    return false;
  }
  return n[0] == n[0].toLowerCase();
}

let namespaces = [];

function processNamespace(namespace) {
  if (namespaces.includes(namespace)) {
    return;
  }
  namespaces.push(namespace);
  fs.writeFileSync(
    "src/" + namespace.toLowerCase() + ".rs",
    "#[allow(unused_imports)]\nuse crate::*;\n"
  );
}

function appendToNamespace(namespace, text) {
  fs.appendFileSync("src/" + namespace.toLowerCase() + ".rs", text);
}

function processOperation(namespace, operation, isInterface) {
  if (
    operation.extAttrs &&
    operation.extAttrs.trivia.open.indexOf("[ChromeOnly]") !== -1
  ) {
    return;
  }
  if (operation.body.name == null) {
    return;
  }
  processNamespace(namespace);
  let operationName = operation.body.name.value;
  let args = [];
  let params = [];
  let extractors = [];
  if (isInterface) {
    params.push({
      name: "instance",
      type: "number",
      description:
        "number that represents a handle to an " + namespace + " instance"
    });
    extractors.push(`let _instance = ALLOCATOR.g(instance);`);
  }
  let returnType = operation.body.idlType.idlType;
  let hasReturn = returnType != "void";
  let returnsString = returnType == "DOMString";
  for (a in operation.body.arguments) {
    let arg = operation.body.arguments[a];
    let name = toSnake(arg.name);
    let type = arg.idlType.idlType;
    if (type == "DOMString") {
      params.push({
        name: name,
        originalType: "DOMString",
        type: "number",
        description: 'memory location of string "' + name + '"'
      });
      extractors.push(`let _${name} = this.s(${name});`);
    } else if (!isPrimitive(type)) {
      extractors.push(`let _${name} = ALLOCATOR.g(${name});`);
      params.push({
        name,
        type: "number",
        originalType: type,
        description: type + " represented as a number"
      });
    } else {
      extractors.push(`let _${name} = ${name};`);
      params.push({
        name,
        originalType: type,
        type: "number",
        description: type + " represented as a number"
      });
    }
    args.push({ name, type });
  }
  FUNCTIONS.push(`
      ${namespace.toLowerCase()}_${toSnake(
    operationName
  )}: function(${params.map(x => x.name).join(", ")}){
          ${extractors.join("\n")}
          ${
            hasReturn
              ? returnsString
                ? "return this.ms("
                : "return ALLOCATOR.a("
              : "("
          }
          ${isInterface ? "_instance" : namespace}.${operationName}(${args
    .map(x => "_" + x.name)
    .join(", ")}));
      }`);
  appendToNamespace(
    namespace,
    `extern \"C\" {
    fn ${namespace.toLowerCase()}_${toSnake(operationName)}(${params
      .map(
        x =>
          toSnake(x.name) +
          (x.originalType == "DOMString" ? ":CString" : ":i32")
      )
      .join(", ")}) ${
      hasReturn ? ` -> ${returnsString ? "CString" : "i32"}` : ""
    };
}\n
pub fn ` +
      toSnake(toSnake(operationName)) +
      `(${params
        .map(
          x =>
            toSnake(x.name) + (x.originalType == "DOMString" ? ":&str" : ":i32")
        )
        .join(", ")}) ${
        hasReturn ? ` -> ${returnsString ? "String" : "i32"}` : ""
      } {\nunsafe{
        ${returnsString ? "cstr_to_string(" : ""}
        ${namespace.toLowerCase()}_${toSnake(operationName)}(${params
        .map(x =>
          x.originalType == "DOMString"
            ? `cstr(${toSnake(x.name)})`
            : toSnake(x.name)
        )
        .join(", ")})
        ${returnsString ? ")" : ""}
      }\n}\n`
  );
}

function processAttribute(interface, idl) {
  if (!idl.name) {
    return;
  }
  processNamespace(interface);
  let name = idl.name;
  let primitive = isPrimitive(idl.idlType.idlType);
  let returnsString = idl.idlType.idlType == "DOMString";
  if (primitive) {
    if (idl.idlType.idlType == "DOMString") {
      FUNCTIONS.push(`
        ${interface.toLowerCase()}_get_${toSnake(name)}: function(instance) {
          let _instance = ALLOCATOR.g(instance);
          return this.ms(_instance.${name});
        }`);
      FUNCTIONS.push(`
        ${interface.toLowerCase()}_set_${toSnake(
        name
      )}: function(instance,str) {
          let _instance = ALLOCATOR.g(instance);
          _instance.${name} = this.s(str);
        }`);
    } else {
      FUNCTIONS.push(`
        ${interface.toLowerCase()}_get_${toSnake(name)}: function(instance) {
          let _instance = ALLOCATOR.g(instance);
          return _instance.${name};
        }`);
      FUNCTIONS.push(`
        ${interface.toLowerCase()}_set_${toSnake(
        name
      )}: function(instance,val) {
          let _instance = ALLOCATOR.g(instance);
          _instance.${name} = val;
        }`);
    }
  } else {
    FUNCTIONS.push(`
      ${interface.toLowerCase()}_get_${toSnake(name)}: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.${name});
      }`);
    FUNCTIONS.push(`
      ${interface.toLowerCase()}_set_${toSnake(
      name
    )}: function(instance,handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.${name} = ALLOCATOR.g(handle);
      }`);
  }
  appendToNamespace(
    interface,
    `extern \"C\" {
    fn ${interface.toLowerCase()}_get_${toSnake(name)}(instance:i32) ${
      returnsString ? " -> CString" : " -> i32"
    };
    fn ${interface.toLowerCase()}_set_${toSnake(name)}(instance:i32,value:i32);
}\n
pub fn get_${toSnake(name)}(instance:i32) ${
      returnsString ? " -> String" : " -> i32"
    } {\nunsafe{
  ${
    returnsString ? "cstr_to_string(" : ""
  } ${interface.toLowerCase()}_get_${toSnake(name)}(instance)
  ${returnsString ? ")" : ""}
  }\n}\n

  pub fn set_${toSnake(name)}(instance:i32,value:i32){\nunsafe{
    ${interface.toLowerCase()}_set_${toSnake(name)}(instance,value);
        }\n}\n`
  );
}

function processIdl(idls, file) {
  console.log(`Processing \`${file}\`...`);
  for (i in idls) {
    let idl = idls[i];
    if (idl.type === "namespace") {
      for (m in idl.members) {
        let member = idl.members[m];
        if (member.type == "operation") {
          processOperation(idl.name, member);
        }
      }
    } else if (idl.type === "interface") {
      for (m in idl.members) {
        let member = idl.members[m];
        if (member.type == "operation") {
          processOperation(idl.name, member, true);
        }
        if (member.type == "attribute") {
          processAttribute(idl.name, member);
        }
      }
    }
  }
}

fs.readdirSync("webidl/").forEach(file => {
  if (WHITELIST.indexOf(file) != -1) {
    var text = fs.readFileSync("webidl/" + file, "utf8");
    processIdl(webidlParser.parse(text), file);
  }
});

const template = `// THIS FILE IS GENERATED FROM tools/generate_webidl.js
import allocator from './allocator'

function createWebIDLContext(){
  let ALLOCATOR = allocator();
  const webidl = {
    global_debugger: function(){
      debugger;
    },

    global_get_window: function(){
      return ALLOCATOR.a(window);
    },

    global_release: function(handle) {
      ALLOCATOR.r(handle);
    },
    global_create_event_listener: function() {
      let handle = ALLOCATOR.a((e) => this.executeCallback(handle,e,ALLOCATOR));
      return handle;
    },
    global_get_property: function(handle,name) {
      let o = ALLOCATOR.g(handle);
      let p = o[this.s(name)];
      if(typeof p == "string"){
        return this.ms(p);
      } else if(typeof p == "boolean"){
        return p?1:0;
      } else if(typeof p == "number"){
        return p;
      }
      return ALLOCATOR.a(p);
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
    element_attach_shadow: function(instance) {
      let _instance = ALLOCATOR.g(instance);
      return ALLOCATOR.a(_instance.attachShadow({mode:"open"}));
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
              var e = new CustomEvent("customelementcreated",{detail:ALLOCATOR.a(this)});
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
              var e = new CustomEvent("customelementcreated",{detail:ALLOCATOR.a(this)});
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
      let _instance = ALLOCATOR.g(instance);
      let _listener = ALLOCATOR.g(listener);
      if(_instance.loaded){
          _listener(new CustomEvent("load",{detail:{id:_instance.workerId}}))
      } else {
          _instance.addEventListener("load", _listener);
      }
    },
    WasmWorker_onWorkerMessage: function(instance,listener){
      let _instance = ALLOCATOR.g(instance);
      let _listener = ALLOCATOR.g(listener);
      _instance.addEventListener("message", _listener);
    },
    WasmWorker_sendWorkerMessage: function(instance,start,len){
      let _instance = ALLOCATOR.g(instance);
      const data = new Uint8Array(this.memory.buffer)
      _instance.sendMessage(data.subarray(start,start+len))
    },
    WasmWorkerLoadEvent_getWorkerId: function(ev){
      let e = ALLOCATOR.g(ev)
      return e.detail.id;
    },
    WasmWorkerMessageEvent_get_length: function(ev){
      let e = ALLOCATOR.g(ev)
      return e.detail.length;
    },
    WasmWorkerMessageEvent_get_data: function(ev){
      let e = ALLOCATOR.g(ev)
      let start = this.m(e.length);
      const data = new Uint8Array(this.memory.buffer)
      data.set(e.detail, start);
      return start
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
  ${namespaces.map(x => `pub mod ${x.toLowerCase()};`).join("\n")}
  pub mod date;
  pub mod math;
  pub mod customelement;
  mod global;
  pub use global::*;
  `
);
