#[doc = "Register `SD_RESP_2` reader"]
pub type R = crate::R<SdResp2Spec>;
#[doc = "Field `RESP2` reader - "]
pub type Resp2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp2(&self) -> Resp2R {
        Resp2R::new(self.bits)
    }
}
#[doc = "Response Register 2 (Bits 47:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp2Spec;
impl crate::RegisterSpec for SdResp2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_2::R`](R) reader structure"]
impl crate::Readable for SdResp2Spec {}
#[doc = "`reset()` method sets SD_RESP_2 to value 0"]
impl crate::Resettable for SdResp2Spec {}
