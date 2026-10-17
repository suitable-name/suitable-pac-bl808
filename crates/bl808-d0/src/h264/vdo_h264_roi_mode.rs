#[doc = "Register `VDO_H264_ROI_MODE` reader"]
pub type R = crate::R<VdoH264RoiModeSpec>;
#[doc = "Register `VDO_H264_ROI_MODE` writer"]
pub type W = crate::W<VdoH264RoiModeSpec>;
#[doc = "Field `cfg_roi_en` reader - ROI enable bits (one bit per ROI region, 8 regions)"]
pub type CfgRoiEnR = crate::FieldReader;
#[doc = "Field `cfg_roi_en` writer - ROI enable bits (one bit per ROI region, 8 regions)"]
pub type CfgRoiEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cfg_roi_upd` reader - ROI configuration update trigger"]
pub type CfgRoiUpdR = crate::BitReader;
#[doc = "Field `cfg_roi_upd` writer - ROI configuration update trigger"]
pub type CfgRoiUpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - ROI enable bits (one bit per ROI region, 8 regions)"]
    #[inline(always)]
    pub fn cfg_roi_en(&self) -> CfgRoiEnR {
        CfgRoiEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - ROI configuration update trigger"]
    #[inline(always)]
    pub fn cfg_roi_upd(&self) -> CfgRoiUpdR {
        CfgRoiUpdR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ROI enable bits (one bit per ROI region, 8 regions)"]
    #[inline(always)]
    pub fn cfg_roi_en(&mut self) -> CfgRoiEnW<'_, VdoH264RoiModeSpec> {
        CfgRoiEnW::new(self, 0)
    }
    #[doc = "Bit 8 - ROI configuration update trigger"]
    #[inline(always)]
    pub fn cfg_roi_upd(&mut self) -> CfgRoiUpdW<'_, VdoH264RoiModeSpec> {
        CfgRoiUpdW::new(self, 8)
    }
}
#[doc = "ROI mode control: enable bits and update trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_roi_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_roi_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264RoiModeSpec;
impl crate::RegisterSpec for VdoH264RoiModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_roi_mode::R`](R) reader structure"]
impl crate::Readable for VdoH264RoiModeSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_roi_mode::W`](W) writer structure"]
impl crate::Writable for VdoH264RoiModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_ROI_MODE to value 0"]
impl crate::Resettable for VdoH264RoiModeSpec {}
