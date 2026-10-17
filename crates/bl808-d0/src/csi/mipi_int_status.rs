#[doc = "Register `MIPI_INT_STATUS` reader"]
pub type R = crate::R<MipiIntStatusSpec>;
#[doc = "Field `csi_int_status` reader - "]
pub type CsiIntStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn csi_int_status(&self) -> CsiIntStatusR {
        CsiIntStatusR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "MIPI CSI Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiIntStatusSpec;
impl crate::RegisterSpec for MipiIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_int_status::R`](R) reader structure"]
impl crate::Readable for MipiIntStatusSpec {}
#[doc = "`reset()` method sets MIPI_INT_STATUS to value 0"]
impl crate::Resettable for MipiIntStatusSpec {}
