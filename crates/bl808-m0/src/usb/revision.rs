#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Field `REVISION` reader - "]
pub type RevisionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(self.bits)
    }
}
#[doc = "USB Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for RevisionSpec {}
