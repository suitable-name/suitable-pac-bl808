# lp-blinky

Bare-metal blink example for the **Sipeed M1s Dock** running on the **E902 (LP) core** of the BL808 SoC.

The onboard white LED (GPIO8, active-low) blinks at roughly 1 Hz.

---

## Hardware

| Item | Detail |
|------|--------|
| Board | Sipeed M1s Dock |
| Core | T-Head E902 (LP), RV32I, up to 150 MHz |
| LED | GPIO8, active-low (3.3V → R18 510Ω → LED → GPIO8) |
| UART (E902) | IO14 TX / IO15 RX, 2 000 000 baud |
| Flash | 16 MB NOR, XIP-mapped at 0x5800\_0000 |
| LP SRAM | 0x2300\_0000, 16 KB |

---

## Building

From the workspace root:

```sh
cargo build --release -p lp-blinky
```

---

## Memory map

Defined in `memory.x`:

```
FLASH : ORIGIN = 0x58000000, LENGTH = 16M
RAM   : ORIGIN = 0x23000000, LENGTH = 16K
```
