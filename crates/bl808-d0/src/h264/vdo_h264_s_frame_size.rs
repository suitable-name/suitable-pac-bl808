#[doc = "Register `VDO_H264_S_FRAME_SIZE` reader"]
pub type R = crate::R<VdoH264SFrameSizeSpec>;
#[doc = "Register `VDO_H264_S_FRAME_SIZE` writer"]
pub type W = crate::W<VdoH264SFrameSizeSpec>;
#[doc = "Field `cfg_s_frame_width` reader - Secondary frame width in pixels"]
pub type CfgSFrameWidthR = crate::FieldReader<u16>;
#[doc = "Field `cfg_s_frame_width` writer - Secondary frame width in pixels"]
pub type CfgSFrameWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `cfg_s_frame_height` reader - Secondary frame height in pixels"]
pub type CfgSFrameHeightR = crate::FieldReader<u16>;
#[doc = "Field `cfg_s_frame_height` writer - Secondary frame height in pixels"]
pub type CfgSFrameHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Secondary frame width in pixels"]
    #[inline(always)]
    pub fn cfg_s_frame_width(&self) -> CfgSFrameWidthR {
        CfgSFrameWidthR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Secondary frame height in pixels"]
    #[inline(always)]
    pub fn cfg_s_frame_height(&self) -> CfgSFrameHeightR {
        CfgSFrameHeightR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Secondary frame width in pixels"]
    #[inline(always)]
    pub fn cfg_s_frame_width(&mut self) -> CfgSFrameWidthW<'_, VdoH264SFrameSizeSpec> {
        CfgSFrameWidthW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Secondary frame height in pixels"]
    #[inline(always)]
    pub fn cfg_s_frame_height(&mut self) -> CfgSFrameHeightW<'_, VdoH264SFrameSizeSpec> {
        CfgSFrameHeightW::new(self, 16)
    }
}
#[doc = "Secondary-stream frame resolution (in pixels)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_frame_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_frame_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264SFrameSizeSpec;
impl crate::RegisterSpec for VdoH264SFrameSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_s_frame_size::R`](R) reader structure"]
impl crate::Readable for VdoH264SFrameSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_s_frame_size::W`](W) writer structure"]
impl crate::Writable for VdoH264SFrameSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_S_FRAME_SIZE to value 0"]
impl crate::Resettable for VdoH264SFrameSizeSpec {}
