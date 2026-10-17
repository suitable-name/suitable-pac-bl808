pub use riscv::interrupt::Exception;
pub use riscv::interrupt::Interrupt as CoreInterrupt;
pub use riscv::{
    interrupt::{disable, enable, free, nested},
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
#[doc = r" External interrupts. These interrupts are handled by the external peripherals."]
# [riscv :: pac_enum (unsafe ExternalInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalInterrupt {
    #[doc = "16 - BMX_DSP_BUS_ERR"]
    BMX_DSP_BUS_ERR = 16,
    #[doc = "20 - UART3"]
    UART3 = 20,
    #[doc = "21 - I2C2"]
    I2C2 = 21,
    #[doc = "22 - I2C3"]
    I2C3 = 22,
    #[doc = "23 - SPI1"]
    SPI1 = 23,
    #[doc = "26 - SEOF_INT0"]
    SEOF_INT0 = 26,
    #[doc = "27 - SEOF_INT1"]
    SEOF_INT1 = 27,
    #[doc = "28 - SEOF_INT2"]
    SEOF_INT2 = 28,
    #[doc = "29 - DVP2BUS_INT0"]
    DVP2BUS_INT0 = 29,
    #[doc = "30 - DVP2BUS_INT1"]
    DVP2BUS_INT1 = 30,
    #[doc = "31 - DVP2BUS_INT2"]
    DVP2BUS_INT2 = 31,
    #[doc = "32 - DVP2BUS_INT3"]
    DVP2BUS_INT3 = 32,
    #[doc = "33 - H264 main-stream bitstream interrupt (H264_BS_IRQn = IRQ_NUM_BASE+17)"]
    H264_BS = 33,
    #[doc = "34 - H264 main-stream frame-done interrupt (H264_FRAME_IRQn = IRQ_NUM_BASE+18)"]
    H264_FRAME = 34,
    #[doc = "35 - H264 main-stream sequence-done interrupt (H264_SEQ_DONE_IRQn = IRQ_NUM_BASE+19)"]
    H264_SEQ_DONE = 35,
    #[doc = "36 - MJPEG Interrupt"]
    MJPEG = 36,
    #[doc = "37 - H264 secondary-stream bitstream interrupt (H264_S_BS_IRQn = IRQ_NUM_BASE+21)"]
    H264_S_BS = 37,
    #[doc = "38 - H264 secondary-stream frame-done interrupt (H264_S_FRAME_IRQn = IRQ_NUM_BASE+22)"]
    H264_S_FRAME = 38,
    #[doc = "39 - H264 secondary-stream sequence-done interrupt (H264_S_SEQ_DONE_IRQn = IRQ_NUM_BASE+23)"]
    H264_S_SEQ_DONE = 39,
    #[doc = "40 - DMA2_INT0"]
    DMA2_INT0 = 40,
    #[doc = "41 - DMA2_INT1"]
    DMA2_INT1 = 41,
    #[doc = "42 - DMA2_INT2"]
    DMA2_INT2 = 42,
    #[doc = "43 - DMA2_INT3"]
    DMA2_INT3 = 43,
    #[doc = "44 - DMA2_INT4"]
    DMA2_INT4 = 44,
    #[doc = "45 - DMA2_INT5"]
    DMA2_INT5 = 45,
    #[doc = "46 - DMA2_INT6"]
    DMA2_INT6 = 46,
    #[doc = "47 - DMA2_INT7"]
    DMA2_INT7 = 47,
    #[doc = "53 - MIPI_CSI"]
    MIPI_CSI = 53,
    #[doc = "54 - Inter-Processor Communication D0"]
    IPC_D0 = 54,
    #[doc = "55 - AI Process Unit Interrupt"]
    APU = 55,
    #[doc = "56 - MJPEG Decoder Interrupt"]
    MJDEC = 56,
    #[doc = "57 - DVP2BUS_INT4"]
    DVP2BUS_INT4 = 57,
    #[doc = "58 - DVP2BUS_INT5"]
    DVP2BUS_INT5 = 58,
    #[doc = "59 - DVP2BUS_INT6"]
    DVP2BUS_INT6 = 59,
    #[doc = "60 - DVP2BUS_INT7"]
    DVP2BUS_INT7 = 60,
    #[doc = "61 - DMA2D_INT0"]
    DMA2D_INT0 = 61,
    #[doc = "62 - DMA2D_INT1"]
    DMA2D_INT1 = 62,
    #[doc = "63 - Display Subsystem Interrupt"]
    DISPLAY = 63,
    #[doc = "64 - PWM Interrupt"]
    PWM = 64,
    #[doc = "69 - Display DBI Interrupt"]
    DBI = 69,
    #[doc = "75 - MIPI DSI Interrupt"]
    MIPI_DSI = 75,
    #[doc = "77 - TIMER1_CH0"]
    TIMER1_CH0 = 77,
    #[doc = "78 - TIMER1_CH1"]
    TIMER1_CH1 = 78,
    #[doc = "79 - TIMER1_WDT"]
    TIMER1_WDT = 79,
    #[doc = "80 - Audio Subsystem Interrupt"]
    AUDIO = 80,
    #[doc = "82 - PDS Interrupt"]
    PDS = 82,
}
