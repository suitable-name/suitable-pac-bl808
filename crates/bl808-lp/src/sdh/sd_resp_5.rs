#[doc = "Register `SD_RESP_5` reader"]
pub type R = crate::R<SdResp5Spec>;
#[doc = "Field `RESP5` reader - "]
pub type Resp5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp5(&self) -> Resp5R {
        Resp5R::new(self.bits)
    }
}
#[doc = "Response Register 5 (Bits 95:80)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp5Spec;
impl crate::RegisterSpec for SdResp5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_5::R`](R) reader structure"]
impl crate::Readable for SdResp5Spec {}
#[doc = "`reset()` method sets SD_RESP_5 to value 0"]
impl crate::Resettable for SdResp5Spec {}
