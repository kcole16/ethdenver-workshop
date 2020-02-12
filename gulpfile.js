const gulp = require("gulp");
const nearUtils = require("near-bindgen-as/compiler");

function build_wasm(done){
  nearUtils.compile("./contracts/assembly/main.ts", "./contracts/out/main.wasm", done);
};

const build = gulp.series(build_wasm);


exports.default = build;

