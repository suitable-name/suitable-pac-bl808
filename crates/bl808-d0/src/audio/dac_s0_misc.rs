#[doc = "Register `dac_s0_misc` reader"]
pub type R = crate::R<DacS0MiscSpec>;
#[doc = "Register `dac_s0_misc` writer"]
pub type W = crate::W<DacS0MiscSpec>;
#[doc = "Field `dac_s01_volume_update` reader - "]
pub type DacS01VolumeUpdateR = crate::BitReader;
#[doc = "Field `dac_s01_volume_update` writer - "]
pub type DacS01VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_s01_mute` reader - "]
pub type DacS01MuteR = crate::BitReader;
#[doc = "Field `dac_s01_mute` writer - "]
pub type DacS01MuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_s0_ctrl_zcd_timeout` reader - "]
pub type DacS0CtrlZcdTimeoutR = crate::FieldReader;
#[doc = "Field `dac_s0_ctrl_zcd_timeout` writer - "]
pub type DacS0CtrlZcdTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_s01_volume_update(&self) -> DacS01VolumeUpdateR {
        DacS01VolumeUpdateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dac_s01_mute(&self) -> DacS01MuteR {
        DacS01MuteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dac_s0_ctrl_zcd_timeout(&self) -> DacS0CtrlZcdTimeoutR {
        DacS0CtrlZcdTimeoutR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_s01_volume_update(&mut self) -> DacS01VolumeUpdateW<'_, DacS0MiscSpec> {
        DacS01VolumeUpdateW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dac_s01_mute(&mut self) -> DacS01MuteW<'_, DacS0MiscSpec> {
        DacS01MuteW::new(self, 1)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dac_s0_ctrl_zcd_timeout(&mut self) -> DacS0CtrlZcdTimeoutW<'_, DacS0MiscSpec> {
        DacS0CtrlZcdTimeoutW::new(self, 28)
    }
}
#[doc = "DAC Sideband 0 misc: Global volume sync and ZCD timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s0_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s0_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacS0MiscSpec;
impl crate::RegisterSpec for DacS0MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_s0_misc::R`](R) reader structure"]
impl crate::Readable for DacS0MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_s0_misc::W`](W) writer structure"]
impl crate::Writable for DacS0MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_s0_misc to value 0"]
impl crate::Resettable for DacS0MiscSpec {}
