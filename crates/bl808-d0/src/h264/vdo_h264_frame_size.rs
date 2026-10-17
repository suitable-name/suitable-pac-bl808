#[doc = "Register `VDO_H264_FRAME_SIZE` reader"]
pub type R = crate::R<VdoH264FrameSizeSpec>;
#[doc = "Register `VDO_H264_FRAME_SIZE` writer"]
pub type W = crate::W<VdoH264FrameSizeSpec>;
#[doc = "Field `cfg_frame_width` reader - Frame width in pixels"]
pub type CfgFrameWidthR = crate::FieldReader<u16>;
#[doc = "Field `cfg_frame_width` writer - Frame width in pixels"]
pub type CfgFrameWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `cfg_frame_height` reader - Frame height in pixels"]
pub type CfgFrameHeightR = crate::FieldReader<u16>;
#[doc = "Field `cfg_frame_height` writer - Frame height in pixels"]
pub type CfgFrameHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame width in pixels"]
    #[inline(always)]
    pub fn cfg_frame_width(&self) -> CfgFrameWidthR {
        CfgFrameWidthR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Frame height in pixels"]
    #[inline(always)]
    pub fn cfg_frame_height(&self) -> CfgFrameHeightR {
        CfgFrameHeightR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame width in pixels"]
    #[inline(always)]
    pub fn cfg_frame_width(&mut self) -> CfgFrameWidthW<'_, VdoH264FrameSizeSpec> {
        CfgFrameWidthW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Frame height in pixels"]
    #[inline(always)]
    pub fn cfg_frame_height(&mut self) -> CfgFrameHeightW<'_, VdoH264FrameSizeSpec> {
        CfgFrameHeightW::new(self, 16)
    }
}
#[doc = "Main-stream frame resolution (in pixels)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_frame_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_frame_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264FrameSizeSpec;
impl crate::RegisterSpec for VdoH264FrameSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_frame_size::R`](R) reader structure"]
impl crate::Readable for VdoH264FrameSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_frame_size::W`](W) writer structure"]
impl crate::Writable for VdoH264FrameSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_FRAME_SIZE to value 0"]
impl crate::Resettable for VdoH264FrameSizeSpec {}
