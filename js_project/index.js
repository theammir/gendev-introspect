const rustIntrospect = require("./introspect/index.node");

const introspect = (func) => {
  return rustIntrospect.introspectPlain(func.toString());
}

// -------

const kill_me = (
  // Function title
  // Function description
  // And function description again
  arg1_name, // arg1_comment, ...
  // arg1 comment continues
  arg2 // boolean, arg2_comment

  // Example: comment1
  // Returns: comment2
) => {

};

const obj = introspect(kill_me);
console.dir(obj);

// vim: ts=2 sts=2 sw=2 et:
