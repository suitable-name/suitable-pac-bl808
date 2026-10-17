#[doc = "Register `VDO_FRAME_BYTECOUNT` reader"]
pub type R = crate::R<VdoFrameBytecountSpec>;
#[doc = "Field `bs_frm_bycnt` reader - "]
pub type BsFrmBycntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bs_frm_bycnt(&self) -> BsFrmBycntR {
        BsFrmBycntR::new(self.bits)
    }
}
#[doc = "Main-stream per-frame bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_frame_bytecount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoFrameBytecountSpec;
impl crate::RegisterSpec for VdoFrameBytecountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_frame_bytecount::R`](R) reader structure"]
impl crate::Readable for VdoFrameBytecountSpec {}
#[doc = "`reset()` method sets VDO_FRAME_BYTECOUNT to value 0"]
impl crate::Resettable for VdoFrameBytecountSpec {}
