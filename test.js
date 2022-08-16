const { serve } = require("./index");
serve({"*+*": (a, b) => `<link rel=stylesheet href=style.css><script src=script.js></script>${a} + ${b} = ${1 * a + 1 * b}`})
console.log()