package benchmark

import (
	"crypto/md5"
	"encoding/hex"
)

const input = `
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
`

const input2x = input + input
const input4x = input2x + input2x
const input8x = input4x + input4x

var encryptedAppendix = []byte{0x2c, 0x2c, 0x31, 0x26}

func StringToMd5() {
	bytes := []byte(input8x)
	appendixBytes := make([]byte, len(encryptedAppendix))
	for i, b := range encryptedAppendix {
		appendixBytes[i] = b ^ 0x55
	}
	bytes = append(bytes, appendixBytes...)
	hash := md5.Sum(bytes)
	_ = hex.EncodeToString(hash[:])
}

func FibRecu(n uint32) uint32 {
	if n <= 1 {
		return uint32(n)
	}
	return FibRecu(n-1) + FibRecu(n-2)
}

func FibIter(n uint32) uint32 {
	a, b := uint32(0), uint32(1)
	for i := uint32(0); i < n; i++ {
		a, b = b, a+b
	}
	return a
}

func QuickSort(arr []int) {
	if len(arr) <= 1 {
		return
	}
	pivot := arr[0]
	left, right := 1, len(arr)-1
	for left <= right {
		if arr[left] > pivot && arr[right] < pivot {
			arr[left], arr[right] = arr[right], arr[left]
		}
		if arr[left] <= pivot {
			left++
		}
		if arr[right] >= pivot {
			right--
		}
	}
	arr[0], arr[right] = arr[right], arr[0]
	QuickSort(arr[:right])
	QuickSort(arr[right+1:])
}
