#[doc = "Register `DBI_RDATA` reader"]
pub type R = crate::R<DbiRdataSpec>;
#[doc = "Field `sts_dbi_rdata` reader - Data read from the display module"]
pub type StsDbiRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the display module"]
    #[inline(always)]
    pub fn sts_dbi_rdata(&self) -> StsDbiRdataR {
        StsDbiRdataR::new(self.bits)
    }
}
#[doc = "DBI Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiRdataSpec;
impl crate::RegisterSpec for DbiRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_rdata::R`](R) reader structure"]
impl crate::Readable for DbiRdataSpec {}
#[doc = "`reset()` method sets DBI_RDATA to value 0"]
impl crate::Resettable for DbiRdataSpec {}
