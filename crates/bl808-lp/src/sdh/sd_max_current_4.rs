#[doc = "Register `SD_MAX_CURRENT_4` reader"]
pub type R = crate::R<SdMaxCurrent4Spec>;
#[doc = "Maximum Current Capabilities Register 4 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdMaxCurrent4Spec;
impl crate::RegisterSpec for SdMaxCurrent4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_max_current_4::R`](R) reader structure"]
impl crate::Readable for SdMaxCurrent4Spec {}
#[doc = "`reset()` method sets SD_MAX_CURRENT_4 to value 0"]
impl crate::Resettable for SdMaxCurrent4Spec {}
