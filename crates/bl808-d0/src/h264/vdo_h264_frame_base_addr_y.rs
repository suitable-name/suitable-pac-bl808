#[doc = "Register `VDO_H264_FRAME_BASE_ADDR_Y` reader"]
pub type R = crate::R<VdoH264FrameBaseAddrYSpec>;
#[doc = "Register `VDO_H264_FRAME_BASE_ADDR_Y` writer"]
pub type W = crate::W<VdoH264FrameBaseAddrYSpec>;
#[doc = "Field `cfg_frame_base_addr_y` reader - "]
pub type CfgFrameBaseAddrYR = crate::FieldReader<u32>;
#[doc = "Field `cfg_frame_base_addr_y` writer - "]
pub type CfgFrameBaseAddrYW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_frame_base_addr_y(&self) -> CfgFrameBaseAddrYR {
        CfgFrameBaseAddrYR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_frame_base_addr_y(&mut self) -> CfgFrameBaseAddrYW<'_, VdoH264FrameBaseAddrYSpec> {
        CfgFrameBaseAddrYW::new(self, 0)
    }
}
#[doc = "Main-stream source Y frame buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_frame_base_addr_y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_frame_base_addr_y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264FrameBaseAddrYSpec;
impl crate::RegisterSpec for VdoH264FrameBaseAddrYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_frame_base_addr_y::R`](R) reader structure"]
impl crate::Readable for VdoH264FrameBaseAddrYSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_frame_base_addr_y::W`](W) writer structure"]
impl crate::Writable for VdoH264FrameBaseAddrYSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_FRAME_BASE_ADDR_Y to value 0"]
impl crate::Resettable for VdoH264FrameBaseAddrYSpec {}
