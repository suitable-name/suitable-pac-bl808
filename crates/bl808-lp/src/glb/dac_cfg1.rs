#[doc = "Register `DAC_CFG1` reader"]
pub type R = crate::R<DacCfg1Spec>;
#[doc = "Register `DAC_CFG1` writer"]
pub type W = crate::W<DacCfg1Spec>;
#[doc = "Field `gpdac_a_en` reader - "]
pub type GpdacAEnR = crate::BitReader;
#[doc = "Field `gpdac_a_en` writer - "]
pub type GpdacAEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_ioa_en` reader - "]
pub type GpdacIoaEnR = crate::BitReader;
#[doc = "Field `gpdac_ioa_en` writer - "]
pub type GpdacIoaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_a_rng` reader - "]
pub type GpdacARngR = crate::FieldReader;
#[doc = "Field `gpdac_a_rng` writer - "]
pub type GpdacARngW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpdac_a_outmux` reader - "]
pub type GpdacAOutmuxR = crate::FieldReader;
#[doc = "Field `gpdac_a_outmux` writer - "]
pub type GpdacAOutmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&self) -> GpdacAEnR {
        GpdacAEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&self) -> GpdacIoaEnR {
        GpdacIoaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&self) -> GpdacARngR {
        GpdacARngR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&self) -> GpdacAOutmuxR {
        GpdacAOutmuxR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&mut self) -> GpdacAEnW<'_, DacCfg1Spec> {
        GpdacAEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&mut self) -> GpdacIoaEnW<'_, DacCfg1Spec> {
        GpdacIoaEnW::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&mut self) -> GpdacARngW<'_, DacCfg1Spec> {
        GpdacARngW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&mut self) -> GpdacAOutmuxW<'_, DacCfg1Spec> {
        GpdacAOutmuxW::new(self, 20)
    }
}
#[doc = "GPDAC Configuration 1 (Channel A)\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCfg1Spec;
impl crate::RegisterSpec for DacCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cfg1::R`](R) reader structure"]
impl crate::Readable for DacCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_cfg1::W`](W) writer structure"]
impl crate::Writable for DacCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_CFG1 to value 0"]
impl crate::Resettable for DacCfg1Spec {}
