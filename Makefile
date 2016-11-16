BUCKET = www.michaelfbryan.com


build: src/Rust/libprime.so src/Rust/rust_from_c.exe src/Rust/rust_from_go.exe
	mdbook build

rust_interop:
	cd src/Rust && \
		rustc prime.rs -O --crate-type cdylib && \
		gcc main.c -o rust_from_c.exe -L. -lprime && \
		go build -o rust_from_go.exe main.go
	@echo
	@echo "NOTE: Remember to cd to the ./src/Rust/ directory and run"
	@echo "'export LD_LIBRARY_PATH=.' before trying any of the binaries"

src/Rust/libprime.so:
		cd src/Rust && rustc prime.rs -O --crate-type cdylib

src/Rust/rust_from_c.exe:
		cd src/Rust && gcc main.c -o rust_from_c.exe -L. -lprime 

src/Rust/rust_from_go.exe:
		cd src/Rust && go build -o rust_from_go.exe main.go

clean:
	$(RM) src/Rust/libprime.so
	$(RM) src/Rust/rust_from_c.exe
	$(RM) src/Rust/rust_from_go.exe
	$(RM) -r book

publish: build
	aws s3 sync book s3://$(BUCKET)/

.PHONY: publish clean rust_interop
