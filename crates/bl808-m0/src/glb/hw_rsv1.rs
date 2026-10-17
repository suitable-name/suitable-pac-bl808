#[doc = "Register `HW_RSV1` reader"]
pub type R = crate::R<HwRsv1Spec>;
#[doc = "Register `HW_RSV1` writer"]
pub type W = crate::W<HwRsv1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Hardware Reserved Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwRsv1Spec;
impl crate::RegisterSpec for HwRsv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_rsv1::R`](R) reader structure"]
impl crate::Readable for HwRsv1Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_rsv1::W`](W) writer structure"]
impl crate::Writable for HwRsv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_RSV1 to value 0"]
impl crate::Resettable for HwRsv1Spec {}
