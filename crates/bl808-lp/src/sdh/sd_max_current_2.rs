#[doc = "Register `SD_MAX_CURRENT_2` reader"]
pub type R = crate::R<SdMaxCurrent2Spec>;
#[doc = "Field `MAX_CUR_18` reader - "]
pub type MaxCur18R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn max_cur_18(&self) -> MaxCur18R {
        MaxCur18R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Host Controller Maximum Current Capabilities 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdMaxCurrent2Spec;
impl crate::RegisterSpec for SdMaxCurrent2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_max_current_2::R`](R) reader structure"]
impl crate::Readable for SdMaxCurrent2Spec {}
#[doc = "`reset()` method sets SD_MAX_CURRENT_2 to value 0"]
impl crate::Resettable for SdMaxCurrent2Spec {}
