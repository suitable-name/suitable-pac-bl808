#[doc = "Register `SD_RESP_1` reader"]
pub type R = crate::R<SdResp1Spec>;
#[doc = "Field `RESP1` reader - "]
pub type Resp1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp1(&self) -> Resp1R {
        Resp1R::new(self.bits)
    }
}
#[doc = "Response Register 1 (Bits 31:16)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp1Spec;
impl crate::RegisterSpec for SdResp1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_1::R`](R) reader structure"]
impl crate::Readable for SdResp1Spec {}
#[doc = "`reset()` method sets SD_RESP_1 to value 0"]
impl crate::Resettable for SdResp1Spec {}
