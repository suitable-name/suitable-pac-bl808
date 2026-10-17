#[doc = "Register `HBNCORE_RESV0` reader"]
pub type R = crate::R<HbncoreResv0Spec>;
#[doc = "Register `HBNCORE_RESV0` writer"]
pub type W = crate::W<HbncoreResv0Spec>;
#[doc = "Field `hbncore_resv0_data` reader - "]
pub type HbncoreResv0DataR = crate::FieldReader<u32>;
#[doc = "Field `hbncore_resv0_data` writer - "]
pub type HbncoreResv0DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv0_data(&self) -> HbncoreResv0DataR {
        HbncoreResv0DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv0_data(&mut self) -> HbncoreResv0DataW<'_, HbncoreResv0Spec> {
        HbncoreResv0DataW::new(self, 0)
    }
}
#[doc = "Hibernate Core Reserved Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hbncore_resv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbncore_resv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbncoreResv0Spec;
impl crate::RegisterSpec for HbncoreResv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbncore_resv0::R`](R) reader structure"]
impl crate::Readable for HbncoreResv0Spec {}
#[doc = "`write(|w| ..)` method takes [`hbncore_resv0::W`](W) writer structure"]
impl crate::Writable for HbncoreResv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBNCORE_RESV0 to value 0"]
impl crate::Resettable for HbncoreResv0Spec {}
