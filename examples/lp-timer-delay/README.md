# lp-timer-delay

Bare-metal blocking delay example using the hardware timer for the **E902 (LP) core**.

Utilizes `TIMER0` Channel 2 to create precise timing intervals for toggling the onboard LED.

---

## Peripheral Usage

- **GLB**: Pin configuration for GPIO8.
- **TIMER0**: Channel 2 for 32-bit counter readback and overflow management.

---

## Building

```sh
cargo build --release -p lp-timer-delay
```
