#[doc = "Register `SD_RESP_6` reader"]
pub type R = crate::R<SdResp6Spec>;
#[doc = "Field `RESP6` reader - "]
pub type Resp6R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp6(&self) -> Resp6R {
        Resp6R::new(self.bits)
    }
}
#[doc = "Response Register 6 (Bits 111:96)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp6Spec;
impl crate::RegisterSpec for SdResp6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_6::R`](R) reader structure"]
impl crate::Readable for SdResp6Spec {}
#[doc = "`reset()` method sets SD_RESP_6 to value 0"]
impl crate::Resettable for SdResp6Spec {}
