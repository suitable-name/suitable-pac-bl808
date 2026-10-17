#[doc = "Register `IRRX_DATA_COUNT` reader"]
pub type R = crate::R<IrrxDataCountSpec>;
#[doc = "Field `sts_irrx_data_cnt` reader - Count of valid data bits received"]
pub type StsIrrxDataCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Count of valid data bits received"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&self) -> StsIrrxDataCntR {
        StsIrrxDataCntR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "IRRX Received Data Bit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxDataCountSpec;
impl crate::RegisterSpec for IrrxDataCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_data_count::R`](R) reader structure"]
impl crate::Readable for IrrxDataCountSpec {}
#[doc = "`reset()` method sets IRRX_DATA_COUNT to value 0"]
impl crate::Resettable for IrrxDataCountSpec {}
