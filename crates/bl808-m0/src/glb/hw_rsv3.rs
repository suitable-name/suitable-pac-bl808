#[doc = "Register `HW_RSV3` reader"]
pub type R = crate::R<HwRsv3Spec>;
#[doc = "Register `HW_RSV3` writer"]
pub type W = crate::W<HwRsv3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Hardware Reserved Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwRsv3Spec;
impl crate::RegisterSpec for HwRsv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_rsv3::R`](R) reader structure"]
impl crate::Readable for HwRsv3Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_rsv3::W`](W) writer structure"]
impl crate::Writable for HwRsv3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_RSV3 to value 0"]
impl crate::Resettable for HwRsv3Spec {}
