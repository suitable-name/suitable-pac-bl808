#[doc = "Register `VDO_H264_S_FRAME_BASE_ADDR_UV` reader"]
pub type R = crate::R<VdoH264SFrameBaseAddrUvSpec>;
#[doc = "Register `VDO_H264_S_FRAME_BASE_ADDR_UV` writer"]
pub type W = crate::W<VdoH264SFrameBaseAddrUvSpec>;
#[doc = "Field `cfg_s_frame_base_addr_uv` reader - "]
pub type CfgSFrameBaseAddrUvR = crate::FieldReader<u32>;
#[doc = "Field `cfg_s_frame_base_addr_uv` writer - "]
pub type CfgSFrameBaseAddrUvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_s_frame_base_addr_uv(&self) -> CfgSFrameBaseAddrUvR {
        CfgSFrameBaseAddrUvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_s_frame_base_addr_uv(
        &mut self,
    ) -> CfgSFrameBaseAddrUvW<'_, VdoH264SFrameBaseAddrUvSpec> {
        CfgSFrameBaseAddrUvW::new(self, 0)
    }
}
#[doc = "Secondary-stream source UV frame buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_frame_base_addr_uv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_frame_base_addr_uv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264SFrameBaseAddrUvSpec;
impl crate::RegisterSpec for VdoH264SFrameBaseAddrUvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_s_frame_base_addr_uv::R`](R) reader structure"]
impl crate::Readable for VdoH264SFrameBaseAddrUvSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_s_frame_base_addr_uv::W`](W) writer structure"]
impl crate::Writable for VdoH264SFrameBaseAddrUvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_S_FRAME_BASE_ADDR_UV to value 0"]
impl crate::Resettable for VdoH264SFrameBaseAddrUvSpec {}
