#[doc = "Register `DEV_SMT` reader"]
pub type R = crate::R<DevSmtSpec>;
#[doc = "Field `SOFMT` reader - SOF Micro Frame Timer"]
pub type SofmtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SOF Micro Frame Timer"]
    #[inline(always)]
    pub fn sofmt(&self) -> SofmtR {
        SofmtR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device SOF Micro Frame Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_smt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevSmtSpec;
impl crate::RegisterSpec for DevSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_smt::R`](R) reader structure"]
impl crate::Readable for DevSmtSpec {}
#[doc = "`reset()` method sets DEV_SMT to value 0"]
impl crate::Resettable for DevSmtSpec {}
