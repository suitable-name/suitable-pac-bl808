#[doc = "Register `VDO_S_FRAME_BYTECOUNT` reader"]
pub type R = crate::R<VdoSFrameBytecountSpec>;
#[doc = "Field `s_bs_frm_bycnt` reader - "]
pub type SBsFrmBycntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn s_bs_frm_bycnt(&self) -> SBsFrmBycntR {
        SBsFrmBycntR::new(self.bits)
    }
}
#[doc = "Secondary-stream per-frame bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_s_frame_bytecount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoSFrameBytecountSpec;
impl crate::RegisterSpec for VdoSFrameBytecountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_s_frame_bytecount::R`](R) reader structure"]
impl crate::Readable for VdoSFrameBytecountSpec {}
#[doc = "`reset()` method sets VDO_S_FRAME_BYTECOUNT to value 0"]
impl crate::Resettable for VdoSFrameBytecountSpec {}
