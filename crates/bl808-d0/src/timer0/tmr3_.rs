#[doc = "Register `TMR3_%s` reader"]
pub type R = crate::R<Tmr3_Spec>;
#[doc = "Register `TMR3_%s` writer"]
pub type W = crate::W<Tmr3_Spec>;
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
    pub fn match_val(&mut self) -> MatchValW<'_, Tmr3_Spec> {
        MatchValW::new(self, 0)
    }
}
#[doc = "Timer 3 Match Value %s\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr3_Spec;
impl crate::RegisterSpec for Tmr3_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr3_::R`](R) reader structure"]
impl crate::Readable for Tmr3_Spec {}
#[doc = "`write(|w| ..)` method takes [`tmr3_::W`](W) writer structure"]
impl crate::Writable for Tmr3_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR3_%s to value 0xffff_ffff"]
impl crate::Resettable for Tmr3_Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
