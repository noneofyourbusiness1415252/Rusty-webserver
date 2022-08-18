const { fork, isMaster } = require("cluster");
for (let _ in require("os").cpus()) if (isMaster) fork();
require("./index").serve({
  "*+*": (a, b) =>
    `<link rel=stylesheet href=style.css><script src=script.js></script>${a} + ${b} = ${
      1 * a + 1 * b
    }`,
});
