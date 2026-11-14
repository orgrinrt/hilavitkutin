// Runs the probe built for wasm and prints its digest stream the way the
// host's build prints its own: one digest a line, sixteen hex digits.
//
// cargo calls this as the target's runner, with the path of the module.

'use strict';

const fs = require('node:fs');

const [path] = process.argv.slice(2);
if (!path) {
    process.stderr.write('usage: run.cjs <module.wasm>\n');
    process.exit(2);
}

const lines = [];
const imports = {
    env: {
        emit: (digest) => {
            lines.push(BigInt.asUintN(64, digest).toString(16).padStart(16, '0'));
        },
    },
};

const built = new WebAssembly.Module(fs.readFileSync(path));
const instance = new WebAssembly.Instance(built, imports);
const main = instance.exports.main;
if (typeof main !== 'function') {
    process.stderr.write(`${path} exports no main\n`);
    process.exit(2);
}
main(0, 0);
process.stdout.write(lines.map((line) => `${line}\n`).join(''));
