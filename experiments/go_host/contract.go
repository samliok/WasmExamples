package main

import (
	"context"
	_ "embed"
	"fmt"
	"log"
	"os"
	"vector"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
	"google.golang.org/protobuf/proto"
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

	// //  setup env
	// _, err := r.NewHostModuleBuilder("env").
	// 	NewFunctionBuilder().WithFunc(getAliceKey).Export("get_alice_key").
	// 	Instantiate(ctx)
	// if err != nil {
	// 	log.Panicf("failed to create new host: %v", err)
	// }

	// Combine the above into our baseline config, overriding defaults.
	config := wazero.NewModuleConfig().
		// By default, I/O streams are discarded
		WithStdout(os.Stdout).WithStderr(os.Stderr)

	// Instantiate WASI, which implements system I/O such as console output.
	wasi_snapshot_preview1.MustInstantiate(ctx, r)

	mod, err := r.InstantiateWithConfig(ctx, contractWasm, config)
	if err != nil {
		log.Panicf("failed to instantiate module: %v", err)
	}

	intVector := &vector.IntVector{
		Values: []int32{1, 2, 3, 4, 5},
	}

	// Serialize the message
	data, err := proto.Marshal(intVector)
	if err != nil {
		log.Fatal("Failed to serialize IntVector:", err)
	}

	// Print the serialized data
	// fmt.Println(data)

	get_bal := mod.ExportedFunction("get_balance")
	result, err := get_bal.Call(ctx)
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println("Alice balance is: ", result)
	// fmt.Println("Alice pubkey is: ", getAliceKey())
}

// type pubkey [32]byte

// func getAliceKey() uint64 {
// 	return [32]byte{}
// }
