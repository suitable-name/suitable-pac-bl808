#[doc = "Register `HBN_RSV0` reader"]
pub type R = crate::R<HbnRsv0Spec>;
#[doc = "Register `HBN_RSV0` writer"]
pub type W = crate::W<HbnRsv0Spec>;
#[doc = "Field `hbn_rsv0` reader - "]
pub type HbnRsv0R = crate::FieldReader<u32>;
#[doc = "Field `hbn_rsv0` writer - "]
pub type HbnRsv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv0(&self) -> HbnRsv0R {
        HbnRsv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv0(&mut self) -> HbnRsv0W<'_, HbnRsv0Spec> {
        HbnRsv0W::new(self, 0)
    }
}
#[doc = "HBN Reserved Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnRsv0Spec;
impl crate::RegisterSpec for HbnRsv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_rsv0::R`](R) reader structure"]
impl crate::Readable for HbnRsv0Spec {}
#[doc = "`write(|w| ..)` method takes [`hbn_rsv0::W`](W) writer structure"]
impl crate::Writable for HbnRsv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_RSV0 to value 0"]
impl crate::Resettable for HbnRsv0Spec {}
