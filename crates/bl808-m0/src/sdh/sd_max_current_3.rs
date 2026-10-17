#[doc = "Register `SD_MAX_CURRENT_3` reader"]
pub type R = crate::R<SdMaxCurrent3Spec>;
#[doc = "Maximum Current Capabilities Register 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdMaxCurrent3Spec;
impl crate::RegisterSpec for SdMaxCurrent3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_max_current_3::R`](R) reader structure"]
impl crate::Readable for SdMaxCurrent3Spec {}
#[doc = "`reset()` method sets SD_MAX_CURRENT_3 to value 0"]
impl crate::Resettable for SdMaxCurrent3Spec {}
