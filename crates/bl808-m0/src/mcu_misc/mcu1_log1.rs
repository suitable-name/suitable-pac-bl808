#[doc = "Register `MCU1_LOG1` reader"]
pub type R = crate::R<Mcu1Log1Spec>;
#[doc = "Field `sts_mcu1_mcause` reader - Machine Cause Register value on exception"]
pub type StsMcu1McauseR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Machine Cause Register value on exception"]
    #[inline(always)]
    pub fn sts_mcu1_mcause(&self) -> StsMcu1McauseR {
        StsMcu1McauseR::new(self.bits)
    }
}
#[doc = "E907 Debug Log 1 (MCAUSE)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcu1Log1Spec;
impl crate::RegisterSpec for Mcu1Log1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log1::R`](R) reader structure"]
impl crate::Readable for Mcu1Log1Spec {}
#[doc = "`reset()` method sets MCU1_LOG1 to value 0"]
impl crate::Resettable for Mcu1Log1Spec {}
