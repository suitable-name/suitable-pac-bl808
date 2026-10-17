#[doc = "Register `DPHY_CONFIG_1` reader"]
pub type R = crate::R<DphyConfig1Spec>;
#[doc = "Register `DPHY_CONFIG_1` writer"]
pub type W = crate::W<DphyConfig1Spec>;
#[doc = "Field `reg_time_ck_zero` reader - "]
pub type RegTimeCkZeroR = crate::FieldReader;
#[doc = "Field `reg_time_ck_zero` writer - "]
pub type RegTimeCkZeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_ck_trail` reader - "]
pub type RegTimeCkTrailR = crate::FieldReader;
#[doc = "Field `reg_time_ck_trail` writer - "]
pub type RegTimeCkTrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_ck_exit` reader - "]
pub type RegTimeCkExitR = crate::FieldReader;
#[doc = "Field `reg_time_ck_exit` writer - "]
pub type RegTimeCkExitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_en_lprx_at_ulps` reader - "]
pub type RegEnLprxAtUlpsR = crate::BitReader;
#[doc = "Field `reg_en_lprx_at_ulps` writer - "]
pub type RegEnLprxAtUlpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_time_ck_zero(&self) -> RegTimeCkZeroR {
        RegTimeCkZeroR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_time_ck_trail(&self) -> RegTimeCkTrailR {
        RegTimeCkTrailR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_time_ck_exit(&self) -> RegTimeCkExitR {
        RegTimeCkExitR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_en_lprx_at_ulps(&self) -> RegEnLprxAtUlpsR {
        RegEnLprxAtUlpsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_time_ck_zero(&mut self) -> RegTimeCkZeroW<'_, DphyConfig1Spec> {
        RegTimeCkZeroW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_time_ck_trail(&mut self) -> RegTimeCkTrailW<'_, DphyConfig1Spec> {
        RegTimeCkTrailW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_time_ck_exit(&mut self) -> RegTimeCkExitW<'_, DphyConfig1Spec> {
        RegTimeCkExitW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_en_lprx_at_ulps(&mut self) -> RegEnLprxAtUlpsW<'_, DphyConfig1Spec> {
        RegEnLprxAtUlpsW::new(self, 24)
    }
}
#[doc = "DPHY Clock Lane Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig1Spec;
impl crate::RegisterSpec for DphyConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_1::R`](R) reader structure"]
impl crate::Readable for DphyConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_1::W`](W) writer structure"]
impl crate::Writable for DphyConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_1 to value 0x0005_030f"]
impl crate::Resettable for DphyConfig1Spec {
    const RESET_VALUE: u32 = 0x0005_030f;
}
