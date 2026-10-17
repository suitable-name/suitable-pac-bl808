# m0-blinky

Bare-metal blink example for the **Sipeed M1s Dock** running on the **E907 (M0) core** of the BL808 SoC.

The onboard white LED (GPIO8, active-low) blinks at roughly 1 Hz.

---

## Hardware

| Item | Detail |
|------|--------|
| Board | Sipeed M1s Dock |
| Core | T-Head E907 (M0), RV32IMAFC, up to 320 MHz |
| LED | GPIO8, active-low (3.3V → R18 510Ω → LED → GPIO8) |
| UART (E907) | IO14 TX / IO15 RX, 2 000 000 baud |
| Flash | 16 MB NOR, XIP-mapped at 0x5800\_0000 |
| M0 SRAM | 0x2202\_0000, 128 KB |

---

## Prerequisites

### Rust toolchain

```sh
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the bare-metal RISC-V target
rustup target add riscv32imac-unknown-none-elf

# cargo-binutils for ELF → binary conversion
cargo install cargo-binutils
rustup component add llvm-tools
```

### Flash tool

Install **Bouffalo Lab Dev Cube** (BLDevCube) from:
<https://dev.bouffalolab.com/download>

Extract the archive. On Linux, the binary is `BLDevCube-ubuntu`.

> **Linux note:** You may need to add your user to the `dialout` group to
> access the serial port without `sudo`:
> ```sh
> sudo usermod -aG dialout $USER   # log out and back in afterwards
> ```

### Partition table

The M1s Dock partition layout is provided by Sipeed's example repository.
Clone it once to get the partition table TOML:

```sh
git clone https://github.com/sipeed/M1s_BL808_example.git
# Partition file: M1s_BL808_example/partition/partition_cfg_16M.toml
```

---

## Building

From the workspace root:

```sh
cargo build --release -p m0-blinky
```

Convert the ELF to a raw binary that BLDevCube expects:

```sh
rust-objcopy -O binary \
  target/riscv32imac-unknown-none-elf/release/m0-blinky \
  target/riscv32imac-unknown-none-elf/release/m0-blinky.bin
```

---

## Flash layout

The M1s Dock uses the following default partition table (16 MB flash):

| Partition | Flash offset | XIP address | Size | Contents |
|-----------|-------------|-------------|------|----------|
| Boot2 | 0x0000\_0000 | 0x5800\_0000 | 56 KB | 2nd-stage bootloader (bundled in BLDevCube) |
| **FW** | **0x0001\_0000** | **0x5801\_0000** | **960 KB** | **E907 / M0 firmware ← flash here** |
| D0FW | 0x0010\_0000 | 0x580F\_0000 | 2 MB | C906 / D0 firmware |
| media | 0x0030\_0000 | — | 12 MB | Filesystem / assets |

This example only replaces the **FW** partition. The other partitions
(Boot2, D0FW) remain unchanged if you choose not to flash them.

---

## Flashing via UART (BLDevCube)

The UART TypeC port (left side of board when facing the screen) programs the
BL808 directly. The OTG TypeC port is for D0 firmware only and cannot
flash E907 firmware.

### 1. Enter download mode

1. Connect the **UART TypeC port** to your PC.
2. Hold **BOOT**, press and release **RST**, then release **BOOT**.
3. Two serial ports appear on the host (e.g. `/dev/ttyUSB0` and
   `/dev/ttyUSB1` on Linux, or two `COMx` on Windows).
   Use the **higher-numbered** port.

### 2. Configure BLDevCube

1. Launch BLDevCube, choose chip **BL808**, click **Finish**.
2. Select the **IOT** tab.
3. Fill in the fields:

| Field | Value |
|-------|-------|
| Partition Table | `M1s_BL808_example/partition/partition_cfg_16M.toml` |
| Boot2 | `<BLDevCube>/chips/bl808/builtin_imgs/boot2_isp_bl808_*/boot2_isp_debug.bin` |
| firmware (check the box) | `target/riscv32imac-unknown-none-elf/release/m0-blinky.bin` |
| firmware Address | `0x10000` |

> **First flash only:** tick *partition table*, *Boot2*, and *firmware*.
> On subsequent flashes you only need to tick *firmware*.
> Leave *d0fw* unticked — this example does not touch the D0 core.

4. Click **Refresh** (serial port selector, top-right), pick the higher-numbered port.
5. Set baud rate to **2000000**.
6. Click **Create & Download**.

### 3. Verify

After flashing completes, press **RST**. The onboard white LED should blink
at approximately 1 Hz.

---

## Flashing via UART (command-line — Linux)

As an alternative to BLDevCube, [BLFlashCommand](https://github.com/bouffalolab/bouffalo_sdk)
can be used on Linux. Create `flash_prog_cfg.ini` according to the tool's
documentation, then:

```sh
BLFlashCommand --interface uart --port /dev/ttyUSB1 --baudrate 2000000 \
  --chipname bl808 --config flash_prog_cfg.ini
```

---

## Serial debug output

This bare-metal blinky does not print anything over UART. If you add debug
prints later using the E907 UART, connect a 3.3 V serial adapter to:

| Signal | Pin |
|--------|-----|
| TX (E907 → PC) | IO14 |
| RX (PC → E907) | IO15 |
| GND | GND |

Baud rate: **2 000 000**.

Open the **smaller**-numbered serial port from the UART TypeC connection at
the same baud rate to see bootloader output on reset.

---

## Memory map

Defined in `memory.x`:

```
FLASH : ORIGIN = 0x58000000, LENGTH = 16M   /* XIP flash, Boot2-remapped */
RAM   : ORIGIN = 0x22020000, LENGTH = 128K  /* M0 OCRAM (SHRAM0 + SHRAM1) */
```

Boot2 configures the XIP controller so that the E907 code segment executes
from 0x5800\_0000, matching the linker origin. `.data` and `.bss` live in RAM;
`.text` and `.rodata` execute directly from XIP flash.

---

## Project layout

```
examples/m0-blinky/
├── .cargo/
│   └── config.toml      # default target + linker flags + RISCV_RT_BASE_ISA
├── src/
│   └── main.rs          # blinky application
├── build.rs             # copies memory.x to OUT_DIR for the linker
├── memory.x             # flash and RAM regions
└── Cargo.toml
```

---

## Troubleshooting

**`error: couldn't read .../device.x`**
Run `make all` from the workspace root first to generate the PAC sources,
including `crates/bl808-m0/device.x` (the interrupt vector stubs).

**Board not detected / only one serial port**
The onboard BL702 USB-to-UART bridge may need its firmware reflashed.
See the [Sipeed wiki](https://wiki.sipeed.com/hardware/en/maix/m1s/other/start.html)
for instructions on reflashing the BL702.

**`Permission denied` on `/dev/ttyUSBx`**
```sh
sudo usermod -aG dialout $USER
# Log out and back in, then retry.
```

**LED does not blink after flashing**
- Confirm you flashed to address **0x10000** (FW partition), not 0x0.
- Check you selected *Boot2* on the first flash.
- Press RST after BLDevCube reports success.
