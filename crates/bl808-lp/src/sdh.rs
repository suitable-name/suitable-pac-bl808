#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sd_sys_addr_low: SdSysAddrLow,
    sd_sys_addr_high: SdSysAddrHigh,
    sd_block_size: SdBlockSize,
    sd_block_count: SdBlockCount,
    sd_arg_low: SdArgLow,
    sd_arg_high: SdArgHigh,
    sd_transfer_mode: SdTransferMode,
    sd_cmd: SdCmd,
    sd_resp_0: SdResp0,
    sd_resp_1: SdResp1,
    sd_resp_2: SdResp2,
    sd_resp_3: SdResp3,
    sd_resp_4: SdResp4,
    sd_resp_5: SdResp5,
    sd_resp_6: SdResp6,
    sd_resp_7: SdResp7,
    sd_buffer_data_port_0: SdBufferDataPort0,
    sd_buffer_data_port_1: SdBufferDataPort1,
    sd_present_state_1: SdPresentState1,
    sd_present_state_2: SdPresentState2,
    sd_host_ctrl: SdHostCtrl,
    sd_block_gap_ctrl: SdBlockGapCtrl,
    sd_clock_ctrl: SdClockCtrl,
    sd_timeout_ctrl_sw_reset: SdTimeoutCtrlSwReset,
    sd_normal_int_status: SdNormalIntStatus,
    sd_error_int_status: SdErrorIntStatus,
    sd_normal_int_status_en: SdNormalIntStatusEn,
    sd_error_int_status_en: SdErrorIntStatusEn,
    sd_normal_int_status_int_en: SdNormalIntStatusIntEn,
    sd_error_int_status_int_en: SdErrorIntStatusIntEn,
    sd_auto_cmd12_error_status: SdAutoCmd12ErrorStatus,
    host_ctrl_2: HostCtrl2,
    sd_capabilities_1: SdCapabilities1,
    sd_capabilities_2: SdCapabilities2,
    sd_capabilities_3: SdCapabilities3,
    sd_capabilities_4: SdCapabilities4,
    sd_max_current_1: SdMaxCurrent1,
    sd_max_current_2: SdMaxCurrent2,
    sd_max_current_3: SdMaxCurrent3,
    sd_max_current_4: SdMaxCurrent4,
    sd_force_event_auto_cmd12_error: SdForceEventAutoCmd12Error,
    sd_force_event_for_error_status: SdForceEventForErrorStatus,
    sd_adma_error_status: SdAdmaErrorStatus,
    _reserved43: [u8; 0x02],
    sd_adma_sys_addr_1: SdAdmaSysAddr1,
    sd_adma_sys_addr_2: SdAdmaSysAddr2,
    sd_adma_sys_addr_3: SdAdmaSysAddr3,
    sd_adma_sys_addr_4: SdAdmaSysAddr4,
    preset_value_for_init: PresetValueForInit,
    preset_value_for_ds: PresetValueForDs,
    preset_value_for_hs: PresetValueForHs,
    preset_value_for_sdr12: PresetValueForSdr12,
    preset_value_for_sdr25: PresetValueForSdr25,
    preset_value_for_sdr50: PresetValueForSdr50,
    preset_value_for_sdr104: PresetValueForSdr104,
    preset_value_for_ddr50: PresetValueForDdr50,
    _reserved55: [u8; 0x70],
    shared_bus_ctrl: SharedBusCtrl,
    _reserved56: [u8; 0x18],
    sd_slot_int_status: SdSlotIntStatus,
    sd_host_ctrl_ver: SdHostCtrlVer,
    sd_cfg_fifo_param: SdCfgFifoParam,
    sd_fifo_param: SdFifoParam,
    sd_spi_mode: SdSpiMode,
    sd_clock_and_burst_size_setup: SdClockAndBurstSizeSetup,
    sd_ce_ata_1: SdCeAta1,
    sd_ce_ata_2: SdCeAta2,
    sd_pad_io_setup: SdPadIoSetup,
    rx_cfg_reg: RxCfgReg,
    tx_cfg_reg: TxCfgReg,
    tuning_cfg_reg: TuningCfgReg,
}
impl RegisterBlock {
    #[doc = "0x00 - System Address Low Register (16-bit part of SDMA/ADMA address)"]
    #[inline(always)]
    pub const fn sd_sys_addr_low(&self) -> &SdSysAddrLow {
        &self.sd_sys_addr_low
    }
    #[doc = "0x02 - System Address High Register (16-bit part of SDMA/ADMA address)"]
    #[inline(always)]
    pub const fn sd_sys_addr_high(&self) -> &SdSysAddrHigh {
        &self.sd_sys_addr_high
    }
    #[doc = "0x04 - Block Size Register"]
    #[inline(always)]
    pub const fn sd_block_size(&self) -> &SdBlockSize {
        &self.sd_block_size
    }
    #[doc = "0x06 - Block Count Register"]
    #[inline(always)]
    pub const fn sd_block_count(&self) -> &SdBlockCount {
        &self.sd_block_count
    }
    #[doc = "0x08 - Command Argument Low Register (Bits 15:0)"]
    #[inline(always)]
    pub const fn sd_arg_low(&self) -> &SdArgLow {
        &self.sd_arg_low
    }
    #[doc = "0x0a - Command Argument High Register (Bits 31:16)"]
    #[inline(always)]
    pub const fn sd_arg_high(&self) -> &SdArgHigh {
        &self.sd_arg_high
    }
    #[doc = "0x0c - Transfer Mode Register"]
    #[inline(always)]
    pub const fn sd_transfer_mode(&self) -> &SdTransferMode {
        &self.sd_transfer_mode
    }
    #[doc = "0x0e - Command Register"]
    #[inline(always)]
    pub const fn sd_cmd(&self) -> &SdCmd {
        &self.sd_cmd
    }
    #[doc = "0x10 - Response Register 0 (Bits 15:0)"]
    #[inline(always)]
    pub const fn sd_resp_0(&self) -> &SdResp0 {
        &self.sd_resp_0
    }
    #[doc = "0x12 - Response Register 1 (Bits 31:16)"]
    #[inline(always)]
    pub const fn sd_resp_1(&self) -> &SdResp1 {
        &self.sd_resp_1
    }
    #[doc = "0x14 - Response Register 2 (Bits 47:32)"]
    #[inline(always)]
    pub const fn sd_resp_2(&self) -> &SdResp2 {
        &self.sd_resp_2
    }
    #[doc = "0x16 - Response Register 3 (Bits 63:48)"]
    #[inline(always)]
    pub const fn sd_resp_3(&self) -> &SdResp3 {
        &self.sd_resp_3
    }
    #[doc = "0x18 - Response Register 4 (Bits 79:64)"]
    #[inline(always)]
    pub const fn sd_resp_4(&self) -> &SdResp4 {
        &self.sd_resp_4
    }
    #[doc = "0x1a - Response Register 5 (Bits 95:80)"]
    #[inline(always)]
    pub const fn sd_resp_5(&self) -> &SdResp5 {
        &self.sd_resp_5
    }
    #[doc = "0x1c - Response Register 6 (Bits 111:96)"]
    #[inline(always)]
    pub const fn sd_resp_6(&self) -> &SdResp6 {
        &self.sd_resp_6
    }
    #[doc = "0x1e - Response Register 7 (Bits 127:112)"]
    #[inline(always)]
    pub const fn sd_resp_7(&self) -> &SdResp7 {
        &self.sd_resp_7
    }
    #[doc = "0x20 - Buffer Data Port 0 (16 bits)"]
    #[inline(always)]
    pub const fn sd_buffer_data_port_0(&self) -> &SdBufferDataPort0 {
        &self.sd_buffer_data_port_0
    }
    #[doc = "0x22 - Buffer Data Port 1 (16 bits)"]
    #[inline(always)]
    pub const fn sd_buffer_data_port_1(&self) -> &SdBufferDataPort1 {
        &self.sd_buffer_data_port_1
    }
    #[doc = "0x24 - Present State Register 1"]
    #[inline(always)]
    pub const fn sd_present_state_1(&self) -> &SdPresentState1 {
        &self.sd_present_state_1
    }
    #[doc = "0x26 - Present State Register 2 (Card Detection)"]
    #[inline(always)]
    pub const fn sd_present_state_2(&self) -> &SdPresentState2 {
        &self.sd_present_state_2
    }
    #[doc = "0x28 - Host Control Register 1"]
    #[inline(always)]
    pub const fn sd_host_ctrl(&self) -> &SdHostCtrl {
        &self.sd_host_ctrl
    }
    #[doc = "0x2a - Block Gap Control Register"]
    #[inline(always)]
    pub const fn sd_block_gap_ctrl(&self) -> &SdBlockGapCtrl {
        &self.sd_block_gap_ctrl
    }
    #[doc = "0x2c - Clock Control Register"]
    #[inline(always)]
    pub const fn sd_clock_ctrl(&self) -> &SdClockCtrl {
        &self.sd_clock_ctrl
    }
    #[doc = "0x2e - Timeout Control / Software Reset Register"]
    #[inline(always)]
    pub const fn sd_timeout_ctrl_sw_reset(&self) -> &SdTimeoutCtrlSwReset {
        &self.sd_timeout_ctrl_sw_reset
    }
    #[doc = "0x30 - Normal Interrupt Status Register (Read/Write 1 to Clear)"]
    #[inline(always)]
    pub const fn sd_normal_int_status(&self) -> &SdNormalIntStatus {
        &self.sd_normal_int_status
    }
    #[doc = "0x32 - Error Interrupt Status Register (Read/Write 1 to Clear)"]
    #[inline(always)]
    pub const fn sd_error_int_status(&self) -> &SdErrorIntStatus {
        &self.sd_error_int_status
    }
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn sd_normal_int_status_en(&self) -> &SdNormalIntStatusEn {
        &self.sd_normal_int_status_en
    }
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn sd_error_int_status_en(&self) -> &SdErrorIntStatusEn {
        &self.sd_error_int_status_en
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn sd_normal_int_status_int_en(&self) -> &SdNormalIntStatusIntEn {
        &self.sd_normal_int_status_int_en
    }
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn sd_error_int_status_int_en(&self) -> &SdErrorIntStatusIntEn {
        &self.sd_error_int_status_int_en
    }
    #[doc = "0x3c - Auto CMD12/23 Error Status Register"]
    #[inline(always)]
    pub const fn sd_auto_cmd12_error_status(&self) -> &SdAutoCmd12ErrorStatus {
        &self.sd_auto_cmd12_error_status
    }
    #[doc = "0x3e - Host Control 2 Register (UHS-I and Sampling Control)"]
    #[inline(always)]
    pub const fn host_ctrl_2(&self) -> &HostCtrl2 {
        &self.host_ctrl_2
    }
    #[doc = "0x40 - Host Controller Capabilities Register 1 (Freq/Clock)"]
    #[inline(always)]
    pub const fn sd_capabilities_1(&self) -> &SdCapabilities1 {
        &self.sd_capabilities_1
    }
    #[doc = "0x42 - Host Controller Capabilities Register 2 (Voltage/DMA)"]
    #[inline(always)]
    pub const fn sd_capabilities_2(&self) -> &SdCapabilities2 {
        &self.sd_capabilities_2
    }
    #[doc = "0x44 - Host Controller Capabilities Register 3 (UHS-I)"]
    #[inline(always)]
    pub const fn sd_capabilities_3(&self) -> &SdCapabilities3 {
        &self.sd_capabilities_3
    }
    #[doc = "0x46 - Host Controller Capabilities Register 4 (Clock Multiplier)"]
    #[inline(always)]
    pub const fn sd_capabilities_4(&self) -> &SdCapabilities4 {
        &self.sd_capabilities_4
    }
    #[doc = "0x48 - Host Controller Maximum Current Capabilities 1"]
    #[inline(always)]
    pub const fn sd_max_current_1(&self) -> &SdMaxCurrent1 {
        &self.sd_max_current_1
    }
    #[doc = "0x4a - Host Controller Maximum Current Capabilities 2"]
    #[inline(always)]
    pub const fn sd_max_current_2(&self) -> &SdMaxCurrent2 {
        &self.sd_max_current_2
    }
    #[doc = "0x4c - Maximum Current Capabilities Register 3 (Reserved)"]
    #[inline(always)]
    pub const fn sd_max_current_3(&self) -> &SdMaxCurrent3 {
        &self.sd_max_current_3
    }
    #[doc = "0x4e - Maximum Current Capabilities Register 4 (Reserved)"]
    #[inline(always)]
    pub const fn sd_max_current_4(&self) -> &SdMaxCurrent4 {
        &self.sd_max_current_4
    }
    #[doc = "0x50 - Force Event Register for Auto CMD12 Error Status"]
    #[inline(always)]
    pub const fn sd_force_event_auto_cmd12_error(&self) -> &SdForceEventAutoCmd12Error {
        &self.sd_force_event_auto_cmd12_error
    }
    #[doc = "0x52 - Force Event Register for Error Status"]
    #[inline(always)]
    pub const fn sd_force_event_for_error_status(&self) -> &SdForceEventForErrorStatus {
        &self.sd_force_event_for_error_status
    }
    #[doc = "0x54 - ADMA Error Status Register"]
    #[inline(always)]
    pub const fn sd_adma_error_status(&self) -> &SdAdmaErrorStatus {
        &self.sd_adma_error_status
    }
    #[doc = "0x58 - ADMA System Address Register 1 (Bits 15:0)"]
    #[inline(always)]
    pub const fn sd_adma_sys_addr_1(&self) -> &SdAdmaSysAddr1 {
        &self.sd_adma_sys_addr_1
    }
    #[doc = "0x5a - ADMA System Address Register 2 (Bits 31:16)"]
    #[inline(always)]
    pub const fn sd_adma_sys_addr_2(&self) -> &SdAdmaSysAddr2 {
        &self.sd_adma_sys_addr_2
    }
    #[doc = "0x5c - ADMA System Address Register 3 (Bits 47:32)"]
    #[inline(always)]
    pub const fn sd_adma_sys_addr_3(&self) -> &SdAdmaSysAddr3 {
        &self.sd_adma_sys_addr_3
    }
    #[doc = "0x5e - ADMA System Address Register 4 (Bits 63:48)"]
    #[inline(always)]
    pub const fn sd_adma_sys_addr_4(&self) -> &SdAdmaSysAddr4 {
        &self.sd_adma_sys_addr_4
    }
    #[doc = "0x60 - Preset Value for Initialization"]
    #[inline(always)]
    pub const fn preset_value_for_init(&self) -> &PresetValueForInit {
        &self.preset_value_for_init
    }
    #[doc = "0x62 - Preset Value for Default Speed"]
    #[inline(always)]
    pub const fn preset_value_for_ds(&self) -> &PresetValueForDs {
        &self.preset_value_for_ds
    }
    #[doc = "0x64 - Preset Value for High Speed"]
    #[inline(always)]
    pub const fn preset_value_for_hs(&self) -> &PresetValueForHs {
        &self.preset_value_for_hs
    }
    #[doc = "0x66 - Preset Value for SDR12"]
    #[inline(always)]
    pub const fn preset_value_for_sdr12(&self) -> &PresetValueForSdr12 {
        &self.preset_value_for_sdr12
    }
    #[doc = "0x68 - Preset Value for SDR25"]
    #[inline(always)]
    pub const fn preset_value_for_sdr25(&self) -> &PresetValueForSdr25 {
        &self.preset_value_for_sdr25
    }
    #[doc = "0x6a - Preset Value for SDR50"]
    #[inline(always)]
    pub const fn preset_value_for_sdr50(&self) -> &PresetValueForSdr50 {
        &self.preset_value_for_sdr50
    }
    #[doc = "0x6c - Preset Value for SDR104"]
    #[inline(always)]
    pub const fn preset_value_for_sdr104(&self) -> &PresetValueForSdr104 {
        &self.preset_value_for_sdr104
    }
    #[doc = "0x6e - Preset Value for DDR50"]
    #[inline(always)]
    pub const fn preset_value_for_ddr50(&self) -> &PresetValueForDdr50 {
        &self.preset_value_for_ddr50
    }
    #[doc = "0xe0 - Shared Bus Control Register (Vendor Specific)"]
    #[inline(always)]
    pub const fn shared_bus_ctrl(&self) -> &SharedBusCtrl {
        &self.shared_bus_ctrl
    }
    #[doc = "0xfc - Slot Interrupt Status Register"]
    #[inline(always)]
    pub const fn sd_slot_int_status(&self) -> &SdSlotIntStatus {
        &self.sd_slot_int_status
    }
    #[doc = "0xfe - Host Controller Version Register"]
    #[inline(always)]
    pub const fn sd_host_ctrl_ver(&self) -> &SdHostCtrlVer {
        &self.sd_host_ctrl_ver
    }
    #[doc = "0x100 - Vendor specific FIFO and PAD clock configuration"]
    #[inline(always)]
    pub const fn sd_cfg_fifo_param(&self) -> &SdCfgFifoParam {
        &self.sd_cfg_fifo_param
    }
    #[doc = "0x104 - Vendor specific FIFO control and clock gating parameters"]
    #[inline(always)]
    pub const fn sd_fifo_param(&self) -> &SdFifoParam {
        &self.sd_fifo_param
    }
    #[doc = "0x108 - SD legacy SPI mode control and error tokens"]
    #[inline(always)]
    pub const fn sd_spi_mode(&self) -> &SdSpiMode {
        &self.sd_spi_mode
    }
    #[doc = "0x10a - Vendor specific burst size and endianness configuration"]
    #[inline(always)]
    pub const fn sd_clock_and_burst_size_setup(&self) -> &SdClockAndBurstSizeSetup {
        &self.sd_clock_and_burst_size_setup
    }
    #[doc = "0x10c - CE-ATA specific timeout configuration"]
    #[inline(always)]
    pub const fn sd_ce_ata_1(&self) -> &SdCeAta1 {
        &self.sd_ce_ata_1
    }
    #[doc = "0x10e - CE-ATA specific command completion and interrupt control"]
    #[inline(always)]
    pub const fn sd_ce_ata_2(&self) -> &SdCeAta2 {
        &self.sd_ce_ata_2
    }
    #[doc = "0x110 - Vendor specific IO pad and ECO register configuration"]
    #[inline(always)]
    pub const fn sd_pad_io_setup(&self) -> &SdPadIoSetup {
        &self.sd_pad_io_setup
    }
    #[doc = "0x114 - Vendor specific RX sampling clock delay and tuning configuration"]
    #[inline(always)]
    pub const fn rx_cfg_reg(&self) -> &RxCfgReg {
        &self.rx_cfg_reg
    }
    #[doc = "0x118 - Vendor specific TX clock output delay and muxing configuration"]
    #[inline(always)]
    pub const fn tx_cfg_reg(&self) -> &TxCfgReg {
        &self.tx_cfg_reg
    }
    #[doc = "0x11c - Vendor specific hardware auto-tuning results and clock delay status"]
    #[inline(always)]
    pub const fn tuning_cfg_reg(&self) -> &TuningCfgReg {
        &self.tuning_cfg_reg
    }
}
#[doc = "SD_SYS_ADDR_LOW (rw) register accessor: System Address Low Register (16-bit part of SDMA/ADMA address)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_sys_addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_sys_addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_sys_addr_low`] module"]
#[doc(alias = "SD_SYS_ADDR_LOW")]
pub type SdSysAddrLow = crate::Reg<sd_sys_addr_low::SdSysAddrLowSpec>;
#[doc = "System Address Low Register (16-bit part of SDMA/ADMA address)"]
pub mod sd_sys_addr_low;
#[doc = "SD_SYS_ADDR_HIGH (rw) register accessor: System Address High Register (16-bit part of SDMA/ADMA address)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_sys_addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_sys_addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_sys_addr_high`] module"]
#[doc(alias = "SD_SYS_ADDR_HIGH")]
pub type SdSysAddrHigh = crate::Reg<sd_sys_addr_high::SdSysAddrHighSpec>;
#[doc = "System Address High Register (16-bit part of SDMA/ADMA address)"]
pub mod sd_sys_addr_high;
#[doc = "SD_BLOCK_SIZE (rw) register accessor: Block Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_block_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_block_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_block_size`] module"]
#[doc(alias = "SD_BLOCK_SIZE")]
pub type SdBlockSize = crate::Reg<sd_block_size::SdBlockSizeSpec>;
#[doc = "Block Size Register"]
pub mod sd_block_size;
#[doc = "SD_BLOCK_COUNT (rw) register accessor: Block Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_block_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_block_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_block_count`] module"]
#[doc(alias = "SD_BLOCK_COUNT")]
pub type SdBlockCount = crate::Reg<sd_block_count::SdBlockCountSpec>;
#[doc = "Block Count Register"]
pub mod sd_block_count;
#[doc = "SD_ARG_LOW (rw) register accessor: Command Argument Low Register (Bits 15:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_arg_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_arg_low`] module"]
#[doc(alias = "SD_ARG_LOW")]
pub type SdArgLow = crate::Reg<sd_arg_low::SdArgLowSpec>;
#[doc = "Command Argument Low Register (Bits 15:0)"]
pub mod sd_arg_low;
#[doc = "SD_ARG_HIGH (rw) register accessor: Command Argument High Register (Bits 31:16)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_arg_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_arg_high`] module"]
#[doc(alias = "SD_ARG_HIGH")]
pub type SdArgHigh = crate::Reg<sd_arg_high::SdArgHighSpec>;
#[doc = "Command Argument High Register (Bits 31:16)"]
pub mod sd_arg_high;
#[doc = "SD_TRANSFER_MODE (rw) register accessor: Transfer Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_transfer_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_transfer_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_transfer_mode`] module"]
#[doc(alias = "SD_TRANSFER_MODE")]
pub type SdTransferMode = crate::Reg<sd_transfer_mode::SdTransferModeSpec>;
#[doc = "Transfer Mode Register"]
pub mod sd_transfer_mode;
#[doc = "SD_CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_cmd`] module"]
#[doc(alias = "SD_CMD")]
pub type SdCmd = crate::Reg<sd_cmd::SdCmdSpec>;
#[doc = "Command Register"]
pub mod sd_cmd;
#[doc = "SD_RESP_0 (r) register accessor: Response Register 0 (Bits 15:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_0`] module"]
#[doc(alias = "SD_RESP_0")]
pub type SdResp0 = crate::Reg<sd_resp_0::SdResp0Spec>;
#[doc = "Response Register 0 (Bits 15:0)"]
pub mod sd_resp_0;
#[doc = "SD_RESP_1 (r) register accessor: Response Register 1 (Bits 31:16)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_1`] module"]
#[doc(alias = "SD_RESP_1")]
pub type SdResp1 = crate::Reg<sd_resp_1::SdResp1Spec>;
#[doc = "Response Register 1 (Bits 31:16)"]
pub mod sd_resp_1;
#[doc = "SD_RESP_2 (r) register accessor: Response Register 2 (Bits 47:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_2`] module"]
#[doc(alias = "SD_RESP_2")]
pub type SdResp2 = crate::Reg<sd_resp_2::SdResp2Spec>;
#[doc = "Response Register 2 (Bits 47:32)"]
pub mod sd_resp_2;
#[doc = "SD_RESP_3 (r) register accessor: Response Register 3 (Bits 63:48)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_3`] module"]
#[doc(alias = "SD_RESP_3")]
pub type SdResp3 = crate::Reg<sd_resp_3::SdResp3Spec>;
#[doc = "Response Register 3 (Bits 63:48)"]
pub mod sd_resp_3;
#[doc = "SD_RESP_4 (r) register accessor: Response Register 4 (Bits 79:64)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_4`] module"]
#[doc(alias = "SD_RESP_4")]
pub type SdResp4 = crate::Reg<sd_resp_4::SdResp4Spec>;
#[doc = "Response Register 4 (Bits 79:64)"]
pub mod sd_resp_4;
#[doc = "SD_RESP_5 (r) register accessor: Response Register 5 (Bits 95:80)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_5`] module"]
#[doc(alias = "SD_RESP_5")]
pub type SdResp5 = crate::Reg<sd_resp_5::SdResp5Spec>;
#[doc = "Response Register 5 (Bits 95:80)"]
pub mod sd_resp_5;
#[doc = "SD_RESP_6 (r) register accessor: Response Register 6 (Bits 111:96)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_6`] module"]
#[doc(alias = "SD_RESP_6")]
pub type SdResp6 = crate::Reg<sd_resp_6::SdResp6Spec>;
#[doc = "Response Register 6 (Bits 111:96)"]
pub mod sd_resp_6;
#[doc = "SD_RESP_7 (r) register accessor: Response Register 7 (Bits 127:112)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_resp_7`] module"]
#[doc(alias = "SD_RESP_7")]
pub type SdResp7 = crate::Reg<sd_resp_7::SdResp7Spec>;
#[doc = "Response Register 7 (Bits 127:112)"]
pub mod sd_resp_7;
#[doc = "SD_BUFFER_DATA_PORT_0 (rw) register accessor: Buffer Data Port 0 (16 bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_buffer_data_port_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buffer_data_port_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_buffer_data_port_0`] module"]
#[doc(alias = "SD_BUFFER_DATA_PORT_0")]
pub type SdBufferDataPort0 = crate::Reg<sd_buffer_data_port_0::SdBufferDataPort0Spec>;
#[doc = "Buffer Data Port 0 (16 bits)"]
pub mod sd_buffer_data_port_0;
#[doc = "SD_BUFFER_DATA_PORT_1 (rw) register accessor: Buffer Data Port 1 (16 bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_buffer_data_port_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buffer_data_port_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_buffer_data_port_1`] module"]
#[doc(alias = "SD_BUFFER_DATA_PORT_1")]
pub type SdBufferDataPort1 = crate::Reg<sd_buffer_data_port_1::SdBufferDataPort1Spec>;
#[doc = "Buffer Data Port 1 (16 bits)"]
pub mod sd_buffer_data_port_1;
#[doc = "SD_PRESENT_STATE_1 (r) register accessor: Present State Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_present_state_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_present_state_1`] module"]
#[doc(alias = "SD_PRESENT_STATE_1")]
pub type SdPresentState1 = crate::Reg<sd_present_state_1::SdPresentState1Spec>;
#[doc = "Present State Register 1"]
pub mod sd_present_state_1;
#[doc = "SD_PRESENT_STATE_2 (r) register accessor: Present State Register 2 (Card Detection)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_present_state_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_present_state_2`] module"]
#[doc(alias = "SD_PRESENT_STATE_2")]
pub type SdPresentState2 = crate::Reg<sd_present_state_2::SdPresentState2Spec>;
#[doc = "Present State Register 2 (Card Detection)"]
pub mod sd_present_state_2;
#[doc = "SD_HOST_CTRL (rw) register accessor: Host Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_host_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_host_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_host_ctrl`] module"]
#[doc(alias = "SD_HOST_CTRL")]
pub type SdHostCtrl = crate::Reg<sd_host_ctrl::SdHostCtrlSpec>;
#[doc = "Host Control Register 1"]
pub mod sd_host_ctrl;
#[doc = "SD_BLOCK_GAP_CTRL (rw) register accessor: Block Gap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_block_gap_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_block_gap_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_block_gap_ctrl`] module"]
#[doc(alias = "SD_BLOCK_GAP_CTRL")]
pub type SdBlockGapCtrl = crate::Reg<sd_block_gap_ctrl::SdBlockGapCtrlSpec>;
#[doc = "Block Gap Control Register"]
pub mod sd_block_gap_ctrl;
#[doc = "SD_CLOCK_CTRL (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_clock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_clock_ctrl`] module"]
#[doc(alias = "SD_CLOCK_CTRL")]
pub type SdClockCtrl = crate::Reg<sd_clock_ctrl::SdClockCtrlSpec>;
#[doc = "Clock Control Register"]
pub mod sd_clock_ctrl;
#[doc = "SD_TIMEOUT_CTRL_SW_RESET (rw) register accessor: Timeout Control / Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_timeout_ctrl_sw_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_timeout_ctrl_sw_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_timeout_ctrl_sw_reset`] module"]
#[doc(alias = "SD_TIMEOUT_CTRL_SW_RESET")]
pub type SdTimeoutCtrlSwReset = crate::Reg<sd_timeout_ctrl_sw_reset::SdTimeoutCtrlSwResetSpec>;
#[doc = "Timeout Control / Software Reset Register"]
pub mod sd_timeout_ctrl_sw_reset;
#[doc = "SD_NORMAL_INT_STATUS (rw) register accessor: Normal Interrupt Status Register (Read/Write 1 to Clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_normal_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_normal_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_normal_int_status`] module"]
#[doc(alias = "SD_NORMAL_INT_STATUS")]
pub type SdNormalIntStatus = crate::Reg<sd_normal_int_status::SdNormalIntStatusSpec>;
#[doc = "Normal Interrupt Status Register (Read/Write 1 to Clear)"]
pub mod sd_normal_int_status;
#[doc = "SD_ERROR_INT_STATUS (rw) register accessor: Error Interrupt Status Register (Read/Write 1 to Clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_error_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_error_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_error_int_status`] module"]
#[doc(alias = "SD_ERROR_INT_STATUS")]
pub type SdErrorIntStatus = crate::Reg<sd_error_int_status::SdErrorIntStatusSpec>;
#[doc = "Error Interrupt Status Register (Read/Write 1 to Clear)"]
pub mod sd_error_int_status;
#[doc = "SD_NORMAL_INT_STATUS_EN (rw) register accessor: Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_normal_int_status_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_normal_int_status_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_normal_int_status_en`] module"]
#[doc(alias = "SD_NORMAL_INT_STATUS_EN")]
pub type SdNormalIntStatusEn = crate::Reg<sd_normal_int_status_en::SdNormalIntStatusEnSpec>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod sd_normal_int_status_en;
#[doc = "SD_ERROR_INT_STATUS_EN (rw) register accessor: Error Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_error_int_status_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_error_int_status_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_error_int_status_en`] module"]
#[doc(alias = "SD_ERROR_INT_STATUS_EN")]
pub type SdErrorIntStatusEn = crate::Reg<sd_error_int_status_en::SdErrorIntStatusEnSpec>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod sd_error_int_status_en;
#[doc = "SD_NORMAL_INT_STATUS_INT_EN (rw) register accessor: Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_normal_int_status_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_normal_int_status_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_normal_int_status_int_en`] module"]
#[doc(alias = "SD_NORMAL_INT_STATUS_INT_EN")]
pub type SdNormalIntStatusIntEn =
    crate::Reg<sd_normal_int_status_int_en::SdNormalIntStatusIntEnSpec>;
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod sd_normal_int_status_int_en;
#[doc = "SD_ERROR_INT_STATUS_INT_EN (rw) register accessor: Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_error_int_status_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_error_int_status_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_error_int_status_int_en`] module"]
#[doc(alias = "SD_ERROR_INT_STATUS_INT_EN")]
pub type SdErrorIntStatusIntEn = crate::Reg<sd_error_int_status_int_en::SdErrorIntStatusIntEnSpec>;
#[doc = "Error Interrupt Signal Enable Register"]
pub mod sd_error_int_status_int_en;
#[doc = "SD_AUTO_CMD12_ERROR_STATUS (rw) register accessor: Auto CMD12/23 Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_auto_cmd12_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_auto_cmd12_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_auto_cmd12_error_status`] module"]
#[doc(alias = "SD_AUTO_CMD12_ERROR_STATUS")]
pub type SdAutoCmd12ErrorStatus =
    crate::Reg<sd_auto_cmd12_error_status::SdAutoCmd12ErrorStatusSpec>;
#[doc = "Auto CMD12/23 Error Status Register"]
pub mod sd_auto_cmd12_error_status;
#[doc = "HOST_CTRL_2 (rw) register accessor: Host Control 2 Register (UHS-I and Sampling Control)\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl_2`] module"]
#[doc(alias = "HOST_CTRL_2")]
pub type HostCtrl2 = crate::Reg<host_ctrl_2::HostCtrl2Spec>;
#[doc = "Host Control 2 Register (UHS-I and Sampling Control)"]
pub mod host_ctrl_2;
#[doc = "SD_CAPABILITIES_1 (r) register accessor: Host Controller Capabilities Register 1 (Freq/Clock)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_capabilities_1`] module"]
#[doc(alias = "SD_CAPABILITIES_1")]
pub type SdCapabilities1 = crate::Reg<sd_capabilities_1::SdCapabilities1Spec>;
#[doc = "Host Controller Capabilities Register 1 (Freq/Clock)"]
pub mod sd_capabilities_1;
#[doc = "SD_CAPABILITIES_2 (rw) register accessor: Host Controller Capabilities Register 2 (Voltage/DMA)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_capabilities_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_capabilities_2`] module"]
#[doc(alias = "SD_CAPABILITIES_2")]
pub type SdCapabilities2 = crate::Reg<sd_capabilities_2::SdCapabilities2Spec>;
#[doc = "Host Controller Capabilities Register 2 (Voltage/DMA)"]
pub mod sd_capabilities_2;
#[doc = "SD_CAPABILITIES_3 (r) register accessor: Host Controller Capabilities Register 3 (UHS-I)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_capabilities_3`] module"]
#[doc(alias = "SD_CAPABILITIES_3")]
pub type SdCapabilities3 = crate::Reg<sd_capabilities_3::SdCapabilities3Spec>;
#[doc = "Host Controller Capabilities Register 3 (UHS-I)"]
pub mod sd_capabilities_3;
#[doc = "SD_CAPABILITIES_4 (r) register accessor: Host Controller Capabilities Register 4 (Clock Multiplier)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_capabilities_4`] module"]
#[doc(alias = "SD_CAPABILITIES_4")]
pub type SdCapabilities4 = crate::Reg<sd_capabilities_4::SdCapabilities4Spec>;
#[doc = "Host Controller Capabilities Register 4 (Clock Multiplier)"]
pub mod sd_capabilities_4;
#[doc = "SD_MAX_CURRENT_1 (r) register accessor: Host Controller Maximum Current Capabilities 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_max_current_1`] module"]
#[doc(alias = "SD_MAX_CURRENT_1")]
pub type SdMaxCurrent1 = crate::Reg<sd_max_current_1::SdMaxCurrent1Spec>;
#[doc = "Host Controller Maximum Current Capabilities 1"]
pub mod sd_max_current_1;
#[doc = "SD_MAX_CURRENT_2 (r) register accessor: Host Controller Maximum Current Capabilities 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_max_current_2`] module"]
#[doc(alias = "SD_MAX_CURRENT_2")]
pub type SdMaxCurrent2 = crate::Reg<sd_max_current_2::SdMaxCurrent2Spec>;
#[doc = "Host Controller Maximum Current Capabilities 2"]
pub mod sd_max_current_2;
#[doc = "SD_MAX_CURRENT_3 (r) register accessor: Maximum Current Capabilities Register 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_max_current_3`] module"]
#[doc(alias = "SD_MAX_CURRENT_3")]
pub type SdMaxCurrent3 = crate::Reg<sd_max_current_3::SdMaxCurrent3Spec>;
#[doc = "Maximum Current Capabilities Register 3 (Reserved)"]
pub mod sd_max_current_3;
#[doc = "SD_MAX_CURRENT_4 (r) register accessor: Maximum Current Capabilities Register 4 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_max_current_4`] module"]
#[doc(alias = "SD_MAX_CURRENT_4")]
pub type SdMaxCurrent4 = crate::Reg<sd_max_current_4::SdMaxCurrent4Spec>;
#[doc = "Maximum Current Capabilities Register 4 (Reserved)"]
pub mod sd_max_current_4;
#[doc = "SD_FORCE_EVENT_AUTO_CMD12_ERROR (w) register accessor: Force Event Register for Auto CMD12 Error Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_force_event_auto_cmd12_error::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_force_event_auto_cmd12_error`] module"]
#[doc(alias = "SD_FORCE_EVENT_AUTO_CMD12_ERROR")]
pub type SdForceEventAutoCmd12Error =
    crate::Reg<sd_force_event_auto_cmd12_error::SdForceEventAutoCmd12ErrorSpec>;
#[doc = "Force Event Register for Auto CMD12 Error Status"]
pub mod sd_force_event_auto_cmd12_error;
#[doc = "SD_FORCE_EVENT_FOR_ERROR_STATUS (w) register accessor: Force Event Register for Error Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_force_event_for_error_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_force_event_for_error_status`] module"]
#[doc(alias = "SD_FORCE_EVENT_FOR_ERROR_STATUS")]
pub type SdForceEventForErrorStatus =
    crate::Reg<sd_force_event_for_error_status::SdForceEventForErrorStatusSpec>;
#[doc = "Force Event Register for Error Status"]
pub mod sd_force_event_for_error_status;
#[doc = "SD_ADMA_ERROR_STATUS (rw) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_adma_error_status`] module"]
#[doc(alias = "SD_ADMA_ERROR_STATUS")]
pub type SdAdmaErrorStatus = crate::Reg<sd_adma_error_status::SdAdmaErrorStatusSpec>;
#[doc = "ADMA Error Status Register"]
pub mod sd_adma_error_status;
#[doc = "SD_ADMA_SYS_ADDR_1 (rw) register accessor: ADMA System Address Register 1 (Bits 15:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_sys_addr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_sys_addr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_adma_sys_addr_1`] module"]
#[doc(alias = "SD_ADMA_SYS_ADDR_1")]
pub type SdAdmaSysAddr1 = crate::Reg<sd_adma_sys_addr_1::SdAdmaSysAddr1Spec>;
#[doc = "ADMA System Address Register 1 (Bits 15:0)"]
pub mod sd_adma_sys_addr_1;
#[doc = "SD_ADMA_SYS_ADDR_2 (rw) register accessor: ADMA System Address Register 2 (Bits 31:16)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_sys_addr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_sys_addr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_adma_sys_addr_2`] module"]
#[doc(alias = "SD_ADMA_SYS_ADDR_2")]
pub type SdAdmaSysAddr2 = crate::Reg<sd_adma_sys_addr_2::SdAdmaSysAddr2Spec>;
#[doc = "ADMA System Address Register 2 (Bits 31:16)"]
pub mod sd_adma_sys_addr_2;
#[doc = "SD_ADMA_SYS_ADDR_3 (rw) register accessor: ADMA System Address Register 3 (Bits 47:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_sys_addr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_sys_addr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_adma_sys_addr_3`] module"]
#[doc(alias = "SD_ADMA_SYS_ADDR_3")]
pub type SdAdmaSysAddr3 = crate::Reg<sd_adma_sys_addr_3::SdAdmaSysAddr3Spec>;
#[doc = "ADMA System Address Register 3 (Bits 47:32)"]
pub mod sd_adma_sys_addr_3;
#[doc = "SD_ADMA_SYS_ADDR_4 (rw) register accessor: ADMA System Address Register 4 (Bits 63:48)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_sys_addr_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_sys_addr_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_adma_sys_addr_4`] module"]
#[doc(alias = "SD_ADMA_SYS_ADDR_4")]
pub type SdAdmaSysAddr4 = crate::Reg<sd_adma_sys_addr_4::SdAdmaSysAddr4Spec>;
#[doc = "ADMA System Address Register 4 (Bits 63:48)"]
pub mod sd_adma_sys_addr_4;
#[doc = "PRESET_VALUE_FOR_INIT (r) register accessor: Preset Value for Initialization\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_init::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_init`] module"]
#[doc(alias = "PRESET_VALUE_FOR_INIT")]
pub type PresetValueForInit = crate::Reg<preset_value_for_init::PresetValueForInitSpec>;
#[doc = "Preset Value for Initialization"]
pub mod preset_value_for_init;
#[doc = "PRESET_VALUE_FOR_DS (r) register accessor: Preset Value for Default Speed\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_ds::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_ds`] module"]
#[doc(alias = "PRESET_VALUE_FOR_DS")]
pub type PresetValueForDs = crate::Reg<preset_value_for_ds::PresetValueForDsSpec>;
#[doc = "Preset Value for Default Speed"]
pub mod preset_value_for_ds;
#[doc = "PRESET_VALUE_FOR_HS (r) register accessor: Preset Value for High Speed\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_hs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_hs`] module"]
#[doc(alias = "PRESET_VALUE_FOR_HS")]
pub type PresetValueForHs = crate::Reg<preset_value_for_hs::PresetValueForHsSpec>;
#[doc = "Preset Value for High Speed"]
pub mod preset_value_for_hs;
#[doc = "PRESET_VALUE_FOR_SDR12 (r) register accessor: Preset Value for SDR12\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_sdr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_sdr12`] module"]
#[doc(alias = "PRESET_VALUE_FOR_SDR12")]
pub type PresetValueForSdr12 = crate::Reg<preset_value_for_sdr12::PresetValueForSdr12Spec>;
#[doc = "Preset Value for SDR12"]
pub mod preset_value_for_sdr12;
#[doc = "PRESET_VALUE_FOR_SDR25 (r) register accessor: Preset Value for SDR25\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_sdr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_sdr25`] module"]
#[doc(alias = "PRESET_VALUE_FOR_SDR25")]
pub type PresetValueForSdr25 = crate::Reg<preset_value_for_sdr25::PresetValueForSdr25Spec>;
#[doc = "Preset Value for SDR25"]
pub mod preset_value_for_sdr25;
#[doc = "PRESET_VALUE_FOR_SDR50 (r) register accessor: Preset Value for SDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_sdr50::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_sdr50`] module"]
#[doc(alias = "PRESET_VALUE_FOR_SDR50")]
pub type PresetValueForSdr50 = crate::Reg<preset_value_for_sdr50::PresetValueForSdr50Spec>;
#[doc = "Preset Value for SDR50"]
pub mod preset_value_for_sdr50;
#[doc = "PRESET_VALUE_FOR_SDR104 (r) register accessor: Preset Value for SDR104\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_sdr104::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_sdr104`] module"]
#[doc(alias = "PRESET_VALUE_FOR_SDR104")]
pub type PresetValueForSdr104 = crate::Reg<preset_value_for_sdr104::PresetValueForSdr104Spec>;
#[doc = "Preset Value for SDR104"]
pub mod preset_value_for_sdr104;
#[doc = "PRESET_VALUE_FOR_DDR50 (r) register accessor: Preset Value for DDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_ddr50::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_value_for_ddr50`] module"]
#[doc(alias = "PRESET_VALUE_FOR_DDR50")]
pub type PresetValueForDdr50 = crate::Reg<preset_value_for_ddr50::PresetValueForDdr50Spec>;
#[doc = "Preset Value for DDR50"]
pub mod preset_value_for_ddr50;
#[doc = "SHARED_BUS_CTRL (rw) register accessor: Shared Bus Control Register (Vendor Specific)\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_bus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_bus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shared_bus_ctrl`] module"]
#[doc(alias = "SHARED_BUS_CTRL")]
pub type SharedBusCtrl = crate::Reg<shared_bus_ctrl::SharedBusCtrlSpec>;
#[doc = "Shared Bus Control Register (Vendor Specific)"]
pub mod shared_bus_ctrl;
#[doc = "SD_SLOT_INT_STATUS (r) register accessor: Slot Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_slot_int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_slot_int_status`] module"]
#[doc(alias = "SD_SLOT_INT_STATUS")]
pub type SdSlotIntStatus = crate::Reg<sd_slot_int_status::SdSlotIntStatusSpec>;
#[doc = "Slot Interrupt Status Register"]
pub mod sd_slot_int_status;
#[doc = "SD_HOST_CTRL_VER (r) register accessor: Host Controller Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_host_ctrl_ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_host_ctrl_ver`] module"]
#[doc(alias = "SD_HOST_CTRL_VER")]
pub type SdHostCtrlVer = crate::Reg<sd_host_ctrl_ver::SdHostCtrlVerSpec>;
#[doc = "Host Controller Version Register"]
pub mod sd_host_ctrl_ver;
#[doc = "SD_CFG_FIFO_PARAM (rw) register accessor: Vendor specific FIFO and PAD clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_cfg_fifo_param::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cfg_fifo_param::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_cfg_fifo_param`] module"]
#[doc(alias = "SD_CFG_FIFO_PARAM")]
pub type SdCfgFifoParam = crate::Reg<sd_cfg_fifo_param::SdCfgFifoParamSpec>;
#[doc = "Vendor specific FIFO and PAD clock configuration"]
pub mod sd_cfg_fifo_param;
#[doc = "SD_FIFO_PARAM (rw) register accessor: Vendor specific FIFO control and clock gating parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_fifo_param::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_fifo_param::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_fifo_param`] module"]
#[doc(alias = "SD_FIFO_PARAM")]
pub type SdFifoParam = crate::Reg<sd_fifo_param::SdFifoParamSpec>;
#[doc = "Vendor specific FIFO control and clock gating parameters"]
pub mod sd_fifo_param;
#[doc = "SD_SPI_MODE (rw) register accessor: SD legacy SPI mode control and error tokens\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_spi_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_spi_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_spi_mode`] module"]
#[doc(alias = "SD_SPI_MODE")]
pub type SdSpiMode = crate::Reg<sd_spi_mode::SdSpiModeSpec>;
#[doc = "SD legacy SPI mode control and error tokens"]
pub mod sd_spi_mode;
#[doc = "SD_CLOCK_AND_BURST_SIZE_SETUP (rw) register accessor: Vendor specific burst size and endianness configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_clock_and_burst_size_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clock_and_burst_size_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_clock_and_burst_size_setup`] module"]
#[doc(alias = "SD_CLOCK_AND_BURST_SIZE_SETUP")]
pub type SdClockAndBurstSizeSetup =
    crate::Reg<sd_clock_and_burst_size_setup::SdClockAndBurstSizeSetupSpec>;
#[doc = "Vendor specific burst size and endianness configuration"]
pub mod sd_clock_and_burst_size_setup;
#[doc = "SD_CE_ATA_1 (rw) register accessor: CE-ATA specific timeout configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_ce_ata_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_ce_ata_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_ce_ata_1`] module"]
#[doc(alias = "SD_CE_ATA_1")]
pub type SdCeAta1 = crate::Reg<sd_ce_ata_1::SdCeAta1Spec>;
#[doc = "CE-ATA specific timeout configuration"]
pub mod sd_ce_ata_1;
#[doc = "SD_CE_ATA_2 (rw) register accessor: CE-ATA specific command completion and interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_ce_ata_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_ce_ata_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_ce_ata_2`] module"]
#[doc(alias = "SD_CE_ATA_2")]
pub type SdCeAta2 = crate::Reg<sd_ce_ata_2::SdCeAta2Spec>;
#[doc = "CE-ATA specific command completion and interrupt control"]
pub mod sd_ce_ata_2;
#[doc = "SD_PAD_IO_SETUP (rw) register accessor: Vendor specific IO pad and ECO register configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_pad_io_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_pad_io_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_pad_io_setup`] module"]
#[doc(alias = "SD_PAD_IO_SETUP")]
pub type SdPadIoSetup = crate::Reg<sd_pad_io_setup::SdPadIoSetupSpec>;
#[doc = "Vendor specific IO pad and ECO register configuration"]
pub mod sd_pad_io_setup;
#[doc = "RX_CFG_REG (rw) register accessor: Vendor specific RX sampling clock delay and tuning configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_cfg_reg`] module"]
#[doc(alias = "RX_CFG_REG")]
pub type RxCfgReg = crate::Reg<rx_cfg_reg::RxCfgRegSpec>;
#[doc = "Vendor specific RX sampling clock delay and tuning configuration"]
pub mod rx_cfg_reg;
#[doc = "TX_CFG_REG (rw) register accessor: Vendor specific TX clock output delay and muxing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_cfg_reg`] module"]
#[doc(alias = "TX_CFG_REG")]
pub type TxCfgReg = crate::Reg<tx_cfg_reg::TxCfgRegSpec>;
#[doc = "Vendor specific TX clock output delay and muxing configuration"]
pub mod tx_cfg_reg;
#[doc = "TUNING_CFG_REG (rw) register accessor: Vendor specific hardware auto-tuning results and clock delay status\n\nYou can [`read`](crate::Reg::read) this register and get [`tuning_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tuning_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tuning_cfg_reg`] module"]
#[doc(alias = "TUNING_CFG_REG")]
pub type TuningCfgReg = crate::Reg<tuning_cfg_reg::TuningCfgRegSpec>;
#[doc = "Vendor specific hardware auto-tuning results and clock delay status"]
pub mod tuning_cfg_reg;
