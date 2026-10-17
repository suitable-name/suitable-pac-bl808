MEMORY
{
  /* D0 (C906) firmware is copied by M0 into PSRAM via Boot2.
   * boot_cpu0.c: C906_START_ADDR = 0x50100000.
   * Partition D0FW: flash offset 0x100000, size 2MB.
   * Code and rodata live in PSRAM (already placed there by M0 before D0 starts).
   * Stack, data, and BSS live in D0's local SRAM. */
  PSRAM : ORIGIN = 0x50100000, LENGTH = 2M
  /* D0 local SRAM: dtcm(64K) + ram_memory(64K) + itcm(64K) = 192K
   * 0x3EFD0000..0x3F000000 */
  SRAM  : ORIGIN = 0x3EFD0000, LENGTH = 192K
}

REGION_ALIAS("REGION_TEXT",   PSRAM);
REGION_ALIAS("REGION_RODATA", PSRAM);
REGION_ALIAS("REGION_DATA",   SRAM);
REGION_ALIAS("REGION_BSS",    SRAM);
REGION_ALIAS("REGION_HEAP",   SRAM);
REGION_ALIAS("REGION_STACK",  SRAM);

_stack_start = ORIGIN(SRAM) + LENGTH(SRAM);
