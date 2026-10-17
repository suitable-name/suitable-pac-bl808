#[doc = "Register `VDO_S_TOTAL_BYTECOUNT` reader"]
pub type R = crate::R<VdoSTotalBytecountSpec>;
#[doc = "Field `s_bs_tot_bycnt` reader - "]
pub type SBsTotBycntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn s_bs_tot_bycnt(&self) -> SBsTotBycntR {
        SBsTotBycntR::new(self.bits)
    }
}
#[doc = "Secondary-stream total bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_s_total_bytecount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoSTotalBytecountSpec;
impl crate::RegisterSpec for VdoSTotalBytecountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_s_total_bytecount::R`](R) reader structure"]
impl crate::Readable for VdoSTotalBytecountSpec {}
#[doc = "`reset()` method sets VDO_S_TOTAL_BYTECOUNT to value 0"]
impl crate::Resettable for VdoSTotalBytecountSpec {}
