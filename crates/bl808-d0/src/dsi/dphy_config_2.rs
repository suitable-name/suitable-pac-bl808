#[doc = "Register `DPHY_CONFIG_2` reader"]
pub type R = crate::R<DphyConfig2Spec>;
#[doc = "Register `DPHY_CONFIG_2` writer"]
pub type W = crate::W<DphyConfig2Spec>;
#[doc = "Field `reg_time_hs_zero` reader - "]
pub type RegTimeHsZeroR = crate::FieldReader;
#[doc = "Field `reg_time_hs_zero` writer - "]
pub type RegTimeHsZeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_hs_trail` reader - "]
pub type RegTimeHsTrailR = crate::FieldReader;
#[doc = "Field `reg_time_hs_trail` writer - "]
pub type RegTimeHsTrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_hs_prep` reader - "]
pub type RegTimeHsPrepR = crate::FieldReader;
#[doc = "Field `reg_time_hs_prep` writer - "]
pub type RegTimeHsPrepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_hs_exit` reader - "]
pub type RegTimeHsExitR = crate::FieldReader;
#[doc = "Field `reg_time_hs_exit` writer - "]
pub type RegTimeHsExitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_time_hs_zero(&self) -> RegTimeHsZeroR {
        RegTimeHsZeroR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_time_hs_trail(&self) -> RegTimeHsTrailR {
        RegTimeHsTrailR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_time_hs_prep(&self) -> RegTimeHsPrepR {
        RegTimeHsPrepR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_time_hs_exit(&self) -> RegTimeHsExitR {
        RegTimeHsExitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_time_hs_zero(&mut self) -> RegTimeHsZeroW<'_, DphyConfig2Spec> {
        RegTimeHsZeroW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_time_hs_trail(&mut self) -> RegTimeHsTrailW<'_, DphyConfig2Spec> {
        RegTimeHsTrailW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_time_hs_prep(&mut self) -> RegTimeHsPrepW<'_, DphyConfig2Spec> {
        RegTimeHsPrepW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_time_hs_exit(&mut self) -> RegTimeHsExitW<'_, DphyConfig2Spec> {
        RegTimeHsExitW::new(self, 24)
    }
}
#[doc = "DPHY Data Lane Timing 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig2Spec;
impl crate::RegisterSpec for DphyConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_2::R`](R) reader structure"]
impl crate::Readable for DphyConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_2::W`](W) writer structure"]
impl crate::Writable for DphyConfig2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_2 to value 0x0502_0305"]
impl crate::Resettable for DphyConfig2Spec {
    const RESET_VALUE: u32 = 0x0502_0305;
}
