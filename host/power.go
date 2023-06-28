package main

import (
	"context"
	_ "embed"
	"fmt"
	"log"

	"github.com/tetratelabs/wazero"
)

//go:embed testdata/power.wasm
var powerAdd []byte

// ExportImportExample calls  two function exported from WASM
// and exports a Power function to WASM.
func ExportImportExample() {
	// Choose the context to use for function calls.
	ctx := context.Background()

	// Create a new WebAssembly Runtime.
	r := wazero.NewRuntime(ctx)
	defer r.Close(ctx) // This closes everything this Runtime created.

	_, err := r.NewHostModuleBuilder("env").
		NewFunctionBuilder().WithFunc(Power).Export("power").
		Instantiate(ctx)
	if err != nil {
		log.Panicf("failed to create new host: %v", err)
	}

	// Instantiate the guest Wasm into the same runtime. It exports the `add`
	// function, implemented in WebAssembly.
	mod, err := r.Instantiate(ctx, powerAdd)
	if err != nil {
		log.Panicf("failed to instantiate module: %v", err)
	}

	// Call the `add` function and print the results to the console.
	add := mod.ExportedFunction("add")
	double_power := mod.ExportedFunction("double_power")
	results, err := add.Call(ctx, 9, 4)
	if err != nil {
		log.Panicf("failed to call add: %v", err)
	}
	fmt.Printf("Adding 9 + 4: : %d\n", results[0])

	results, err = double_power.Call(ctx, 10)
	if err != nil {
		log.Panicf("failed to call power: %v", err)
	}
	fmt.Printf("Double Power of 10: %d\n", results[0])
}

func Power(x int32) int32 {
	return x * x
}
