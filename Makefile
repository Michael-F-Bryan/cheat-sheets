build:
	mdbook build

rust_interop:
	cd src/Rust && \
		rustc prime.rs -O --crate-type cdylib && \
		gcc main.c -o rust_from_c.exe -L. -lprime && \
		go build -o rust_from_go.exe main.go
	@echo
	@echo "NOTE: Remember to cd to the ./src/Rust/ directory and run"
	@echo "'export LD_LIBRARY_PATH=.' before trying any of the binaries"
