#[doc = "Register `revision` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Field `patch` reader - Patch version number"]
pub type PatchR = crate::FieldReader;
#[doc = "Field `minor` reader - Minor version number"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `major` reader - Major version number"]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Patch version number"]
    #[inline(always)]
    pub fn patch(&self) -> PatchR {
        PatchR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Minor version number"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Major version number"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Peripheral silicon revision\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`reset()` method sets revision to value 0"]
impl crate::Resettable for RevisionSpec {}
