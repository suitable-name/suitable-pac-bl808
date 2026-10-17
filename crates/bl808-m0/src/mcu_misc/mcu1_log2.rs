#[doc = "Register `MCU1_LOG2` reader"]
pub type R = crate::R<Mcu1Log2Spec>;
#[doc = "Field `sts_mcu1_mintstatus` reader - Machine Interrupt Status Register value"]
pub type StsMcu1MintstatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Machine Interrupt Status Register value"]
    #[inline(always)]
    pub fn sts_mcu1_mintstatus(&self) -> StsMcu1MintstatusR {
        StsMcu1MintstatusR::new(self.bits)
    }
}
#[doc = "E907 Debug Log 2 (MINTSTATUS)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcu1Log2Spec;
impl crate::RegisterSpec for Mcu1Log2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log2::R`](R) reader structure"]
impl crate::Readable for Mcu1Log2Spec {}
#[doc = "`reset()` method sets MCU1_LOG2 to value 0"]
impl crate::Resettable for Mcu1Log2Spec {}
