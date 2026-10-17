#[doc = "Register `TZC_UART_CTRL` reader"]
pub type R = crate::R<TzcUartCtrlSpec>;
#[doc = "UART TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_uart_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcUartCtrlSpec;
impl crate::RegisterSpec for TzcUartCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_uart_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcUartCtrlSpec {}
#[doc = "`reset()` method sets TZC_UART_CTRL to value 0"]
impl crate::Resettable for TzcUartCtrlSpec {}
