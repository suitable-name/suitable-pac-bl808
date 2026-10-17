#[doc = "Register `VDO_H264_HWBACK_MODE` reader"]
pub type R = crate::R<VdoH264HwbackModeSpec>;
#[doc = "Register `VDO_H264_HWBACK_MODE` writer"]
pub type W = crate::W<VdoH264HwbackModeSpec>;
#[doc = "Field `cfg_sramck_mode` reader - SRAM clock mode (0=new, 1=old)"]
pub type CfgSramckModeR = crate::BitReader;
#[doc = "Field `cfg_sramck_mode` writer - SRAM clock mode (0=new, 1=old)"]
pub type CfgSramckModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_nal_mp_mode` reader - Main-stream NAL multiple-partition mode"]
pub type CfgNalMpModeR = crate::BitReader;
#[doc = "Field `cfg_nal_mp_mode` writer - Main-stream NAL multiple-partition mode"]
pub type CfgNalMpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_nal_mp_mode` reader - Secondary-stream NAL multiple-partition mode"]
pub type CfgSNalMpModeR = crate::BitReader;
#[doc = "Field `cfg_s_nal_mp_mode` writer - Secondary-stream NAL multiple-partition mode"]
pub type CfgSNalMpModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM clock mode (0=new, 1=old)"]
    #[inline(always)]
    pub fn cfg_sramck_mode(&self) -> CfgSramckModeR {
        CfgSramckModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main-stream NAL multiple-partition mode"]
    #[inline(always)]
    pub fn cfg_nal_mp_mode(&self) -> CfgNalMpModeR {
        CfgNalMpModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary-stream NAL multiple-partition mode"]
    #[inline(always)]
    pub fn cfg_s_nal_mp_mode(&self) -> CfgSNalMpModeR {
        CfgSNalMpModeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM clock mode (0=new, 1=old)"]
    #[inline(always)]
    pub fn cfg_sramck_mode(&mut self) -> CfgSramckModeW<'_, VdoH264HwbackModeSpec> {
        CfgSramckModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Main-stream NAL multiple-partition mode"]
    #[inline(always)]
    pub fn cfg_nal_mp_mode(&mut self) -> CfgNalMpModeW<'_, VdoH264HwbackModeSpec> {
        CfgNalMpModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Secondary-stream NAL multiple-partition mode"]
    #[inline(always)]
    pub fn cfg_s_nal_mp_mode(&mut self) -> CfgSNalMpModeW<'_, VdoH264HwbackModeSpec> {
        CfgSNalMpModeW::new(self, 2)
    }
}
#[doc = "H264 hardware back-write mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_hwback_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_hwback_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264HwbackModeSpec;
impl crate::RegisterSpec for VdoH264HwbackModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_hwback_mode::R`](R) reader structure"]
impl crate::Readable for VdoH264HwbackModeSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_hwback_mode::W`](W) writer structure"]
impl crate::Writable for VdoH264HwbackModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_HWBACK_MODE to value 0"]
impl crate::Resettable for VdoH264HwbackModeSpec {}
