let fs = require("fs");
let webidlParser = require("webidl2");

let WHITELIST = process.argv.slice(2);

function toInterfaceName(n) {
  return n.replace(" ", "");
}

let dom_api = {}

function isPrimitive(n) {
  if (n == "DOMString" || n == "long" || n == "double" || n == "short" || n == "unrestricted double" || n == "boolean" || n == "unsigned long" || n == "unsigned short" || n == "float" || n == "GLenum" || n == "GLfloat" || n == "GLint" || n == "GLboolean" || n == "GLsizei" || n == "GLintptr" || n == "GLuint" || n == "GLbitfield" || n == "GLclampf") {
    return true;
  }
  if (Array.isArray(n)) {
    return false;
  }
  return false;
}

let namespaces = [];

function processNamespace(namespace) {
  if (namespaces.includes(namespace)) {
    return namespace;
  }
  namespaces.push(namespace);
  dom_api[namespace] = [];
  return namespace;
}

function appendToNamespace(namespace, text) {
  fs.appendFileSync("src/" + namespace + ".rs", text);
}

function toType(type){
  if (type == "DOMString") {
    return "string"
  } else if (!isPrimitive(type)) {
    return "object"
  } else {
    if(type == "boolean" || type == "GLboolean"){
      return "boolean"
    }
    else {
      return "number"
    };
  }
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
  namespace = processNamespace(namespace);
  let operationName = operation.body.name.value;
  let returnsNullable = operation.body.idlType.nullable !== null;
  let params = [];
  let returnType = operation.body.idlType.idlType;
  let hasReturn = returnType != "void";
  let returnsString = returnType == "DOMString";
  for (a in operation.body.arguments) {
    let arg = operation.body.arguments[a];
    let name = arg.name;
    let type = arg.idlType.idlType;
    params.push({
      name,
      type: toType(type),
    })
  }
  let f = {type:"function",name:operationName,params:params};
  if(hasReturn){
    f.return_type = toType(returnType);
    if(returnsNullable){
      f.return_nullable = returnsNullable;
    }
  }
  if(!isInterface){
    f.is_static = true;
  }
  dom_api[namespace].push(f)
}

function processAttribute(interface, idl) {
  if (!idl.name) {
    return;
  }
  let namespace = processNamespace(interface);
  let name = idl.name;
  let primitive = isPrimitive(idl.idlType.idlType);
  let propType = "number";
  if (primitive) {
    if (idl.idlType.idlType == "DOMString") {
      propType = "string"
    } else if(idl.idlType.idlType == "boolean"){
      propType = "boolean"
    }
  } else {
    propType = "object"
  }
  dom_api[namespace].push({type:"property",name:name,return_type:propType})
}

function processConst(interface, idl) {
  if (!idl.name) {
    return;
  }
  let namespace = processNamespace(interface);
  let name = idl.name;
  let primitive = isPrimitive(idl.idlType.idlType);
  let propType = "number";
  if (primitive) {
    if (idl.idlType.idlType == "DOMString") {
      propType = "string"
    } else if(idl.idlType.idlType == "boolean"){
      propType = "boolean"
    }
  } else {
    propType = "object"
  }
  dom_api[namespace].push({type:"const",name:name,const_type:idl.value.type,value:idl.value.value})
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
        } else if (member.type == "attribute") {
          processAttribute(idl.name, member);
        } else if (member.type == "const") {
          processConst(idl.name, member);
        } else {
          console.log(member);
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

fs.writeFileSync(
  "dom_api.json",
  JSON.stringify(dom_api, null, 2)
);
