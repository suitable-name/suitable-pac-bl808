#[doc = "Register `dbg_1` reader"]
pub type R = crate::R<Dbg1Spec>;
#[doc = "Field `aud_test_read` reader - Audio test read data results"]
pub type AudTestReadR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Audio test read data results"]
    #[inline(always)]
    pub fn aud_test_read(&self) -> AudTestReadR {
        AudTestReadR::new(self.bits)
    }
}
#[doc = "Debug Register 1: Audio internal data read port\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg1Spec;
impl crate::RegisterSpec for Dbg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_1::R`](R) reader structure"]
impl crate::Readable for Dbg1Spec {}
#[doc = "`reset()` method sets dbg_1 to value 0"]
impl crate::Resettable for Dbg1Spec {}
