#[doc = "Register `DEV_SFN` reader"]
pub type R = crate::R<DevSfnSpec>;
#[doc = "Field `SOFN` reader - SOF Frame Number"]
pub type SofnR = crate::FieldReader<u16>;
#[doc = "Field `USOFN` reader - Micro SOF Frame Number"]
pub type UsofnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - SOF Frame Number"]
    #[inline(always)]
    pub fn sofn(&self) -> SofnR {
        SofnR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:13 - Micro SOF Frame Number"]
    #[inline(always)]
    pub fn usofn(&self) -> UsofnR {
        UsofnR::new(((self.bits >> 11) & 7) as u8)
    }
}
#[doc = "Device SOF Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_sfn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevSfnSpec;
impl crate::RegisterSpec for DevSfnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_sfn::R`](R) reader structure"]
impl crate::Readable for DevSfnSpec {}
#[doc = "`reset()` method sets DEV_SFN to value 0"]
impl crate::Resettable for DevSfnSpec {}
