#[doc = "Register `UHS_AUTO_FRESH_3` reader"]
pub type R = crate::R<UhsAutoFresh3Spec>;
#[doc = "Register `UHS_AUTO_FRESH_3` writer"]
pub type W = crate::W<UhsAutoFresh3Spec>;
#[doc = "Field `reg_auto_ref_thre` reader - Auto refresh threshold"]
pub type RegAutoRefThreR = crate::FieldReader<u16>;
#[doc = "Field `reg_auto_ref_thre` writer - Auto refresh threshold"]
pub type RegAutoRefThreW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `auto_refresh_level` reader - Current auto refresh level"]
pub type AutoRefreshLevelR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Auto refresh threshold"]
    #[inline(always)]
    pub fn reg_auto_ref_thre(&self) -> RegAutoRefThreR {
        RegAutoRefThreR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Current auto refresh level"]
    #[inline(always)]
    pub fn auto_refresh_level(&self) -> AutoRefreshLevelR {
        AutoRefreshLevelR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Auto refresh threshold"]
    #[inline(always)]
    pub fn reg_auto_ref_thre(&mut self) -> RegAutoRefThreW<'_, UhsAutoFresh3Spec> {
        RegAutoRefThreW::new(self, 0)
    }
}
#[doc = "UHS Auto Refresh Threshold and Level\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsAutoFresh3Spec;
impl crate::RegisterSpec for UhsAutoFresh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_auto_fresh_3::R`](R) reader structure"]
impl crate::Readable for UhsAutoFresh3Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_auto_fresh_3::W`](W) writer structure"]
impl crate::Writable for UhsAutoFresh3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_AUTO_FRESH_3 to value 0"]
impl crate::Resettable for UhsAutoFresh3Spec {}
