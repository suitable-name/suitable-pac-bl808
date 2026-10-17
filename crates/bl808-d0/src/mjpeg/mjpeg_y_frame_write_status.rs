#[doc = "Register `MJPEG_Y_FRAME_WRITE_STATUS` reader"]
pub type R = crate::R<MjpegYFrameWriteStatusSpec>;
#[doc = "Field `yy_mem_hblk_w` reader - Y memory horizontal block count (write pointer)."]
pub type YyMemHblkWR = crate::FieldReader<u16>;
#[doc = "Field `yy_mem_rnd_w` reader - Y memory round counter (write pointer)."]
pub type YyMemRndWR = crate::FieldReader;
#[doc = "Field `yy_frm_cnt_w` reader - Y frame count (write pointer)."]
pub type YyFrmCntWR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - Y memory horizontal block count (write pointer)."]
    #[inline(always)]
    pub fn yy_mem_hblk_w(&self) -> YyMemHblkWR {
        YyMemHblkWR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:23 - Y memory round counter (write pointer)."]
    #[inline(always)]
    pub fn yy_mem_rnd_w(&self) -> YyMemRndWR {
        YyMemRndWR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Y frame count (write pointer)."]
    #[inline(always)]
    pub fn yy_frm_cnt_w(&self) -> YyFrmCntWR {
        YyFrmCntWR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Y Component Frame Write Status: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_y_frame_write_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegYFrameWriteStatusSpec;
impl crate::RegisterSpec for MjpegYFrameWriteStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_y_frame_write_status::R`](R) reader structure"]
impl crate::Readable for MjpegYFrameWriteStatusSpec {}
#[doc = "`reset()` method sets MJPEG_Y_FRAME_WRITE_STATUS to value 0"]
impl crate::Resettable for MjpegYFrameWriteStatusSpec {}
