#[doc = "Register `I2C_FIFO_CONFIG_1` reader"]
pub type R = crate::R<I2cFifoConfig1Spec>;
#[doc = "Register `I2C_FIFO_CONFIG_1` writer"]
pub type W = crate::W<I2cFifoConfig1Spec>;
#[doc = "Field `tx_fifo_cnt` reader - "]
pub type TxFifoCntR = crate::FieldReader;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RxFifoCntR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` reader - "]
pub type TxFifoThR = crate::BitReader;
#[doc = "Field `tx_fifo_th` writer - "]
pub type TxFifoThW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_th` reader - "]
pub type RxFifoThR = crate::BitReader;
#[doc = "Field `rx_fifo_th` writer - "]
pub type RxFifoThW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TxFifoCntR {
        TxFifoCntR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RxFifoCntR {
        RxFifoCntR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TxFifoThR {
        TxFifoThR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RxFifoThR {
        RxFifoThR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TxFifoThW<'_, I2cFifoConfig1Spec> {
        TxFifoThW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RxFifoThW<'_, I2cFifoConfig1Spec> {
        RxFifoThW::new(self, 24)
    }
}
#[doc = "FIFO Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFifoConfig1Spec;
impl crate::RegisterSpec for I2cFifoConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_fifo_config_1::R`](R) reader structure"]
impl crate::Readable for I2cFifoConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_fifo_config_1::W`](W) writer structure"]
impl crate::Writable for I2cFifoConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FIFO_CONFIG_1 to value 0x02"]
impl crate::Resettable for I2cFifoConfig1Spec {
    const RESET_VALUE: u32 = 0x02;
}
