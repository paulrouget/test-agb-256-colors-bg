run:
	cargo run --target=thumbv4t-none-eabi -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
