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
    #[doc = "16 - Bus Matrix MCU Bus Error"]
    BMX_MCU_BUS_ERR = 16,
    #[doc = "17 - Bus Matrix MCU Timeout"]
    BMX_MCU_TO = 17,
    #[doc = "19 - Inter-Processor Communication M0"]
    IPC_M0 = 19,
    #[doc = "20 - Audio Subsystem Interrupt"]
    AUDIO = 20,
    #[doc = "21 - RF Top Interrupt 0"]
    RF_TOP_INT0 = 21,
    #[doc = "22 - RF Top Interrupt 1"]
    RF_TOP_INT1 = 22,
    #[doc = "23 - LZ4 Decompressor Interrupt"]
    LZ4D = 23,
    #[doc = "24 - Gauge Interface Interrupt"]
    GAUGE_ITF = 24,
    #[doc = "25 - Security Engine ID1 SHA/AES/TRNG/PKA/GMAC Interrupt"]
    SEC_ENG_ID1_SHA_AES_TRNG_PKA_GMAC = 25,
    #[doc = "26 - Security Engine ID0 SHA/AES/TRNG/PKA/GMAC Interrupt"]
    SEC_ENG_ID0_SHA_AES_TRNG_PKA_GMAC = 26,
    #[doc = "27 - Security Engine ID1 CDET Interrupt"]
    SEC_ENG_ID1_CDET = 27,
    #[doc = "28 - Security Engine ID0 CDET Interrupt"]
    SEC_ENG_ID0_CDET = 28,
    #[doc = "29 - Serial Flash Control ID1 Interrupt"]
    SF_CTRL_ID1 = 29,
    #[doc = "30 - Serial Flash Control ID0 Interrupt"]
    SF_CTRL_ID0 = 30,
    #[doc = "31 - DMA0 All Channels Interrupt"]
    DMA0_ALL = 31,
    #[doc = "32 - DMA1_ALL"]
    DMA1_ALL = 32,
    #[doc = "33 - SD Host Controller Interrupt"]
    SDH = 33,
    #[doc = "34 - MM System All Interrupt"]
    MM_ALL = 34,
    #[doc = "35 - IRTX"]
    IRTX = 35,
    #[doc = "36 - IRRX"]
    IRRX = 36,
    #[doc = "37 - USB 2.0 OTG Interrupt"]
    USB = 37,
    #[doc = "38 - Audio PDM Touch Interrupt"]
    AUPDM_TOUCH = 38,
    #[doc = "40 - Ethernet MAC Interrupt"]
    EMAC = 40,
    #[doc = "41 - GPADC DMA Interrupt"]
    GPADC_DMA = 41,
    #[doc = "42 - EFUSE Interrupt"]
    EFUSE = 42,
    #[doc = "43 - SPI0 Interrupt"]
    SPI0 = 43,
    #[doc = "44 - UART0 Interrupt"]
    UART0 = 44,
    #[doc = "45 - UART1"]
    UART1 = 45,
    #[doc = "46 - UART2"]
    UART2 = 46,
    #[doc = "47 - GPIO DMA FIFO Interrupt"]
    GPIO_DMA = 47,
    #[doc = "48 - I2C0 Interrupt"]
    I2C0 = 48,
    #[doc = "49 - PWM Interrupt"]
    PWM = 49,
    #[doc = "50 - Inter-Processor Communication Reserved"]
    IPC_RSVD = 50,
    #[doc = "51 - Inter-Processor Communication LP"]
    IPC_LP = 51,
    #[doc = "52 - TIMER0_CH0"]
    TIMER0_CH0 = 52,
    #[doc = "53 - TIMER0_CH1"]
    TIMER0_CH1 = 53,
    #[doc = "54 - TIMER0_WDT"]
    TIMER0_WDT = 54,
    #[doc = "55 - I2C1"]
    I2C1 = 55,
    #[doc = "57 - Analog OCP Output to CPU 0"]
    ANA_OCP_OUT_TO_CPU_0 = 57,
    #[doc = "58 - Analog OCP Output to CPU 1"]
    ANA_OCP_OUT_TO_CPU_1 = 58,
    #[doc = "59 - Analog OCP Output to CPU 2"]
    ANA_OCP_OUT_TO_CPU_2 = 59,
    #[doc = "60 - Unified GPIO Interrupt (All Pins)"]
    GPIO_INT0 = 60,
    #[doc = "66 - PDS Wakeup Interrupt"]
    PDS_WAKEUP = 66,
    #[doc = "67 - HBN Out 0 Interrupt (Wakeup)"]
    HBN_OUT0 = 67,
    #[doc = "68 - HBN Out 1 Interrupt (Wakeup)"]
    HBN_OUT1 = 68,
    #[doc = "69 - Brown-Out Reset Interrupt"]
    BOR = 69,
}
