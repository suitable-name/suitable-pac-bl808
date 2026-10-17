#[doc = "Register `UHS_AUTO_FRESH_4` reader"]
pub type R = crate::R<UhsAutoFresh4Spec>;
#[doc = "Register `UHS_AUTO_FRESH_4` writer"]
pub type W = crate::W<UhsAutoFresh4Spec>;
#[doc = "Field `reg_bust_cycle` reader - Burst cycle count for refresh"]
pub type RegBustCycleR = crate::FieldReader;
#[doc = "Field `reg_bust_cycle` writer - Burst cycle count for refresh"]
pub type RegBustCycleW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Burst cycle count for refresh"]
    #[inline(always)]
    pub fn reg_bust_cycle(&self) -> RegBustCycleR {
        RegBustCycleR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Burst cycle count for refresh"]
    #[inline(always)]
    pub fn reg_bust_cycle(&mut self) -> RegBustCycleW<'_, UhsAutoFresh4Spec> {
        RegBustCycleW::new(self, 0)
    }
}
#[doc = "UHS Auto Refresh Burst Cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsAutoFresh4Spec;
impl crate::RegisterSpec for UhsAutoFresh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_auto_fresh_4::R`](R) reader structure"]
impl crate::Readable for UhsAutoFresh4Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_auto_fresh_4::W`](W) writer structure"]
impl crate::Writable for UhsAutoFresh4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_AUTO_FRESH_4 to value 0x1d"]
impl crate::Resettable for UhsAutoFresh4Spec {
    const RESET_VALUE: u32 = 0x1d;
}
