'use strict';
const { readFile } = require('node:fs/promises');
const path = require('path');

require('./lib/wasm_exec.js');

globalThis.require = require;
globalThis.fs = require('fs');
globalThis.TextEncoder = require('util').TextEncoder;
globalThis.TextDecoder = require('util').TextDecoder;

const go = new Go();

(async () => {
    const buffer = await readFile('./src/main.wasm');
    const wasm = await WebAssembly.compile(buffer);
    const instance = await WebAssembly.instantiate(wasm, go.importObject);

    go.run(instance);

    let timer = 0;

    for (let i = 0; i < 10000; i++) {
        timer += globalThis.stringToMD5()
    }
    // timer += globalThis.stringToMD5()

    console.warn('Execute time:', timer)
})();
  