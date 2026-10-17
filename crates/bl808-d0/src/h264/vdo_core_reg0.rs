#[doc = "Register `VDO_CORE_REG0` reader"]
pub type R = crate::R<VdoCoreReg0Spec>;
#[doc = "Register `VDO_CORE_REG0` writer"]
pub type W = crate::W<VdoCoreReg0Spec>;
#[doc = "Field `enc_en` reader - Main-stream encoder enable"]
pub type EncEnR = crate::BitReader;
#[doc = "Field `enc_en` writer - Main-stream encoder enable"]
pub type EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vui_aspect_info` reader - Main VUI aspect ratio info present"]
pub type VuiAspectInfoR = crate::BitReader;
#[doc = "Field `vui_aspect_info` writer - Main VUI aspect ratio info present"]
pub type VuiAspectInfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vui_fixed_f_rate` reader - Main VUI fixed frame-rate flag"]
pub type VuiFixedFRateR = crate::BitReader;
#[doc = "Field `vui_fixed_f_rate` writer - Main VUI fixed frame-rate flag"]
pub type VuiFixedFRateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vui_timing_info` reader - Main VUI timing info present"]
pub type VuiTimingInfoR = crate::BitReader;
#[doc = "Field `vui_timing_info` writer - Main VUI timing info present"]
pub type VuiTimingInfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `idr_enable` reader - Main-stream IDR enable"]
pub type IdrEnableR = crate::BitReader;
#[doc = "Field `idr_enable` writer - Main-stream IDR enable"]
pub type IdrEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_enc_en` reader - Secondary-stream encoder enable"]
pub type SEncEnR = crate::BitReader;
#[doc = "Field `s_enc_en` writer - Secondary-stream encoder enable"]
pub type SEncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_vui_aspect_info` reader - Secondary VUI aspect ratio info present"]
pub type SVuiAspectInfoR = crate::BitReader;
#[doc = "Field `s_vui_aspect_info` writer - Secondary VUI aspect ratio info present"]
pub type SVuiAspectInfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_vui_fixed_f_rate` reader - Secondary VUI fixed frame-rate flag"]
pub type SVuiFixedFRateR = crate::BitReader;
#[doc = "Field `s_vui_fixed_f_rate` writer - Secondary VUI fixed frame-rate flag"]
pub type SVuiFixedFRateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_vui_timing_info` reader - Secondary VUI timing info present"]
pub type SVuiTimingInfoR = crate::BitReader;
#[doc = "Field `s_vui_timing_info` writer - Secondary VUI timing info present"]
pub type SVuiTimingInfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_idr_enable` reader - Secondary-stream IDR enable"]
pub type SIdrEnableR = crate::BitReader;
#[doc = "Field `s_idr_enable` writer - Secondary-stream IDR enable"]
pub type SIdrEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main-stream encoder enable"]
    #[inline(always)]
    pub fn enc_en(&self) -> EncEnR {
        EncEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Main VUI aspect ratio info present"]
    #[inline(always)]
    pub fn vui_aspect_info(&self) -> VuiAspectInfoR {
        VuiAspectInfoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Main VUI fixed frame-rate flag"]
    #[inline(always)]
    pub fn vui_fixed_f_rate(&self) -> VuiFixedFRateR {
        VuiFixedFRateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Main VUI timing info present"]
    #[inline(always)]
    pub fn vui_timing_info(&self) -> VuiTimingInfoR {
        VuiTimingInfoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main-stream IDR enable"]
    #[inline(always)]
    pub fn idr_enable(&self) -> IdrEnableR {
        IdrEnableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Secondary-stream encoder enable"]
    #[inline(always)]
    pub fn s_enc_en(&self) -> SEncEnR {
        SEncEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Secondary VUI aspect ratio info present"]
    #[inline(always)]
    pub fn s_vui_aspect_info(&self) -> SVuiAspectInfoR {
        SVuiAspectInfoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Secondary VUI fixed frame-rate flag"]
    #[inline(always)]
    pub fn s_vui_fixed_f_rate(&self) -> SVuiFixedFRateR {
        SVuiFixedFRateR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Secondary VUI timing info present"]
    #[inline(always)]
    pub fn s_vui_timing_info(&self) -> SVuiTimingInfoR {
        SVuiTimingInfoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secondary-stream IDR enable"]
    #[inline(always)]
    pub fn s_idr_enable(&self) -> SIdrEnableR {
        SIdrEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main-stream encoder enable"]
    #[inline(always)]
    pub fn enc_en(&mut self) -> EncEnW<'_, VdoCoreReg0Spec> {
        EncEnW::new(self, 0)
    }
    #[doc = "Bit 12 - Main VUI aspect ratio info present"]
    #[inline(always)]
    pub fn vui_aspect_info(&mut self) -> VuiAspectInfoW<'_, VdoCoreReg0Spec> {
        VuiAspectInfoW::new(self, 12)
    }
    #[doc = "Bit 13 - Main VUI fixed frame-rate flag"]
    #[inline(always)]
    pub fn vui_fixed_f_rate(&mut self) -> VuiFixedFRateW<'_, VdoCoreReg0Spec> {
        VuiFixedFRateW::new(self, 13)
    }
    #[doc = "Bit 14 - Main VUI timing info present"]
    #[inline(always)]
    pub fn vui_timing_info(&mut self) -> VuiTimingInfoW<'_, VdoCoreReg0Spec> {
        VuiTimingInfoW::new(self, 14)
    }
    #[doc = "Bit 15 - Main-stream IDR enable"]
    #[inline(always)]
    pub fn idr_enable(&mut self) -> IdrEnableW<'_, VdoCoreReg0Spec> {
        IdrEnableW::new(self, 15)
    }
    #[doc = "Bit 16 - Secondary-stream encoder enable"]
    #[inline(always)]
    pub fn s_enc_en(&mut self) -> SEncEnW<'_, VdoCoreReg0Spec> {
        SEncEnW::new(self, 16)
    }
    #[doc = "Bit 28 - Secondary VUI aspect ratio info present"]
    #[inline(always)]
    pub fn s_vui_aspect_info(&mut self) -> SVuiAspectInfoW<'_, VdoCoreReg0Spec> {
        SVuiAspectInfoW::new(self, 28)
    }
    #[doc = "Bit 29 - Secondary VUI fixed frame-rate flag"]
    #[inline(always)]
    pub fn s_vui_fixed_f_rate(&mut self) -> SVuiFixedFRateW<'_, VdoCoreReg0Spec> {
        SVuiFixedFRateW::new(self, 29)
    }
    #[doc = "Bit 30 - Secondary VUI timing info present"]
    #[inline(always)]
    pub fn s_vui_timing_info(&mut self) -> SVuiTimingInfoW<'_, VdoCoreReg0Spec> {
        SVuiTimingInfoW::new(self, 30)
    }
    #[doc = "Bit 31 - Secondary-stream IDR enable"]
    #[inline(always)]
    pub fn s_idr_enable(&mut self) -> SIdrEnableW<'_, VdoCoreReg0Spec> {
        SIdrEnableW::new(self, 31)
    }
}
#[doc = "Core control: encoder enable, VUI settings, IDR\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoCoreReg0Spec;
impl crate::RegisterSpec for VdoCoreReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_core_reg0::R`](R) reader structure"]
impl crate::Readable for VdoCoreReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`vdo_core_reg0::W`](W) writer structure"]
impl crate::Writable for VdoCoreReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_CORE_REG0 to value 0"]
impl crate::Resettable for VdoCoreReg0Spec {}
