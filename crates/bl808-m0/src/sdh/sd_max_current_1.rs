#[doc = "Register `SD_MAX_CURRENT_1` reader"]
pub type R = crate::R<SdMaxCurrent1Spec>;
#[doc = "Field `MAX_CUR_33` reader - "]
pub type MaxCur33R = crate::FieldReader;
#[doc = "Field `MAX_CUR_30` reader - "]
pub type MaxCur30R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn max_cur_33(&self) -> MaxCur33R {
        MaxCur33R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn max_cur_30(&self) -> MaxCur30R {
        MaxCur30R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Maximum Current Capabilities 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_max_current_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdMaxCurrent1Spec;
impl crate::RegisterSpec for SdMaxCurrent1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_max_current_1::R`](R) reader structure"]
impl crate::Readable for SdMaxCurrent1Spec {}
#[doc = "`reset()` method sets SD_MAX_CURRENT_1 to value 0"]
impl crate::Resettable for SdMaxCurrent1Spec {}
