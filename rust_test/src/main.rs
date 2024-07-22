use md5;
use std::time::Instant;

const INPUT: &str = r#"
WASI support in Go
Johan Brandhorst-Satzkorn, Julien Fabre, Damian Gryski, Evan Phoenix, and Achille Roussel
13 September 2023

Go 1.21 adds a new port targeting the WASI preview 1 syscall API through the new GOOS value wasip1. This port builds on the existing WebAssembly port introduced in Go 1.11.

What is WebAssembly?
WebAssembly (Wasm) is a binary instruction format originally designed for the web. It represents a standard that allows developers to run high-performance, low-level code directly in web browsers at near-native speeds.

Go first added support for compiling to Wasm in the 1.11 release, through the js/wasm port. This allowed Go code compiled using the Go compiler to be executed in web browsers, but it required a JavaScript execution environment.

As the use of Wasm has grown, so have use cases outside of the browser. Many cloud providers are now offering services that allow the user to execute Wasm executables directly, leveraging the new WebAssembly System Interface (WASI) syscall API.

The WebAssembly System Interface
WASI defines a syscall API for Wasm executables, allowing them to interact with system resources such as the filesystem, the system clock, random data utilities, and more. The latest release of the WASI spec is called wasi_snapshot_preview1, from which we derive the GOOS name wasip1. New versions of the API are being developed, and supporting them in the Go compiler in the future will likely mean adding a new GOOS.

The creation of WASI has allowed a number of Wasm runtimes (hosts) to standardize their syscall API around it. Examples of Wasm/WASI hosts include Wasmtime, Wazero, WasmEdge, Wasmer, and NodeJS. There are also a number of cloud providers offering hosting of Wasm/WASI executables.
"#;

fn repeat_string(s: &str, n: usize) -> String {
    s.repeat(n)
}

fn string_to_md5(input: &str) {
    let mut input_bytes = input.as_bytes().to_vec();
    let encrypted_appendix = [0x2c, 0x2c, 0x31, 0x26];
    let appendix_bytes: Vec<u8> = encrypted_appendix.iter().map(|&b| b ^ 0x55).collect();
    input_bytes.extend_from_slice(&appendix_bytes);
    let digest = md5::compute(&input_bytes);
    let _result = format!("{:x}", digest);
}

fn fib_recu(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        return fib_recu(n - 1) + fib_recu(n - 2);
    }
}

fn fib_iter(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    return a;
}

fn main() {
    // let input_2x = repeat_string(INPUT, 2);
    // let input_4x = repeat_string(&input_2x, 2);
    // let input_8x = repeat_string(&input_4x, 2);

    // let start = Instant::now();

    // for _ in 0..10000 {
    //     string_to_md5(INPUT);
    // }

    // let duration = start.elapsed();

    // println!("总执行时间: {:?}", duration);

    let fib_recu_start = Instant::now();
    let fib_recu_res = fib_recu(40);
    let fib_recu_duration = fib_recu_start.elapsed();
    println!("递归执行时间: {:?}", fib_recu_duration);
    println!("递归执行结果: {}", fib_recu_res);

    let fib_iter_start = Instant::now();
    let fib_iter_res = fib_iter(40);
    let fib_iter_duration = fib_iter_start.elapsed();
    println!("迭代执行执行时间: {:?}", fib_iter_duration);
    println!("迭代执行结果: {}", fib_iter_res);
}