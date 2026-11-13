// Load the wasm build of a4-names-on-wasm and report. The case that must
// fail: a const name resolving inside its own image on wasm32.
// Usage: node run.mjs <path to a4_names_on_wasm.wasm>
// Output in output-wasm.txt was taken with node v22.23.4 on 2026-11-13.
import { readFileSync } from "node:fs";

const bytes = readFileSync(process.argv[2]);
const { instance } = await WebAssembly.instantiate(bytes, {});
const resolves = instance.exports.own_name_resolves();
const entries = instance.exports.section_entries();
console.log(`wasm32: section entries ${entries}; own const name resolves: ${resolves === 1}`);
if (resolves !== 1) {
  console.log("FAILED: a name declared with str_const! does not resolve in its own wasm image");
  process.exitCode = 1;
}
