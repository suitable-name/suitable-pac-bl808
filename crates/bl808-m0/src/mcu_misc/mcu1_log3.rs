#[doc = "Register `MCU1_LOG3` reader"]
pub type R = crate::R<Mcu1Log3Spec>;
#[doc = "Field `sts_mcu1_mstatus` reader - Machine Status Register value"]
pub type StsMcu1MstatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Machine Status Register value"]
    #[inline(always)]
    pub fn sts_mcu1_mstatus(&self) -> StsMcu1MstatusR {
        StsMcu1MstatusR::new(self.bits)
    }
}
#[doc = "E907 Debug Log 3 (MSTATUS)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcu1Log3Spec;
impl crate::RegisterSpec for Mcu1Log3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log3::R`](R) reader structure"]
impl crate::Readable for Mcu1Log3Spec {}
#[doc = "`reset()` method sets MCU1_LOG3 to value 0"]
impl crate::Resettable for Mcu1Log3Spec {}
