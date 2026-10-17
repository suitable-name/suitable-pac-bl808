# ==============================================================================
# BL808 Multi-Core Workspace PAC Generation Makefile
#
# Delegates generation to cross-platform scripts/generate_pac.py
# Cores:
#   - M0 (E907, 32-bit RISC-V)
#   - D0 (C906, 64-bit RISC-V)
#   - LP (E902, 32-bit RISC-V)
# ==============================================================================

PYTHON ?= python3

.PHONY: all m0 d0 lp clean check help

all:
	$(PYTHON) scripts/generate_pac.py --core all

m0:
	$(PYTHON) scripts/generate_pac.py --core m0

d0:
	$(PYTHON) scripts/generate_pac.py --core d0

lp:
	$(PYTHON) scripts/generate_pac.py --core lp

check:
	cargo check -p bl808-m0
	cargo check -p bl808-d0
	cargo check -p bl808-lp
	cargo check -p bl808-pac --all-features

clean:
	rm -rf crates/bl808-m0/src/* crates/bl808-m0/device.x
	rm -rf crates/bl808-d0/src/* crates/bl808-d0/device.x
	rm -rf crates/bl808-lp/src/* crates/bl808-lp/device.x
	rm -f *.svd lib.rs device.x build.rs

help:
	@echo "BL808 PAC Generator"
	@echo "Usage:"
	@echo "  make all    - Clean and build PACs for all three cores (M0, D0, LP)"
	@echo "  make m0     - Build only the M0 PAC"
	@echo "  make d0     - Build only the D0 PAC"
	@echo "  make lp     - Build only the LP PAC"
	@echo "  make check  - Verify compilation of all PAC crates"
	@echo "  make clean  - Remove all generated code and artifacts"
