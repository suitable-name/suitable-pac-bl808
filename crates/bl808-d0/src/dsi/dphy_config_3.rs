#[doc = "Register `DPHY_CONFIG_3` reader"]
pub type R = crate::R<DphyConfig3Spec>;
#[doc = "Register `DPHY_CONFIG_3` writer"]
pub type W = crate::W<DphyConfig3Spec>;
#[doc = "Field `reg_time_ta_go` reader - "]
pub type RegTimeTaGoR = crate::FieldReader;
#[doc = "Field `reg_time_ta_go` writer - "]
pub type RegTimeTaGoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_ta_get` reader - "]
pub type RegTimeTaGetR = crate::FieldReader;
#[doc = "Field `reg_time_ta_get` writer - "]
pub type RegTimeTaGetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_reqrdy` reader - "]
pub type RegTimeReqrdyR = crate::FieldReader;
#[doc = "Field `reg_time_reqrdy` writer - "]
pub type RegTimeReqrdyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_lpx` reader - "]
pub type RegTimeLpxR = crate::FieldReader;
#[doc = "Field `reg_time_lpx` writer - "]
pub type RegTimeLpxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_time_ta_go(&self) -> RegTimeTaGoR {
        RegTimeTaGoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_time_ta_get(&self) -> RegTimeTaGetR {
        RegTimeTaGetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_time_reqrdy(&self) -> RegTimeReqrdyR {
        RegTimeReqrdyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_time_lpx(&self) -> RegTimeLpxR {
        RegTimeLpxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_time_ta_go(&mut self) -> RegTimeTaGoW<'_, DphyConfig3Spec> {
        RegTimeTaGoW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_time_ta_get(&mut self) -> RegTimeTaGetW<'_, DphyConfig3Spec> {
        RegTimeTaGetW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_time_reqrdy(&mut self) -> RegTimeReqrdyW<'_, DphyConfig3Spec> {
        RegTimeReqrdyW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_time_lpx(&mut self) -> RegTimeLpxW<'_, DphyConfig3Spec> {
        RegTimeLpxW::new(self, 24)
    }
}
#[doc = "DPHY Data Lane Timing 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig3Spec;
impl crate::RegisterSpec for DphyConfig3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_3::R`](R) reader structure"]
impl crate::Readable for DphyConfig3Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_3::W`](W) writer structure"]
impl crate::Writable for DphyConfig3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_3 to value 0x0300_0f0c"]
impl crate::Resettable for DphyConfig3Spec {
    const RESET_VALUE: u32 = 0x0300_0f0c;
}
