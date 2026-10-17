#[doc = "Register `SD_RESP_7` reader"]
pub type R = crate::R<SdResp7Spec>;
#[doc = "Field `RESP7` reader - "]
pub type Resp7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp7(&self) -> Resp7R {
        Resp7R::new(self.bits)
    }
}
#[doc = "Response Register 7 (Bits 127:112)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp7Spec;
impl crate::RegisterSpec for SdResp7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_7::R`](R) reader structure"]
impl crate::Readable for SdResp7Spec {}
#[doc = "`reset()` method sets SD_RESP_7 to value 0"]
impl crate::Resettable for SdResp7Spec {}
