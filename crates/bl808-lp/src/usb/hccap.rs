#[doc = "Register `HCCAP` reader"]
pub type R = crate::R<HccapSpec>;
#[doc = "Field `CAPLENGTH` reader - "]
pub type CaplengthR = crate::FieldReader;
#[doc = "Field `HCIVERSION` reader - "]
pub type HciversionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn caplength(&self) -> CaplengthR {
        CaplengthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn hciversion(&self) -> HciversionR {
        HciversionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Host Controller Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccapSpec;
impl crate::RegisterSpec for HccapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccap::R`](R) reader structure"]
impl crate::Readable for HccapSpec {}
#[doc = "`reset()` method sets HCCAP to value 0"]
impl crate::Resettable for HccapSpec {}
