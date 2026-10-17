#[doc = "Register `SF_IF_STATUS_1` reader"]
pub type R = crate::R<SfIfStatus1Spec>;
#[doc = "Field `sf_if_status_1` reader - "]
pub type SfIfStatus1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_1(&self) -> SfIfStatus1R {
        SfIfStatus1R::new(self.bits)
    }
}
#[doc = "Interface Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfStatus1Spec;
impl crate::RegisterSpec for SfIfStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_status_1::R`](R) reader structure"]
impl crate::Readable for SfIfStatus1Spec {}
#[doc = "`reset()` method sets SF_IF_STATUS_1 to value 0"]
impl crate::Resettable for SfIfStatus1Spec {}
