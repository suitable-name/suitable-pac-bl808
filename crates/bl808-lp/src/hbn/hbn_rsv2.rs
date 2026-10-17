#[doc = "Register `HBN_RSV2` reader"]
pub type R = crate::R<HbnRsv2Spec>;
#[doc = "Register `HBN_RSV2` writer"]
pub type W = crate::W<HbnRsv2Spec>;
#[doc = "Field `hbn_rsv2` reader - "]
pub type HbnRsv2R = crate::FieldReader<u32>;
#[doc = "Field `hbn_rsv2` writer - "]
pub type HbnRsv2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv2(&self) -> HbnRsv2R {
        HbnRsv2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv2(&mut self) -> HbnRsv2W<'_, HbnRsv2Spec> {
        HbnRsv2W::new(self, 0)
    }
}
#[doc = "HBN Reserved Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnRsv2Spec;
impl crate::RegisterSpec for HbnRsv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_rsv2::R`](R) reader structure"]
impl crate::Readable for HbnRsv2Spec {}
#[doc = "`write(|w| ..)` method takes [`hbn_rsv2::W`](W) writer structure"]
impl crate::Writable for HbnRsv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_RSV2 to value 0"]
impl crate::Resettable for HbnRsv2Spec {}
