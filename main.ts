import init, { minify_css as minifyCss } from "./pkg/marindeck_minifier.js";

await init(Deno.readFile("./pkg/marindeck_minifier_bg.wasm"));

const css = new TextDecoder("utf8").decode(
  await Deno.readFile("./mod/custom.css"),
);
const minified = minifyCss(css);

console.log(minified.content);
