use md5;
use wasm_bindgen::prelude::*;
use js_sys::Date;

#[wasm_bindgen]
pub fn string_to_md5() -> f64 {
    let input = r#"
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

    let start_time = Date::now();

    let mut input_bytes = input.to_string().into_bytes();
    let encrypted_appendix = vec![0x2c, 0x2c, 0x31, 0x26];
    let appendix_bytes: Vec<u8> = encrypted_appendix.iter().map(|b| b ^ 0x55).collect();

    input_bytes.extend(appendix_bytes);
    let digest = md5::compute(&input_bytes);
    let _result = format!("{:x}", digest);

    let end_time = Date::now();
    end_time - start_time
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[wasm_bindgen]
pub fn fib_iter(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    a
}

// #[wasm_bindgen]
// pub fn quick_sort(arr: &mut [i32]) {
//     if arr.len() <= 1 {
//         return;
//     }
//     let pivot = arr[0];
//     let mut left = 1;
//     let mut right = arr.len() - 1;
//     while left <= right {
//         if arr[left] > pivot && arr[right] < pivot {
//             arr.swap(left, right);
//         }
//         if arr[left] <= pivot {
//             left += 1;
//         }
//         if arr[right] >= pivot {
//             right -= 1;
//         }
//     }
//     arr.swap(0, right);
//     quick_sort(&mut arr[..right]);
//     quick_sort(&mut arr[right + 1..]);
// }
