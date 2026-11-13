// The node front end: loads the wasm module and prints the same lines as the
// native binary. Usage: node run.mjs <path-to.wasm> <cases>
import { readFileSync } from "node:fs";

const [wasmPath, casesArg] = process.argv.slice(2);
const cases = Number(casesArg);
const { instance } = await WebAssembly.instantiate(readFileSync(wasmPath), {});
const e = instance.exports;
const view = () => new DataView(e.memory.buffer);
const hex = (v) => v.toString(16).padStart(16, "0");

const lines = [];
for (let c = 0; c < cases; c++) {
  const written = e.run_case_export(c);
  const out = e.out_ptr();
  const status = e.status_ptr();
  for (let i = 0; i < written; i++) {
    lines.push(`c${c} s${i + 1} ${hex(view().getBigUint64(out + 8 * i, true))}`);
  }
  const w = [0, 1, 2, 3, 4].map((i) => view().getBigUint64(status + 8 * i, true));
  let tail;
  if (w[1] === 0n) tail = "refusal=none";
  else if (w[1] === 1n) tail = `refusal=cells step=${w[2]} needed=${w[3]} cap=${w[4]}`;
  else tail = `refusal=out needed=${w[2]} have=${w[3]}`;
  lines.push(`c${c} end written=${w[0]} ${tail}`);
}
process.stdout.write(lines.join("\n") + "\n");
