require("./index").serve({"/*+*": (a, b) => `<link rel=stylesheet href=0.css><script src=script.0.js></script>${a} + ${b} = ${1 * a + 1 * b}`, "/0.css": () => "*{background-color:#FDF6E3;color:#657BE3", "/0.js": () => "alert('hi')"})
console.log()