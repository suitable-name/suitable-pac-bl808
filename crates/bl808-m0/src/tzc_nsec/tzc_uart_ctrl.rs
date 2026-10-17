#[doc = "Register `TZC_UART_CTRL` reader"]
pub type R = crate::R<TzcUartCtrlSpec>;
#[doc = "Register `TZC_UART_CTRL` writer"]
pub type W = crate::W<TzcUartCtrlSpec>;
impl W {}
#[doc = "UART Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_uart_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_uart_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcUartCtrlSpec;
impl crate::RegisterSpec for TzcUartCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_uart_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcUartCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_uart_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcUartCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_UART_CTRL to value 0"]
impl crate::Resettable for TzcUartCtrlSpec {}
