#[doc = "Register `MJPEG_UV_FRAME_WRITE_STATUS` reader"]
pub type R = crate::R<MjpegUvFrameWriteStatusSpec>;
#[doc = "Field `uv_mem_hblk_w` reader - UV memory horizontal block count (write pointer)."]
pub type UvMemHblkWR = crate::FieldReader<u16>;
#[doc = "Field `uv_mem_rnd_w` reader - UV memory round counter (write pointer)."]
pub type UvMemRndWR = crate::FieldReader;
#[doc = "Field `uv_frm_cnt_w` reader - UV frame count (write pointer)."]
pub type UvFrmCntWR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - UV memory horizontal block count (write pointer)."]
    #[inline(always)]
    pub fn uv_mem_hblk_w(&self) -> UvMemHblkWR {
        UvMemHblkWR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:23 - UV memory round counter (write pointer)."]
    #[inline(always)]
    pub fn uv_mem_rnd_w(&self) -> UvMemRndWR {
        UvMemRndWR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - UV frame count (write pointer)."]
    #[inline(always)]
    pub fn uv_frm_cnt_w(&self) -> UvFrmCntWR {
        UvFrmCntWR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "UV Component Frame Write Status: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_write_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegUvFrameWriteStatusSpec;
impl crate::RegisterSpec for MjpegUvFrameWriteStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_uv_frame_write_status::R`](R) reader structure"]
impl crate::Readable for MjpegUvFrameWriteStatusSpec {}
#[doc = "`reset()` method sets MJPEG_UV_FRAME_WRITE_STATUS to value 0"]
impl crate::Resettable for MjpegUvFrameWriteStatusSpec {}
