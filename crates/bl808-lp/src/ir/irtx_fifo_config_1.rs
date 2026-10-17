#[doc = "Register `IRTX_FIFO_CONFIG_1` reader"]
pub type R = crate::R<IrtxFifoConfig1Spec>;
#[doc = "Register `IRTX_FIFO_CONFIG_1` writer"]
pub type W = crate::W<IrtxFifoConfig1Spec>;
#[doc = "Field `tx_fifo_cnt` reader - TX FIFO available count"]
pub type TxFifoCntR = crate::FieldReader;
#[doc = "Field `rx_fifo_cnt` reader - RX FIFO available count"]
pub type RxFifoCntR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` reader - TX FIFO threshold"]
pub type TxFifoThR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` writer - TX FIFO threshold"]
pub type TxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rx_fifo_th` reader - RX FIFO threshold"]
pub type RxFifoThR = crate::FieldReader;
#[doc = "Field `rx_fifo_th` writer - RX FIFO threshold"]
pub type RxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - TX FIFO available count"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TxFifoCntR {
        TxFifoCntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - RX FIFO available count"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RxFifoCntR {
        RxFifoCntR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17 - TX FIFO threshold"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TxFifoThR {
        TxFifoThR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - RX FIFO threshold"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RxFifoThR {
        RxFifoThR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - TX FIFO threshold"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TxFifoThW<'_, IrtxFifoConfig1Spec> {
        TxFifoThW::new(self, 16)
    }
    #[doc = "Bits 24:29 - RX FIFO threshold"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RxFifoThW<'_, IrtxFifoConfig1Spec> {
        RxFifoThW::new(self, 24)
    }
}
#[doc = "IRTX FIFO Status and Thresholds 1\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_fifo_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_fifo_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxFifoConfig1Spec;
impl crate::RegisterSpec for IrtxFifoConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_fifo_config_1::R`](R) reader structure"]
impl crate::Readable for IrtxFifoConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_fifo_config_1::W`](W) writer structure"]
impl crate::Writable for IrtxFifoConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRTX_FIFO_CONFIG_1 to value 0x04"]
impl crate::Resettable for IrtxFifoConfig1Spec {
    const RESET_VALUE: u32 = 0x04;
}
