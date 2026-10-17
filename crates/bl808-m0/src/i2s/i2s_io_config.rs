#[doc = "Register `I2S_IO_CONFIG` reader"]
pub type R = crate::R<I2sIoConfigSpec>;
#[doc = "Register `I2S_IO_CONFIG` writer"]
pub type W = crate::W<I2sIoConfigSpec>;
#[doc = "Field `cr_i2s_txd_inv` reader - TXD signal inversion"]
pub type CrI2sTxdInvR = crate::BitReader;
#[doc = "Field `cr_i2s_txd_inv` writer - TXD signal inversion"]
pub type CrI2sTxdInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_rxd_inv` reader - RXD signal inversion"]
pub type CrI2sRxdInvR = crate::BitReader;
#[doc = "Field `cr_i2s_rxd_inv` writer - RXD signal inversion"]
pub type CrI2sRxdInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_fs_inv` reader - FS signal inversion"]
pub type CrI2sFsInvR = crate::BitReader;
#[doc = "Field `cr_i2s_fs_inv` writer - FS signal inversion"]
pub type CrI2sFsInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_bclk_inv` reader - BCLK signal inversion"]
pub type CrI2sBclkInvR = crate::BitReader;
#[doc = "Field `cr_i2s_bclk_inv` writer - BCLK signal inversion"]
pub type CrI2sBclkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_deg_cnt` reader - Deglitch cycle count"]
pub type CrDegCntR = crate::FieldReader;
#[doc = "Field `cr_deg_cnt` writer - Deglitch cycle count"]
pub type CrDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_deg_en` reader - Deglitch enable"]
pub type CrDegEnR = crate::BitReader;
#[doc = "Field `cr_deg_en` writer - Deglitch enable"]
pub type CrDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXD signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_txd_inv(&self) -> CrI2sTxdInvR {
        CrI2sTxdInvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXD signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_rxd_inv(&self) -> CrI2sRxdInvR {
        CrI2sRxdInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FS signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_fs_inv(&self) -> CrI2sFsInvR {
        CrI2sFsInvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BCLK signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_bclk_inv(&self) -> CrI2sBclkInvR {
        CrI2sBclkInvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Deglitch cycle count"]
    #[inline(always)]
    pub fn cr_deg_cnt(&self) -> CrDegCntR {
        CrDegCntR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Deglitch enable"]
    #[inline(always)]
    pub fn cr_deg_en(&self) -> CrDegEnR {
        CrDegEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXD signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_txd_inv(&mut self) -> CrI2sTxdInvW<'_, I2sIoConfigSpec> {
        CrI2sTxdInvW::new(self, 0)
    }
    #[doc = "Bit 1 - RXD signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_rxd_inv(&mut self) -> CrI2sRxdInvW<'_, I2sIoConfigSpec> {
        CrI2sRxdInvW::new(self, 1)
    }
    #[doc = "Bit 2 - FS signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_fs_inv(&mut self) -> CrI2sFsInvW<'_, I2sIoConfigSpec> {
        CrI2sFsInvW::new(self, 2)
    }
    #[doc = "Bit 3 - BCLK signal inversion"]
    #[inline(always)]
    pub fn cr_i2s_bclk_inv(&mut self) -> CrI2sBclkInvW<'_, I2sIoConfigSpec> {
        CrI2sBclkInvW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Deglitch cycle count"]
    #[inline(always)]
    pub fn cr_deg_cnt(&mut self) -> CrDegCntW<'_, I2sIoConfigSpec> {
        CrDegCntW::new(self, 4)
    }
    #[doc = "Bit 7 - Deglitch enable"]
    #[inline(always)]
    pub fn cr_deg_en(&mut self) -> CrDegEnW<'_, I2sIoConfigSpec> {
        CrDegEnW::new(self, 7)
    }
}
#[doc = "I2S Pin Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_io_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_io_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sIoConfigSpec;
impl crate::RegisterSpec for I2sIoConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_io_config::R`](R) reader structure"]
impl crate::Readable for I2sIoConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_io_config::W`](W) writer structure"]
impl crate::Writable for I2sIoConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_IO_CONFIG to value 0"]
impl crate::Resettable for I2sIoConfigSpec {}
