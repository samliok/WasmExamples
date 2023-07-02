package main

import (
	"context"
	_ "embed"
	"log"
	"os"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

//go:embed testdata/contract.wasm
var contractWasm []byte

// ExportImportExample calls  two function exported from WASM
// and exports a Power function to WASM.
func ContractExample() {
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

	// Instantiate the guest Wasm into the same runtime. It exports the `add`
	// function, implemented in WebAssembly.
	// _, err := r.Instantiate(ctx, contract)
	_, err := r.InstantiateWithConfig(ctx, contractWasm, config)

	if err != nil {
		log.Panicf("failed to instantiate module: %v", err)
	}

}
