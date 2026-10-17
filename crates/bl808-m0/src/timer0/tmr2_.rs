#[doc = "Register `TMR2_%s` reader"]
pub type R = crate::R<Tmr2_Spec>;
#[doc = "Register `TMR2_%s` writer"]
pub type W = crate::W<Tmr2_Spec>;
#[doc = "Field `match_val` reader - "]
pub type MatchValR = crate::FieldReader<u32>;
#[doc = "Field `match_val` writer - "]
pub type MatchValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn match_val(&self) -> MatchValR {
        MatchValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn match_val(&mut self) -> MatchValW<'_, Tmr2_Spec> {
        MatchValW::new(self, 0)
    }
}
#[doc = "Timer 2 Match Value %s\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr2_Spec;
impl crate::RegisterSpec for Tmr2_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2_::R`](R) reader structure"]
impl crate::Readable for Tmr2_Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr2_::W`](W) writer structure"]
impl crate::Writable for Tmr2_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR2_%s to value 0xffff_ffff"]
impl crate::Resettable for Tmr2_Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
