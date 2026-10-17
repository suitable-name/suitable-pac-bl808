#[doc = "Register `TPLVR2` reader"]
pub type R = crate::R<Tplvr2Spec>;
#[doc = "Register `TPLVR2` writer"]
pub type W = crate::W<Tplvr2Spec>;
#[doc = "Field `tplvr2` reader - "]
pub type Tplvr2R = crate::FieldReader<u32>;
#[doc = "Field `tplvr2` writer - "]
pub type Tplvr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tplvr2(&self) -> Tplvr2R {
        Tplvr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tplvr2(&mut self) -> Tplvr2W<'_, Tplvr2Spec> {
        Tplvr2W::new(self, 0)
    }
}
#[doc = "Timer 2 Pre-Load Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tplvr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplvr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tplvr2Spec;
impl crate::RegisterSpec for Tplvr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tplvr2::R`](R) reader structure"]
impl crate::Readable for Tplvr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tplvr2::W`](W) writer structure"]
impl crate::Writable for Tplvr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPLVR2 to value 0"]
impl crate::Resettable for Tplvr2Spec {}
