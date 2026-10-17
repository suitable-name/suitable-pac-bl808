#[doc = "Register `SD_RESP_4` reader"]
pub type R = crate::R<SdResp4Spec>;
#[doc = "Field `RESP4` reader - "]
pub type Resp4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp4(&self) -> Resp4R {
        Resp4R::new(self.bits)
    }
}
#[doc = "Response Register 4 (Bits 79:64)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp4Spec;
impl crate::RegisterSpec for SdResp4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_4::R`](R) reader structure"]
impl crate::Readable for SdResp4Spec {}
#[doc = "`reset()` method sets SD_RESP_4 to value 0"]
impl crate::Resettable for SdResp4Spec {}
