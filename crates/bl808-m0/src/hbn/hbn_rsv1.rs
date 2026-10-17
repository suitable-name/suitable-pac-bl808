#[doc = "Register `HBN_RSV1` reader"]
pub type R = crate::R<HbnRsv1Spec>;
#[doc = "Register `HBN_RSV1` writer"]
pub type W = crate::W<HbnRsv1Spec>;
#[doc = "Field `hbn_rsv1` reader - "]
pub type HbnRsv1R = crate::FieldReader<u32>;
#[doc = "Field `hbn_rsv1` writer - "]
pub type HbnRsv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv1(&self) -> HbnRsv1R {
        HbnRsv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv1(&mut self) -> HbnRsv1W<'_, HbnRsv1Spec> {
        HbnRsv1W::new(self, 0)
    }
}
#[doc = "HBN Reserved Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnRsv1Spec;
impl crate::RegisterSpec for HbnRsv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_rsv1::R`](R) reader structure"]
impl crate::Readable for HbnRsv1Spec {}
#[doc = "`write(|w| ..)` method takes [`hbn_rsv1::W`](W) writer structure"]
impl crate::Writable for HbnRsv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_RSV1 to value 0xffff_ffff"]
impl crate::Resettable for HbnRsv1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
