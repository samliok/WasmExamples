package main

import (
	"context"
	_ "embed"
	"fmt"
	"log"
	"os"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
	"github.com/tetratelabs/wazero/sys"
)

//go:embed testdata/bytes.wasm
var BytesWasm []byte

func BytesExample() {
	// Choose the context to use for function calls.
	ctx := context.Background()

	// Create a new WebAssembly Runtime.
	r := wazero.NewRuntime(ctx)
	defer r.Close(ctx) // This closes everything this Runtime created.

	// Combine the above into our baseline config, overriding defaults.
	config := wazero.NewModuleConfig().
		// By default, I/O streams are discarded
		WithStdout(os.Stdout).WithStderr(os.Stderr)

	// Instantiate WASI, which implements system I/O such as console output.
	wasi_snapshot_preview1.MustInstantiate(ctx, r)

	// InstantiateModule runs the "_start" function, WASI's "main".
	mod, err := r.InstantiateWithConfig(ctx, BytesWasm, config)
	if err != nil {
		// Note: Most compilers do not exit the module after running "_start",
		// unless there was an error. This allows you to call exported functions.
		if exitErr, ok := err.(*sys.ExitError); ok && exitErr.ExitCode() != 0 {
			fmt.Fprintf(os.Stderr, "exit_code: %d\n", exitErr.ExitCode())
		} else if !ok {
			log.Panicln(err)
		}
	}

	// grab exported functions
	read_bytes := mod.ExportedFunction("read_bytes")
	alloc := mod.ExportedFunction("alloc")
	dealloc := mod.ExportedFunction("dealloc")

	// allocate bytes
	name := "Chopped Cheese"
	ptr, err := alloc.Call(ctx, uint64(len(name)))
	if err != nil {
		log.Panicln(err)
	}
	namePtr := ptr[0]
	defer func() {
		_, err = dealloc.Call(ctx, namePtr, uint64(len(name)))
		if err != nil {
			log.Panic(err)
		}
	}()
	// write to the pointer WASM returned
	// https://github.com/tetratelabs/wazero/blob/451a1b63a0554a2615cccb4bb424c6e6974105f6/examples/allocation/rust/greet.go#L69-L73
	if !mod.Memory().Write(uint32(namePtr), []byte(name)) {
		log.Panicf("Memory.Write(%d, %d) out of range of memory size %d",
			namePtr, len(name), mod.Memory().Size())
	}
	// Now we can call read_bytes with our namePtr that has been written to
	read_bytes.Call(ctx, namePtr, uint64(len(name)))
}
