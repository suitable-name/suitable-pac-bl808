#[doc = "Register `DAC_CFG2` reader"]
pub type R = crate::R<DacCfg2Spec>;
#[doc = "Register `DAC_CFG2` writer"]
pub type W = crate::W<DacCfg2Spec>;
#[doc = "Field `gpdac_b_en` reader - "]
pub type GpdacBEnR = crate::BitReader;
#[doc = "Field `gpdac_b_en` writer - "]
pub type GpdacBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_iob_en` reader - "]
pub type GpdacIobEnR = crate::BitReader;
#[doc = "Field `gpdac_iob_en` writer - "]
pub type GpdacIobEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_b_rng` reader - "]
pub type GpdacBRngR = crate::FieldReader;
#[doc = "Field `gpdac_b_rng` writer - "]
pub type GpdacBRngW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_b_outmux` reader - "]
pub type GpdacBOutmuxR = crate::FieldReader;
#[doc = "Field `gpdac_b_outmux` writer - "]
pub type GpdacBOutmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&self) -> GpdacBEnR {
        GpdacBEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&self) -> GpdacIobEnR {
        GpdacIobEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&self) -> GpdacBRngR {
        GpdacBRngR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_b_outmux(&self) -> GpdacBOutmuxR {
        GpdacBOutmuxR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_b_en(&mut self) -> GpdacBEnW<'_, DacCfg2Spec> {
        GpdacBEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_iob_en(&mut self) -> GpdacIobEnW<'_, DacCfg2Spec> {
        GpdacIobEnW::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_b_rng(&mut self) -> GpdacBRngW<'_, DacCfg2Spec> {
        GpdacBRngW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_b_outmux(&mut self) -> GpdacBOutmuxW<'_, DacCfg2Spec> {
        GpdacBOutmuxW::new(self, 20)
    }
}
#[doc = "GPDAC Configuration 2 (Channel B)\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCfg2Spec;
impl crate::RegisterSpec for DacCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cfg2::R`](R) reader structure"]
impl crate::Readable for DacCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_cfg2::W`](W) writer structure"]
impl crate::Writable for DacCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_CFG2 to value 0"]
impl crate::Resettable for DacCfg2Spec {}
