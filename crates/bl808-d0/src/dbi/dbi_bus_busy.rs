#[doc = "Register `DBI_BUS_BUSY` reader"]
pub type R = crate::R<DbiBusBusySpec>;
#[doc = "Field `sts_dbi_bus_busy` reader - "]
pub type StsDbiBusBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_dbi_bus_busy(&self) -> StsDbiBusBusyR {
        StsDbiBusBusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "DBI Bus Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_bus_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiBusBusySpec;
impl crate::RegisterSpec for DbiBusBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_bus_busy::R`](R) reader structure"]
impl crate::Readable for DbiBusBusySpec {}
#[doc = "`reset()` method sets DBI_BUS_BUSY to value 0"]
impl crate::Resettable for DbiBusBusySpec {}
