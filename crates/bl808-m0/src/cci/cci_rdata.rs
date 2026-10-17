#[doc = "Register `CCI_RDATA` reader"]
pub type R = crate::R<CciRdataSpec>;
#[doc = "Field `apb_cci_rdata` reader - Data read from the target address"]
pub type ApbCciRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the target address"]
    #[inline(always)]
    pub fn apb_cci_rdata(&self) -> ApbCciRdataR {
        ApbCciRdataR::new(self.bits)
    }
}
#[doc = "CCI Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciRdataSpec;
impl crate::RegisterSpec for CciRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_rdata::R`](R) reader structure"]
impl crate::Readable for CciRdataSpec {}
#[doc = "`reset()` method sets CCI_RDATA to value 0"]
impl crate::Resettable for CciRdataSpec {}
