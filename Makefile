all: build run open
build: build-skylake

build-general:
	cargo build --release

build-skylake:
	RUSTFLAGS="-Ctarget-cpu=skylake" cargo build --release

gen:
	./util.sh gen

run:
	./target/release/yarr

run-debug:
	./target/debug/yarr

open:
	feh --auto-zoom --force-aliasing out/image.ppm

clean:
	rm -rf target

.PHONY: build build-general build-skylake gen run run-debug open clean
