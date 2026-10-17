#[doc = "Register `RX_CFG_REG` reader"]
pub type R = crate::R<RxCfgRegSpec>;
#[doc = "Register `RX_CFG_REG` writer"]
pub type W = crate::W<RxCfgRegSpec>;
#[doc = "Field `SDCLK_SEL0` reader - "]
pub type SdclkSel0R = crate::FieldReader;
#[doc = "Field `SDCLK_SEL0` writer - "]
pub type SdclkSel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLK_SEL1` reader - "]
pub type SdclkSel1R = crate::FieldReader;
#[doc = "Field `SDCLK_SEL1` writer - "]
pub type SdclkSel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLK_DELAY` reader - "]
pub type SdclkDelayR = crate::FieldReader<u16>;
#[doc = "Field `SDCLK_DELAY` writer - "]
pub type SdclkDelayW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TUNING_DLY_INC` reader - "]
pub type TuningDlyIncR = crate::FieldReader<u16>;
#[doc = "Field `TUNING_DLY_INC` writer - "]
pub type TuningDlyIncW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdclk_sel0(&self) -> SdclkSel0R {
        SdclkSel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sdclk_sel1(&self) -> SdclkSel1R {
        SdclkSel1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:17"]
    #[inline(always)]
    pub fn sdclk_delay(&self) -> SdclkDelayR {
        SdclkDelayR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn tuning_dly_inc(&self) -> TuningDlyIncR {
        TuningDlyIncR::new(((self.bits >> 18) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdclk_sel0(&mut self) -> SdclkSel0W<'_, RxCfgRegSpec> {
        SdclkSel0W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sdclk_sel1(&mut self) -> SdclkSel1W<'_, RxCfgRegSpec> {
        SdclkSel1W::new(self, 2)
    }
    #[doc = "Bits 8:17"]
    #[inline(always)]
    pub fn sdclk_delay(&mut self) -> SdclkDelayW<'_, RxCfgRegSpec> {
        SdclkDelayW::new(self, 8)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn tuning_dly_inc(&mut self) -> TuningDlyIncW<'_, RxCfgRegSpec> {
        TuningDlyIncW::new(self, 18)
    }
}
#[doc = "Vendor specific RX sampling clock delay and tuning configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cfg_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_cfg_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCfgRegSpec;
impl crate::RegisterSpec for RxCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_cfg_reg::R`](R) reader structure"]
impl crate::Readable for RxCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_cfg_reg::W`](W) writer structure"]
impl crate::Writable for RxCfgRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CFG_REG to value 0"]
impl crate::Resettable for RxCfgRegSpec {}
