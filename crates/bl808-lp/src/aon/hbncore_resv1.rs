#[doc = "Register `HBNCORE_RESV1` reader"]
pub type R = crate::R<HbncoreResv1Spec>;
#[doc = "Register `HBNCORE_RESV1` writer"]
pub type W = crate::W<HbncoreResv1Spec>;
#[doc = "Field `hbncore_resv1_data` reader - "]
pub type HbncoreResv1DataR = crate::FieldReader<u32>;
#[doc = "Field `hbncore_resv1_data` writer - "]
pub type HbncoreResv1DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv1_data(&self) -> HbncoreResv1DataR {
        HbncoreResv1DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv1_data(&mut self) -> HbncoreResv1DataW<'_, HbncoreResv1Spec> {
        HbncoreResv1DataW::new(self, 0)
    }
}
#[doc = "Hibernate Core Reserved Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hbncore_resv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbncore_resv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbncoreResv1Spec;
impl crate::RegisterSpec for HbncoreResv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbncore_resv1::R`](R) reader structure"]
impl crate::Readable for HbncoreResv1Spec {}
#[doc = "`write(|w| ..)` method takes [`hbncore_resv1::W`](W) writer structure"]
impl crate::Writable for HbncoreResv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBNCORE_RESV1 to value 0"]
impl crate::Resettable for HbncoreResv1Spec {}
