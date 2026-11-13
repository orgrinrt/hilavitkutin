// Prints the size, exports and imports of a wasm module.
// Usage: node inspect.mjs <path-to.wasm>
import { readFileSync } from "node:fs";

const bytes = readFileSync(process.argv[2]);
const module = new WebAssembly.Module(bytes);
console.log(`bytes: ${bytes.length}`);
console.log(`exports: ${JSON.stringify(WebAssembly.Module.exports(module))}`);
console.log(`imports: ${JSON.stringify(WebAssembly.Module.imports(module))}`);
