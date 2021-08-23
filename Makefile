OUTPUT=esp32c3.svd
BASE=ESP32C3.svd

all: clean patch generate form fmt build

codegen: clean generate form fmt build

clean:
	rm -rf src/

patch:
	rm -f svd/$(OUTPUT)
	svd patch svd/patches/esp32c3.yaml
	xmllint -format svd/espressif_svd/svd/$(BASE).patched > svd/$(OUTPUT)
	rm  svd/espressif_svd/svd/$(BASE).patched

generate:
	svd2rust --target riscv -i svd/$(OUTPUT)

form:
	form -i lib.rs -o src/
	rm lib.rs

fmt:
	cargo fmt

build:
	cargo clean
	cargo build --target riscv32imc-unknown-none-elf
