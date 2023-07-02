package main

func main() {
	// ExportImportExample()
	// WasiSysCall()
	// BytesExample()
	// BytesProfiler()
	// TestSimple()
	ContractExample()
}

// type A struct {
// 	supply  uint64
// 	name    string
// 	balance map[string]uint64
// }

// func TestSimple() {
// 	x := A{
// 		supply:  3301,
// 		name:    "liber primus",
// 		balance: map[string]uint64{},
// 	}
// 	data, err := borsh.Serialize(x)
// 	log.Print(data)
// 	if err != nil {
// 		fmt.Println(err)
// 	}
// 	y := new(A)
// 	err = borsh.Deserialize(y, data)
// 	if err != nil {
// 		fmt.Println(err)
// 	}
// 	if !reflect.DeepEqual(x, *y) {
// 		fmt.Println(err)
// 	}
// }
