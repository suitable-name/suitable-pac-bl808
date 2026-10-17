#[doc = "Register `SD_RESP_3` reader"]
pub type R = crate::R<SdResp3Spec>;
#[doc = "Field `RESP3` reader - "]
pub type Resp3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp3(&self) -> Resp3R {
        Resp3R::new(self.bits)
    }
}
#[doc = "Response Register 3 (Bits 63:48)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp3Spec;
impl crate::RegisterSpec for SdResp3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_3::R`](R) reader structure"]
impl crate::Readable for SdResp3Spec {}
#[doc = "`reset()` method sets SD_RESP_3 to value 0"]
impl crate::Resettable for SdResp3Spec {}
