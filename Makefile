all: build

INSTALL_FOLDER := /usr/local/bin

build:
	cargo build --release
	strip ./target/release/gpurun

install:
	install -D -m 0755 ./target/release/gpurun $(INSTALL_FOLDER)