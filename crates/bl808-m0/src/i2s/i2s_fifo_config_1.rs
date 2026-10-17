#[doc = "Register `I2S_FIFO_CONFIG_1` reader"]
pub type R = crate::R<I2sFifoConfig1Spec>;
#[doc = "Register `I2S_FIFO_CONFIG_1` writer"]
pub type W = crate::W<I2sFifoConfig1Spec>;
#[doc = "Field `tx_fifo_cnt` reader - TX FIFO data count"]
pub type TxFifoCntR = crate::FieldReader;
#[doc = "Field `rx_fifo_cnt` reader - RX FIFO data count"]
pub type RxFifoCntR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` reader - TX FIFO threshold"]
pub type TxFifoThR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` writer - TX FIFO threshold"]
pub type TxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rx_fifo_th` reader - RX FIFO threshold"]
pub type RxFifoThR = crate::FieldReader;
#[doc = "Field `rx_fifo_th` writer - RX FIFO threshold"]
pub type RxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - TX FIFO data count"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TxFifoCntR {
        TxFifoCntR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RX FIFO data count"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RxFifoCntR {
        RxFifoCntR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - TX FIFO threshold"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TxFifoThR {
        TxFifoThR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RX FIFO threshold"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RxFifoThR {
        RxFifoThR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - TX FIFO threshold"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TxFifoThW<'_, I2sFifoConfig1Spec> {
        TxFifoThW::new(self, 16)
    }
    #[doc = "Bits 24:27 - RX FIFO threshold"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RxFifoThW<'_, I2sFifoConfig1Spec> {
        RxFifoThW::new(self, 24)
    }
}
#[doc = "I2S FIFO Status and Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_fifo_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_fifo_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sFifoConfig1Spec;
impl crate::RegisterSpec for I2sFifoConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_fifo_config_1::R`](R) reader structure"]
impl crate::Readable for I2sFifoConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2s_fifo_config_1::W`](W) writer structure"]
impl crate::Writable for I2sFifoConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_FIFO_CONFIG_1 to value 0x10"]
impl crate::Resettable for I2sFifoConfig1Spec {
    const RESET_VALUE: u32 = 0x10;
}
