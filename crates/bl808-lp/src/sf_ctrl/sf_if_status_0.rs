#[doc = "Register `SF_IF_STATUS_0` reader"]
pub type R = crate::R<SfIfStatus0Spec>;
#[doc = "Field `sf_if_status_0` reader - "]
pub type SfIfStatus0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_0(&self) -> SfIfStatus0R {
        SfIfStatus0R::new(self.bits)
    }
}
#[doc = "Interface Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfStatus0Spec;
impl crate::RegisterSpec for SfIfStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_status_0::R`](R) reader structure"]
impl crate::Readable for SfIfStatus0Spec {}
#[doc = "`reset()` method sets SF_IF_STATUS_0 to value 0"]
impl crate::Resettable for SfIfStatus0Spec {}
