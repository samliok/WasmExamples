// used https://github.com/tetratelabs/wazero/blob/main/imports/wasi_snapshot_preview1/example/README.md
// for reference
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

//go:embed testdata/hello.wasm
var helloWasm []byte

// WasiSysCall shows an example of using WASI system calls to print to IO(must be passed in using wazero)
func WasiSysCall() {
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
	if _, err := r.InstantiateWithConfig(ctx, helloWasm, config); err != nil {
		// Note: Most compilers do not exit the module after running "_start",
		// unless there was an error. This allows you to call exported functions.
		if exitErr, ok := err.(*sys.ExitError); ok && exitErr.ExitCode() != 0 {
			fmt.Fprintf(os.Stderr, "exit_code: %d\n", exitErr.ExitCode())
		} else if !ok {
			log.Panicln(err)
		}
	}
}
