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
  console.log(arg1_name, arg2);
};

const signature = introspect(kill_me);
console.dir(signature);

// vim: ts=2 sts=2 sw=2 et:
