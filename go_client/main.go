package main

import (
	"bytes"
	"context"
	_ "embed"
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/tetratelabs/wazero"
)

//go:embed wasm_validation_nojs.wasm
var validationWasm []byte

func validate(payload string) (*string, error) {
	ctx := context.Background()
	r := wazero.NewRuntime(ctx)
	defer r.Close(ctx) // This closes everything this Runtime created.

	mod, err := r.Instantiate(ctx, validationWasm)
	if err != nil {
		return nil, err
	}
	validate := mod.ExportedFunction("validate_create_host_params")
	allocate := mod.ExportedFunction("allocate")
	deallocate := mod.ExportedFunction("deallocate")

	payloadSize := uint64(len(payload))

	results, err := allocate.Call(ctx, payloadSize)
	if err != nil {
		return nil, err
	}
	payloadPtr := results[0]
	defer deallocate.Call(ctx, payloadPtr, payloadSize)

	if !mod.Memory().Write(uint32(payloadPtr), []byte(payload)) {
		return nil, fmt.Errorf("Memory.Write(%d, %d) out of range of memory size %d", payloadPtr, payloadSize, mod.Memory().Size())
	}
	ptrSize, err := validate.Call(ctx, payloadPtr, payloadSize)
	if err != nil {
		return nil, err
	}
	resultPtr := uint32(ptrSize[0] >> 32)
	resultSize := uint32(ptrSize[0])
	defer deallocate.Call(ctx, uint64(resultPtr), uint64(resultSize))

	if bytes, ok := mod.Memory().Read(resultPtr, resultSize); !ok {
		return nil, fmt.Errorf("Memory.Read(%d, %d) out of range of memory size %d",
			resultPtr, resultSize, mod.Memory().Size())
	} else {
		result := string(bytes)
		return &result, nil
	}
}

func main() {
	res, err := validate("{}")
	if err != nil {
		log.Panicf("validate paniced with %v", err)
	}
	log.Printf("Validation result for {}: %s", *res)

	localhostPayload := `{
		"hostname": "localhost",
		"ipv4": "192.168.13.37"
	}`
	res, err = validate(localhostPayload)
	if err != nil {
		log.Panicf("validate paniced with %v", err)
	}
	log.Printf("Validation result for payload with localhost: %s", *res)

	fullPayload := `{
		"hostname": "foobar",
		"ipv4": "192.168.13.37"
	}`
	res, err = validate(fullPayload)
	if err != nil {
		log.Panicf("validate paniced with %v", err)
	}
	log.Printf("Validation result for complete payload: %s", *res)

	buf := bytes.NewBufferString(fullPayload)
	resp, err := http.Post("http://127.0.0.1:3000/hosts", "application/json", buf)
	if err != nil {
		log.Panicf("could not send http request %v", err)
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	log.Printf("Rust Backend POST request completed with status: %d - body %s", resp.StatusCode, body)
}
