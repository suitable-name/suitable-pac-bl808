#[doc = "Register `UHS_AUTO_FRESH_1` reader"]
pub type R = crate::R<UhsAutoFresh1Spec>;
#[doc = "Register `UHS_AUTO_FRESH_1` writer"]
pub type W = crate::W<UhsAutoFresh1Spec>;
#[doc = "Field `reg_win_cycle` reader - Refresh window cycle count"]
pub type RegWinCycleR = crate::FieldReader<u32>;
#[doc = "Field `reg_win_cycle` writer - Refresh window cycle count"]
pub type RegWinCycleW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Refresh window cycle count"]
    #[inline(always)]
    pub fn reg_win_cycle(&self) -> RegWinCycleR {
        RegWinCycleR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Refresh window cycle count"]
    #[inline(always)]
    pub fn reg_win_cycle(&mut self) -> RegWinCycleW<'_, UhsAutoFresh1Spec> {
        RegWinCycleW::new(self, 0)
    }
}
#[doc = "UHS Auto Refresh Window Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsAutoFresh1Spec;
impl crate::RegisterSpec for UhsAutoFresh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_auto_fresh_1::R`](R) reader structure"]
impl crate::Readable for UhsAutoFresh1Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_auto_fresh_1::W`](W) writer structure"]
impl crate::Writable for UhsAutoFresh1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_AUTO_FRESH_1 to value 0x0002_7100"]
impl crate::Resettable for UhsAutoFresh1Spec {
    const RESET_VALUE: u32 = 0x0002_7100;
}
