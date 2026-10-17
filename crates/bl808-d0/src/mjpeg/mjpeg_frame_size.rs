#[doc = "Register `MJPEG_FRAME_SIZE` reader"]
pub type R = crate::R<MjpegFrameSizeSpec>;
#[doc = "Register `MJPEG_FRAME_SIZE` writer"]
pub type W = crate::W<MjpegFrameSizeSpec>;
#[doc = "Field `reg_frame_wblk` reader - Frame width in blocks."]
pub type RegFrameWblkR = crate::FieldReader<u16>;
#[doc = "Field `reg_frame_wblk` writer - Frame width in blocks."]
pub type RegFrameWblkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_frame_hblk` reader - Frame height in blocks."]
pub type RegFrameHblkR = crate::FieldReader<u16>;
#[doc = "Field `reg_frame_hblk` writer - Frame height in blocks."]
pub type RegFrameHblkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Frame width in blocks."]
    #[inline(always)]
    pub fn reg_frame_wblk(&self) -> RegFrameWblkR {
        RegFrameWblkR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Frame height in blocks."]
    #[inline(always)]
    pub fn reg_frame_hblk(&self) -> RegFrameHblkR {
        RegFrameHblkR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Frame width in blocks."]
    #[inline(always)]
    pub fn reg_frame_wblk(&mut self) -> RegFrameWblkW<'_, MjpegFrameSizeSpec> {
        RegFrameWblkW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Frame height in blocks."]
    #[inline(always)]
    pub fn reg_frame_hblk(&mut self) -> RegFrameHblkW<'_, MjpegFrameSizeSpec> {
        RegFrameHblkW::new(self, 16)
    }
}
#[doc = "MJPEG Resolution Configuration (Blocks)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_frame_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegFrameSizeSpec;
impl crate::RegisterSpec for MjpegFrameSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_frame_size::R`](R) reader structure"]
impl crate::Readable for MjpegFrameSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_frame_size::W`](W) writer structure"]
impl crate::Writable for MjpegFrameSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_FRAME_SIZE to value 0x0014_000f"]
impl crate::Resettable for MjpegFrameSizeSpec {
    const RESET_VALUE: u32 = 0x0014_000f;
}
