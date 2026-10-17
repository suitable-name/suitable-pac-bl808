#[doc = "Register `MCU1_LOG4` reader"]
pub type R = crate::R<Mcu1Log4Spec>;
#[doc = "Field `sts_mcu1_sp` reader - Stack Pointer value (L bit)"]
pub type StsMcu1SpR = crate::BitReader;
#[doc = "Field `sts_mcu1_pc` reader - Program Counter value (31:1)"]
pub type StsMcu1PcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Stack Pointer value (L bit)"]
    #[inline(always)]
    pub fn sts_mcu1_sp(&self) -> StsMcu1SpR {
        StsMcu1SpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Program Counter value (31:1)"]
    #[inline(always)]
    pub fn sts_mcu1_pc(&self) -> StsMcu1PcR {
        StsMcu1PcR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "E907 Debug Log 4 (Stack Pointer / Program Counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcu1Log4Spec;
impl crate::RegisterSpec for Mcu1Log4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log4::R`](R) reader structure"]
impl crate::Readable for Mcu1Log4Spec {}
#[doc = "`reset()` method sets MCU1_LOG4 to value 0"]
impl crate::Resettable for Mcu1Log4Spec {}
