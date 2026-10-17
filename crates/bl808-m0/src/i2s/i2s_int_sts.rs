#[doc = "Register `I2S_INT_STS` reader"]
pub type R = crate::R<I2sIntStsSpec>;
#[doc = "Register `I2S_INT_STS` writer"]
pub type W = crate::W<I2sIntStsSpec>;
#[doc = "Field `i2s_txf_int` reader - TX FIFO request interrupt status"]
pub type I2sTxfIntR = crate::BitReader;
#[doc = "Field `i2s_rxf_int` reader - RX FIFO request interrupt status"]
pub type I2sRxfIntR = crate::BitReader;
#[doc = "Field `i2s_fer_int` reader - FIFO error interrupt status"]
pub type I2sFerIntR = crate::BitReader;
#[doc = "Field `cr_i2s_txf_mask` reader - TX FIFO interrupt mask"]
pub type CrI2sTxfMaskR = crate::BitReader;
#[doc = "Field `cr_i2s_txf_mask` writer - TX FIFO interrupt mask"]
pub type CrI2sTxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_rxf_mask` reader - RX FIFO interrupt mask"]
pub type CrI2sRxfMaskR = crate::BitReader;
#[doc = "Field `cr_i2s_rxf_mask` writer - RX FIFO interrupt mask"]
pub type CrI2sRxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_fer_mask` reader - FIFO error interrupt mask"]
pub type CrI2sFerMaskR = crate::BitReader;
#[doc = "Field `cr_i2s_fer_mask` writer - FIFO error interrupt mask"]
pub type CrI2sFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_txf_en` reader - TX FIFO interrupt enable"]
pub type CrI2sTxfEnR = crate::BitReader;
#[doc = "Field `cr_i2s_txf_en` writer - TX FIFO interrupt enable"]
pub type CrI2sTxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_rxf_en` reader - RX FIFO interrupt enable"]
pub type CrI2sRxfEnR = crate::BitReader;
#[doc = "Field `cr_i2s_rxf_en` writer - RX FIFO interrupt enable"]
pub type CrI2sRxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_fer_en` reader - FIFO error interrupt enable"]
pub type CrI2sFerEnR = crate::BitReader;
#[doc = "Field `cr_i2s_fer_en` writer - FIFO error interrupt enable"]
pub type CrI2sFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX FIFO request interrupt status"]
    #[inline(always)]
    pub fn i2s_txf_int(&self) -> I2sTxfIntR {
        I2sTxfIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO request interrupt status"]
    #[inline(always)]
    pub fn i2s_rxf_int(&self) -> I2sRxfIntR {
        I2sRxfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO error interrupt status"]
    #[inline(always)]
    pub fn i2s_fer_int(&self) -> I2sFerIntR {
        I2sFerIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - TX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2s_txf_mask(&self) -> CrI2sTxfMaskR {
        CrI2sTxfMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2s_rxf_mask(&self) -> CrI2sRxfMaskR {
        CrI2sRxfMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO error interrupt mask"]
    #[inline(always)]
    pub fn cr_i2s_fer_mask(&self) -> CrI2sFerMaskR {
        CrI2sFerMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - TX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2s_txf_en(&self) -> CrI2sTxfEnR {
        CrI2sTxfEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2s_rxf_en(&self) -> CrI2sRxfEnR {
        CrI2sRxfEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn cr_i2s_fer_en(&self) -> CrI2sFerEnR {
        CrI2sFerEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - TX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2s_txf_mask(&mut self) -> CrI2sTxfMaskW<'_, I2sIntStsSpec> {
        CrI2sTxfMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - RX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2s_rxf_mask(&mut self) -> CrI2sRxfMaskW<'_, I2sIntStsSpec> {
        CrI2sRxfMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - FIFO error interrupt mask"]
    #[inline(always)]
    pub fn cr_i2s_fer_mask(&mut self) -> CrI2sFerMaskW<'_, I2sIntStsSpec> {
        CrI2sFerMaskW::new(self, 10)
    }
    #[doc = "Bit 24 - TX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2s_txf_en(&mut self) -> CrI2sTxfEnW<'_, I2sIntStsSpec> {
        CrI2sTxfEnW::new(self, 24)
    }
    #[doc = "Bit 25 - RX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2s_rxf_en(&mut self) -> CrI2sRxfEnW<'_, I2sIntStsSpec> {
        CrI2sRxfEnW::new(self, 25)
    }
    #[doc = "Bit 26 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn cr_i2s_fer_en(&mut self) -> CrI2sFerEnW<'_, I2sIntStsSpec> {
        CrI2sFerEnW::new(self, 26)
    }
}
#[doc = "I2S Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sIntStsSpec;
impl crate::RegisterSpec for I2sIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_int_sts::R`](R) reader structure"]
impl crate::Readable for I2sIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_int_sts::W`](W) writer structure"]
impl crate::Writable for I2sIntStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_INT_STS to value 0x0700_0701"]
impl crate::Resettable for I2sIntStsSpec {
    const RESET_VALUE: u32 = 0x0700_0701;
}
