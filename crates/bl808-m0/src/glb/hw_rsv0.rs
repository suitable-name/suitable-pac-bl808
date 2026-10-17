#[doc = "Register `HW_RSV0` reader"]
pub type R = crate::R<HwRsv0Spec>;
#[doc = "Register `HW_RSV0` writer"]
pub type W = crate::W<HwRsv0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Hardware Reserved Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwRsv0Spec;
impl crate::RegisterSpec for HwRsv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_rsv0::R`](R) reader structure"]
impl crate::Readable for HwRsv0Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_rsv0::W`](W) writer structure"]
impl crate::Writable for HwRsv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_RSV0 to value 0"]
impl crate::Resettable for HwRsv0Spec {}
