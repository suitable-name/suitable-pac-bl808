# bl808-pac

Universal Peripheral Access Crate (PAC) for the Bouffalo Lab BL808 heterogeneous tri-core SoC:
- **Core M0**: 32-bit RISC-V T-Head E907 (MCU core)
- **Core D0**: 64-bit RISC-V T-Head C906 (Multimedia / Application core)
- **Core LP**: 32-bit RISC-V T-Head E902 (Low-Power sensor core)

## Crates in this Workspace

- [`bl808-pac`](src/lib.rs): Unified wrapper crate with feature gates (`m0`, `d0`, `lp`, `rt`, `critical-section`).
- [`bl808-m0`](crates/bl808-m0): Dedicated PAC for Core M0 (`riscv32imac-unknown-none-elf`).
- [`bl808-d0`](crates/bl808-d0): Dedicated PAC for Core D0 (`riscv64imac-unknown-none-elf`).
- [`bl808-lp`](crates/bl808-lp): Dedicated PAC for Core LP (`riscv32imac-unknown-none-elf`).

## Building Examples

### Core M0 PAC Test
```powershell
cargo build -p m0-pac-test --target riscv32imac-unknown-none-elf --release
```

### Core D0 PAC Test
```powershell
cargo build -p d0-pac-test --target riscv64imac-unknown-none-elf --release
```

## Live Hardware Verification on Sipeed M1s Dock

To validate all 43 peripheral blocks on physical silicon via the interactive BL808 console UART:

```powershell
pwsh -File scripts/test_pac_live.ps1 -Port COM8
```

The script verifies:
1. Base addresses and register offsets for every peripheral.
2. Read signatures and non-destructive register modifications.
3. Hardware timer counters and eFuse Wi-Fi MAC readout.
4. Generates an automated test report.
