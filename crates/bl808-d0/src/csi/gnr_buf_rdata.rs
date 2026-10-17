#[doc = "Register `GNR_BUF_RDATA` reader"]
pub type R = crate::R<GnrBufRdataSpec>;
#[doc = "Field `gnr_buf_rdata` reader - Read data from generic short/long packet buffer"]
pub type GnrBufRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read data from generic short/long packet buffer"]
    #[inline(always)]
    pub fn gnr_buf_rdata(&self) -> GnrBufRdataR {
        GnrBufRdataR::new(self.bits)
    }
}
#[doc = "Generic Buffer Read Data\n\nYou can [`read`](crate::Reg::read) this register and get [`gnr_buf_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnrBufRdataSpec;
impl crate::RegisterSpec for GnrBufRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnr_buf_rdata::R`](R) reader structure"]
impl crate::Readable for GnrBufRdataSpec {}
#[doc = "`reset()` method sets GNR_BUF_RDATA to value 0"]
impl crate::Resettable for GnrBufRdataSpec {}
