#[doc = "Register `VDO_TOTAL_BYTECOUNT` reader"]
pub type R = crate::R<VdoTotalBytecountSpec>;
#[doc = "Field `bs_tot_bycnt` reader - "]
pub type BsTotBycntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bs_tot_bycnt(&self) -> BsTotBycntR {
        BsTotBycntR::new(self.bits)
    }
}
#[doc = "Main-stream total bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_total_bytecount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoTotalBytecountSpec;
impl crate::RegisterSpec for VdoTotalBytecountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_total_bytecount::R`](R) reader structure"]
impl crate::Readable for VdoTotalBytecountSpec {}
#[doc = "`reset()` method sets VDO_TOTAL_BYTECOUNT to value 0"]
impl crate::Resettable for VdoTotalBytecountSpec {}
