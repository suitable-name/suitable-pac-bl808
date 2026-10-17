#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: Mode,
    command: Command,
    status: Status,
    interrupt: Interrupt,
    interrupt_enable: InterruptEnable,
    _reserved5: [u8; 0x04],
    bus_timing_0: BusTiming0,
    bus_timing_1: BusTiming1,
    _reserved7: [u8; 0x0c],
    arb_lost_capture: ArbLostCapture,
    error_code_capture: ErrorCodeCapture,
    error_warning_limit: ErrorWarningLimit,
    rx_err_count: RxErrCount,
    tx_err_count: TxErrCount,
    buffer: [Buffer; 13],
    rx_message_count: RxMessageCount,
    _reserved14: [u8; 0x04],
    clock_divider: ClockDivider,
}
impl RegisterBlock {
    #[doc = "0x00 - Operating Mode and Reset Mode Configuration"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - Command Register (Write Only)"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Interrupt Status Register (Reading clears CAN interrupt line)"]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &InterruptEnable {
        &self.interrupt_enable
    }
    #[doc = "0x18 - Bus Timing Register 0"]
    #[inline(always)]
    pub const fn bus_timing_0(&self) -> &BusTiming0 {
        &self.bus_timing_0
    }
    #[doc = "0x1c - Bus Timing Register 1"]
    #[inline(always)]
    pub const fn bus_timing_1(&self) -> &BusTiming1 {
        &self.bus_timing_1
    }
    #[doc = "0x2c - Arbitration Lost Capture Register"]
    #[inline(always)]
    pub const fn arb_lost_capture(&self) -> &ArbLostCapture {
        &self.arb_lost_capture
    }
    #[doc = "0x30 - Error Code Capture Register"]
    #[inline(always)]
    pub const fn error_code_capture(&self) -> &ErrorCodeCapture {
        &self.error_code_capture
    }
    #[doc = "0x34 - Error Warning Limit Register. Defines the threshold for triggering the error warning interrupt."]
    #[inline(always)]
    pub const fn error_warning_limit(&self) -> &ErrorWarningLimit {
        &self.error_warning_limit
    }
    #[doc = "0x38 - Receive Error Counter Register. Reflects the current value of the CAN receive error counter (REC)."]
    #[inline(always)]
    pub const fn rx_err_count(&self) -> &RxErrCount {
        &self.rx_err_count
    }
    #[doc = "0x3c - Transmit Error Counter Register. Reflects the current value of the CAN transmit error counter (TEC)."]
    #[inline(always)]
    pub const fn tx_err_count(&self) -> &TxErrCount {
        &self.tx_err_count
    }
    #[doc = "0x40..0x74 - Multiplexed Data/Filter Buffer Cluster"]
    #[inline(always)]
    pub const fn buffer(&self, n: usize) -> &Buffer {
        &self.buffer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x74 - Multiplexed Data/Filter Buffer Cluster"]
    #[inline(always)]
    pub fn buffer_iter(&self) -> impl Iterator<Item = &Buffer> {
        self.buffer.iter()
    }
    #[doc = "0x74 - Receive Message Counter Register. Indicates the number of messages currently stored in the receive FIFO."]
    #[inline(always)]
    pub const fn rx_message_count(&self) -> &RxMessageCount {
        &self.rx_message_count
    }
    #[doc = "0x7c - Clock Divider and Control"]
    #[inline(always)]
    pub const fn clock_divider(&self) -> &ClockDivider {
        &self.clock_divider
    }
}
#[doc = "MODE (rw) register accessor: Operating Mode and Reset Mode Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Operating Mode and Reset Mode Configuration"]
pub mod mode;
#[doc = "COMMAND (w) register accessor: Command Register (Write Only)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command Register (Write Only)"]
pub mod command;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INTERRUPT (r) register accessor: Interrupt Status Register (Reading clears CAN interrupt line)\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`] module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Interrupt Status Register (Reading clears CAN interrupt line)"]
pub mod interrupt;
#[doc = "INTERRUPT_ENABLE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`] module"]
#[doc(alias = "INTERRUPT_ENABLE")]
pub type InterruptEnable = crate::Reg<interrupt_enable::InterruptEnableSpec>;
#[doc = "Interrupt Enable Register"]
pub mod interrupt_enable;
#[doc = "BUS_TIMING_0 (rw) register accessor: Bus Timing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timing_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timing_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timing_0`] module"]
#[doc(alias = "BUS_TIMING_0")]
pub type BusTiming0 = crate::Reg<bus_timing_0::BusTiming0Spec>;
#[doc = "Bus Timing Register 0"]
pub mod bus_timing_0;
#[doc = "BUS_TIMING_1 (rw) register accessor: Bus Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timing_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timing_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timing_1`] module"]
#[doc(alias = "BUS_TIMING_1")]
pub type BusTiming1 = crate::Reg<bus_timing_1::BusTiming1Spec>;
#[doc = "Bus Timing Register 1"]
pub mod bus_timing_1;
#[doc = "ARB_LOST_CAPTURE (r) register accessor: Arbitration Lost Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_lost_capture::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_lost_capture`] module"]
#[doc(alias = "ARB_LOST_CAPTURE")]
pub type ArbLostCapture = crate::Reg<arb_lost_capture::ArbLostCaptureSpec>;
#[doc = "Arbitration Lost Capture Register"]
pub mod arb_lost_capture;
#[doc = "ERROR_CODE_CAPTURE (r) register accessor: Error Code Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_code_capture::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_code_capture`] module"]
#[doc(alias = "ERROR_CODE_CAPTURE")]
pub type ErrorCodeCapture = crate::Reg<error_code_capture::ErrorCodeCaptureSpec>;
#[doc = "Error Code Capture Register"]
pub mod error_code_capture;
#[doc = "ERROR_WARNING_LIMIT (rw) register accessor: Error Warning Limit Register. Defines the threshold for triggering the error warning interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`error_warning_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_warning_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_warning_limit`] module"]
#[doc(alias = "ERROR_WARNING_LIMIT")]
pub type ErrorWarningLimit = crate::Reg<error_warning_limit::ErrorWarningLimitSpec>;
#[doc = "Error Warning Limit Register. Defines the threshold for triggering the error warning interrupt."]
pub mod error_warning_limit;
#[doc = "RX_ERR_COUNT (rw) register accessor: Receive Error Counter Register. Reflects the current value of the CAN receive error counter (REC).\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_err_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_count`] module"]
#[doc(alias = "RX_ERR_COUNT")]
pub type RxErrCount = crate::Reg<rx_err_count::RxErrCountSpec>;
#[doc = "Receive Error Counter Register. Reflects the current value of the CAN receive error counter (REC)."]
pub mod rx_err_count;
#[doc = "TX_ERR_COUNT (rw) register accessor: Transmit Error Counter Register. Reflects the current value of the CAN transmit error counter (TEC).\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_err_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_err_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_err_count`] module"]
#[doc(alias = "TX_ERR_COUNT")]
pub type TxErrCount = crate::Reg<tx_err_count::TxErrCountSpec>;
#[doc = "Transmit Error Counter Register. Reflects the current value of the CAN transmit error counter (TEC)."]
pub mod tx_err_count;
#[doc = "Multiplexed Data/Filter Buffer Cluster"]
pub use self::buffer::Buffer;
#[doc = r"Cluster"]
#[doc = "Multiplexed Data/Filter Buffer Cluster"]
pub mod buffer;
#[doc = "RX_MESSAGE_COUNT (r) register accessor: Receive Message Counter Register. Indicates the number of messages currently stored in the receive FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_message_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_message_count`] module"]
#[doc(alias = "RX_MESSAGE_COUNT")]
pub type RxMessageCount = crate::Reg<rx_message_count::RxMessageCountSpec>;
#[doc = "Receive Message Counter Register. Indicates the number of messages currently stored in the receive FIFO."]
pub mod rx_message_count;
#[doc = "CLOCK_DIVIDER (rw) register accessor: Clock Divider and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_divider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_divider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_divider`] module"]
#[doc(alias = "CLOCK_DIVIDER")]
pub type ClockDivider = crate::Reg<clock_divider::ClockDividerSpec>;
#[doc = "Clock Divider and Control"]
pub mod clock_divider;
