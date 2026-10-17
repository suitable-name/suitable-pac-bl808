#[doc = "Register `SD_RESP_0` reader"]
pub type R = crate::R<SdResp0Spec>;
#[doc = "Field `RESP0` reader - "]
pub type Resp0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn resp0(&self) -> Resp0R {
        Resp0R::new(self.bits)
    }
}
#[doc = "Response Register 0 (Bits 15:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_resp_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdResp0Spec;
impl crate::RegisterSpec for SdResp0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_resp_0::R`](R) reader structure"]
impl crate::Readable for SdResp0Spec {}
#[doc = "`reset()` method sets SD_RESP_0 to value 0"]
impl crate::Resettable for SdResp0Spec {}
