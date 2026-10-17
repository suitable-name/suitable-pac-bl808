#[doc = "Register `REG` reader"]
pub type R = crate::R<RegSpec>;
#[doc = "Field `value` reader - "]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Touch Raw Data for a single channel\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpec;
impl crate::RegisterSpec for RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg::R`](R) reader structure"]
impl crate::Readable for RegSpec {}
#[doc = "`reset()` method sets REG to value 0"]
impl crate::Resettable for RegSpec {}
