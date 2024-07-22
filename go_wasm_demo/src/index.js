'use strict';
const { readFile } = require('node:fs/promises');
const path = require('path');

require('./lib/wasm_exec.js');

globalThis.require = require;
globalThis.fs = require('fs');
globalThis.TextEncoder = require('util').TextEncoder;
globalThis.TextDecoder = require('util').TextDecoder;

const go = new Go();

// const inputStr = `
// WASI support in Go
// Johan Brandhorst-Satzkorn, Julien Fabre, Damian Gryski, Evan Phoenix, and Achille Roussel
// 13 September 2023

// Go 1.21 adds a new port targeting the WASI preview 1 syscall API through the new GOOS value wasip1. This port builds on the existing WebAssembly port introduced in Go 1.11.

// What is WebAssembly?
// WebAssembly (Wasm) is a binary instruction format originally designed for the web. It represents a standard that allows developers to run high-performance, low-level code directly in web browsers at near-native speeds.

// Go first added support for compiling to Wasm in the 1.11 release, through the js/wasm port. This allowed Go code compiled using the Go compiler to be executed in web browsers, but it required a JavaScript execution environment.

// As the use of Wasm has grown, so have use cases outside of the browser. Many cloud providers are now offering services that allow the user to execute Wasm executables directly, leveraging the new WebAssembly System Interface (WASI) syscall API.

// The WebAssembly System Interface
// WASI defines a syscall API for Wasm executables, allowing them to interact with system resources such as the filesystem, the system clock, random data utilities, and more. The latest release of the WASI spec is called wasi_snapshot_preview1, from which we derive the GOOS name wasip1. New versions of the API are being developed, and supporting them in the Go compiler in the future will likely mean adding a new GOOS.

// The creation of WASI has allowed a number of Wasm runtimes (hosts) to standardize their syscall API around it. Examples of Wasm/WASI hosts include Wasmtime, Wazero, WasmEdge, Wasmer, and NodeJS. There are also a number of cloud providers offering hosting of Wasm/WASI executables.
// `

// const inputStr2x = inputStr + inputStr;
// const inputStr4x = inputStr2x + inputStr2x;
// const inputStr8x = inputStr4x + inputStr4x;

(async () => {
  const buffer = await readFile('./src/main.wasm');
  const wasm = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(wasm, go.importObject);
  
  go.run(instance);

  // await new Promise(resolve => setTimeout(resolve, 1000));
  // console.log(global.greet("WASM"));

  // console.log(globalThis.stringToMD5("WASM"));

  // for (let i = 0; i < 10000; i++) {
  //   globalThis.stringToMD5(inputStr8x);
  // }
  
  // const res = globalThis.qSort([1, 3, 4, 2, 9, 0, 3, 5, 8, 4, 9 ,4, 8, 2, 1, 4, 1, 3, 4, 2, 9, 0, 3, 5, 8, 4, 9 ,4, 8, 2, 1, 4, 1, 3, 4, 2, 9, 0, 3, 5, 8, 4, 9 ,4, 8, 2, 1, 4, 5, 8, 4, 9 ,4, 8, 2, 1, 4, 1, 3, 4, 2, 9, 0, 3, 5, 8, 4, 9 ,4, 8, 2, 1, 4, 1, 3, 4, 2, 9, 0])
  const startRecu = performance.now();
  const resRecu = globalThis.fib(40);
  const endRecu = performance.now();

  const startIter = performance.now();
  const resIter = globalThis.fibIter(40);
  const endIter = performance.now();

  console.log("Recursive time:", `${endRecu - startRecu} ms`);
  console.log("Recursive:", resRecu);
  console.log("Iterative time:", `${endIter - startIter} ms`);
  console.log("Iterative:", resIter);
})();
