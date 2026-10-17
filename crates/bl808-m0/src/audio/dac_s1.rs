#[doc = "Register `dac_s1` reader"]
pub type R = crate::R<DacS1Spec>;
#[doc = "Register `dac_s1` writer"]
pub type W = crate::W<DacS1Spec>;
#[doc = "Field `dac_s1_ctrl_rmp_rate` reader - "]
pub type DacS1CtrlRmpRateR = crate::FieldReader;
#[doc = "Field `dac_s1_ctrl_rmp_rate` writer - "]
pub type DacS1CtrlRmpRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s1_ctrl_zcd_rate` reader - "]
pub type DacS1CtrlZcdRateR = crate::FieldReader;
#[doc = "Field `dac_s1_ctrl_zcd_rate` writer - "]
pub type DacS1CtrlZcdRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s1_ctrl_mode` reader - "]
pub type DacS1CtrlModeR = crate::FieldReader;
#[doc = "Field `dac_s1_ctrl_mode` writer - "]
pub type DacS1CtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_s1_volume_update` reader - "]
pub type DacS1VolumeUpdateR = crate::BitReader;
#[doc = "Field `dac_s1_volume_update` writer - "]
pub type DacS1VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_s1_volume` reader - "]
pub type DacS1VolumeR = crate::FieldReader<u16>;
#[doc = "Field `dac_s1_volume` writer - "]
pub type DacS1VolumeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `dac_s1_mute_rmpup_rate` reader - "]
pub type DacS1MuteRmpupRateR = crate::FieldReader;
#[doc = "Field `dac_s1_mute_rmpup_rate` writer - "]
pub type DacS1MuteRmpupRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s1_mute_rmpdn_rate` reader - "]
pub type DacS1MuteRmpdnRateR = crate::FieldReader;
#[doc = "Field `dac_s1_mute_rmpdn_rate` writer - "]
pub type DacS1MuteRmpdnRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_s1_mute_softmode` reader - "]
pub type DacS1MuteSoftmodeR = crate::BitReader;
#[doc = "Field `dac_s1_mute_softmode` writer - "]
pub type DacS1MuteSoftmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_s1_mute` reader - "]
pub type DacS1MuteR = crate::BitReader;
#[doc = "Field `dac_s1_mute` writer - "]
pub type DacS1MuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn dac_s1_ctrl_rmp_rate(&self) -> DacS1CtrlRmpRateR {
        DacS1CtrlRmpRateR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dac_s1_ctrl_zcd_rate(&self) -> DacS1CtrlZcdRateR {
        DacS1CtrlZcdRateR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn dac_s1_ctrl_mode(&self) -> DacS1CtrlModeR {
        DacS1CtrlModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_s1_volume_update(&self) -> DacS1VolumeUpdateR {
        DacS1VolumeUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn dac_s1_volume(&self) -> DacS1VolumeR {
        DacS1VolumeR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dac_s1_mute_rmpup_rate(&self) -> DacS1MuteRmpupRateR {
        DacS1MuteRmpupRateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dac_s1_mute_rmpdn_rate(&self) -> DacS1MuteRmpdnRateR {
        DacS1MuteRmpdnRateR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dac_s1_mute_softmode(&self) -> DacS1MuteSoftmodeR {
        DacS1MuteSoftmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_s1_mute(&self) -> DacS1MuteR {
        DacS1MuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn dac_s1_ctrl_rmp_rate(&mut self) -> DacS1CtrlRmpRateW<'_, DacS1Spec> {
        DacS1CtrlRmpRateW::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dac_s1_ctrl_zcd_rate(&mut self) -> DacS1CtrlZcdRateW<'_, DacS1Spec> {
        DacS1CtrlZcdRateW::new(self, 6)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn dac_s1_ctrl_mode(&mut self) -> DacS1CtrlModeW<'_, DacS1Spec> {
        DacS1CtrlModeW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_s1_volume_update(&mut self) -> DacS1VolumeUpdateW<'_, DacS1Spec> {
        DacS1VolumeUpdateW::new(self, 12)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn dac_s1_volume(&mut self) -> DacS1VolumeW<'_, DacS1Spec> {
        DacS1VolumeW::new(self, 13)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dac_s1_mute_rmpup_rate(&mut self) -> DacS1MuteRmpupRateW<'_, DacS1Spec> {
        DacS1MuteRmpupRateW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dac_s1_mute_rmpdn_rate(&mut self) -> DacS1MuteRmpdnRateW<'_, DacS1Spec> {
        DacS1MuteRmpdnRateW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dac_s1_mute_softmode(&mut self) -> DacS1MuteSoftmodeW<'_, DacS1Spec> {
        DacS1MuteSoftmodeW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_s1_mute(&mut self) -> DacS1MuteW<'_, DacS1Spec> {
        DacS1MuteW::new(self, 31)
    }
}
#[doc = "DAC Sideband 1: Main control for second secondary audio output channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacS1Spec;
impl crate::RegisterSpec for DacS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_s1::R`](R) reader structure"]
impl crate::Readable for DacS1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_s1::W`](W) writer structure"]
impl crate::Writable for DacS1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_s1 to value 0"]
impl crate::Resettable for DacS1Spec {}
