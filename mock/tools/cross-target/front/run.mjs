// The wasm host's runner: loads the module in node with no imports and
// prints the native host's lines from what it leaves in memory.
// Usage: node run.mjs <module.wasm>
import { readFileSync } from "node:fs";

const wasmPath = process.argv[2];
const { instance } = await WebAssembly.instantiate(readFileSync(wasmPath), {});
const e = instance.exports;
const view = () => new DataView(e.memory.buffer);
const hex = (v) => v.toString(16).padStart(16, "0");

const lines = [];
for (let c = 0; c < e.cases(); c++) {
  const written = e.run_case_export(c);
  const out = e.out_ptr();
  for (let i = 0; i < written; i++) {
    lines.push(`c${c} f${i + 1} ${hex(view().getBigUint64(out + 8 * i, true))}`);
  }
  const status = e.status_ptr();
  const w = [0, 1, 2].map((i) => view().getBigUint64(status + 8 * i, true));
  const refused = w[1] === 0n ? "none" : `${w[2]}`;
  lines.push(`c${c} end written=${w[0]} refused=${refused}`);
}
process.stdout.write(lines.join("\n") + "\n");
