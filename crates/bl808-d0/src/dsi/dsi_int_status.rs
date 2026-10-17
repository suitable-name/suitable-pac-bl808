#[doc = "Register `DSI_INT_STATUS` reader"]
pub type R = crate::R<DsiIntStatusSpec>;
#[doc = "Field `dsi_int_status` reader - "]
pub type DsiIntStatusR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dsi_int_status(&self) -> DsiIntStatusR {
        DsiIntStatusR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIntStatusSpec;
impl crate::RegisterSpec for DsiIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_int_status::R`](R) reader structure"]
impl crate::Readable for DsiIntStatusSpec {}
#[doc = "`reset()` method sets DSI_INT_STATUS to value 0"]
impl crate::Resettable for DsiIntStatusSpec {}
