#[doc = "Register `UART_FIFO_CONFIG_1` reader"]
pub type R = crate::R<UartFifoConfig1Spec>;
#[doc = "Register `UART_FIFO_CONFIG_1` writer"]
pub type W = crate::W<UartFifoConfig1Spec>;
#[doc = "Field `tx_fifo_cnt` reader - "]
pub type TxFifoCntR = crate::FieldReader;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RxFifoCntR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` reader - "]
pub type TxFifoThR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` writer - "]
pub type TxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rx_fifo_th` reader - "]
pub type RxFifoThR = crate::FieldReader;
#[doc = "Field `rx_fifo_th` writer - "]
pub type RxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TxFifoCntR {
        TxFifoCntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RxFifoCntR {
        RxFifoCntR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TxFifoThR {
        TxFifoThR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RxFifoThR {
        RxFifoThR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TxFifoThW<'_, UartFifoConfig1Spec> {
        TxFifoThW::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RxFifoThW<'_, UartFifoConfig1Spec> {
        RxFifoThW::new(self, 24)
    }
}
#[doc = "UART FIFO Threshold and Count\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFifoConfig1Spec;
impl crate::RegisterSpec for UartFifoConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_fifo_config_1::R`](R) reader structure"]
impl crate::Readable for UartFifoConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_fifo_config_1::W`](W) writer structure"]
impl crate::Writable for UartFifoConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_FIFO_CONFIG_1 to value 0x20"]
impl crate::Resettable for UartFifoConfig1Spec {
    const RESET_VALUE: u32 = 0x20;
}
