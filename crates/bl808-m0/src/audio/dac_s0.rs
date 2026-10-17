#[doc = "Register `dac_s0` reader"]
pub type R = crate::R<DacS0Spec>;
#[doc = "Register `dac_s0` writer"]
pub type W = crate::W<DacS0Spec>;
#[doc = "Field `dac_s0_ctrl_rmp_rate` reader - "]
pub type DacS0CtrlRmpRateR = crate::FieldReader;
#[doc = "Field `dac_s0_ctrl_rmp_rate` writer - "]
pub type DacS0CtrlRmpRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s0_ctrl_zcd_rate` reader - "]
pub type DacS0CtrlZcdRateR = crate::FieldReader;
#[doc = "Field `dac_s0_ctrl_zcd_rate` writer - "]
pub type DacS0CtrlZcdRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s0_ctrl_mode` reader - "]
pub type DacS0CtrlModeR = crate::FieldReader;
#[doc = "Field `dac_s0_ctrl_mode` writer - "]
pub type DacS0CtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_s0_volume_update` reader - "]
pub type DacS0VolumeUpdateR = crate::BitReader;
#[doc = "Field `dac_s0_volume_update` writer - "]
pub type DacS0VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_s0_volume` reader - "]
pub type DacS0VolumeR = crate::FieldReader<u16>;
#[doc = "Field `dac_s0_volume` writer - "]
pub type DacS0VolumeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `dac_s0_mute_rmpup_rate` reader - "]
pub type DacS0MuteRmpupRateR = crate::FieldReader;
#[doc = "Field `dac_s0_mute_rmpup_rate` writer - "]
pub type DacS0MuteRmpupRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s0_mute_rmpdn_rate` reader - "]
pub type DacS0MuteRmpdnRateR = crate::FieldReader;
#[doc = "Field `dac_s0_mute_rmpdn_rate` writer - "]
pub type DacS0MuteRmpdnRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s0_mute_softmode` reader - "]
pub type DacS0MuteSoftmodeR = crate::BitReader;
#[doc = "Field `dac_s0_mute_softmode` writer - "]
pub type DacS0MuteSoftmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_s0_mute` reader - "]
pub type DacS0MuteR = crate::BitReader;
#[doc = "Field `dac_s0_mute` writer - "]
pub type DacS0MuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn dac_s0_ctrl_rmp_rate(&self) -> DacS0CtrlRmpRateR {
        DacS0CtrlRmpRateR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dac_s0_ctrl_zcd_rate(&self) -> DacS0CtrlZcdRateR {
        DacS0CtrlZcdRateR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn dac_s0_ctrl_mode(&self) -> DacS0CtrlModeR {
        DacS0CtrlModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_s0_volume_update(&self) -> DacS0VolumeUpdateR {
        DacS0VolumeUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn dac_s0_volume(&self) -> DacS0VolumeR {
        DacS0VolumeR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dac_s0_mute_rmpup_rate(&self) -> DacS0MuteRmpupRateR {
        DacS0MuteRmpupRateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dac_s0_mute_rmpdn_rate(&self) -> DacS0MuteRmpdnRateR {
        DacS0MuteRmpdnRateR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dac_s0_mute_softmode(&self) -> DacS0MuteSoftmodeR {
        DacS0MuteSoftmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_s0_mute(&self) -> DacS0MuteR {
        DacS0MuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn dac_s0_ctrl_rmp_rate(&mut self) -> DacS0CtrlRmpRateW<'_, DacS0Spec> {
        DacS0CtrlRmpRateW::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dac_s0_ctrl_zcd_rate(&mut self) -> DacS0CtrlZcdRateW<'_, DacS0Spec> {
        DacS0CtrlZcdRateW::new(self, 6)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn dac_s0_ctrl_mode(&mut self) -> DacS0CtrlModeW<'_, DacS0Spec> {
        DacS0CtrlModeW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_s0_volume_update(&mut self) -> DacS0VolumeUpdateW<'_, DacS0Spec> {
        DacS0VolumeUpdateW::new(self, 12)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn dac_s0_volume(&mut self) -> DacS0VolumeW<'_, DacS0Spec> {
        DacS0VolumeW::new(self, 13)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dac_s0_mute_rmpup_rate(&mut self) -> DacS0MuteRmpupRateW<'_, DacS0Spec> {
        DacS0MuteRmpupRateW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dac_s0_mute_rmpdn_rate(&mut self) -> DacS0MuteRmpdnRateW<'_, DacS0Spec> {
        DacS0MuteRmpdnRateW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dac_s0_mute_softmode(&mut self) -> DacS0MuteSoftmodeW<'_, DacS0Spec> {
        DacS0MuteSoftmodeW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_s0_mute(&mut self) -> DacS0MuteW<'_, DacS0Spec> {
        DacS0MuteW::new(self, 31)
    }
}
#[doc = "DAC Sideband 0: Main control for secondary audio output channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacS0Spec;
impl crate::RegisterSpec for DacS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_s0::R`](R) reader structure"]
impl crate::Readable for DacS0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_s0::W`](W) writer structure"]
impl crate::Writable for DacS0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_s0 to value 0"]
impl crate::Resettable for DacS0Spec {}
