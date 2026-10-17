#[doc = "Register `dac_zd_0` reader"]
pub type R = crate::R<DacZd0Spec>;
#[doc = "Register `dac_zd_0` writer"]
pub type W = crate::W<DacZd0Spec>;
#[doc = "Field `zd_time` reader - "]
pub type ZdTimeR = crate::FieldReader<u16>;
#[doc = "Field `zd_time` writer - "]
pub type ZdTimeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `zd_en` reader - "]
pub type ZdEnR = crate::FieldReader;
#[doc = "Field `zd_en` writer - "]
pub type ZdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_mute_hw_ctrl` reader - "]
pub type DacMuteHwCtrlR = crate::BitReader;
#[doc = "Field `dac_mute_hw_ctrl` writer - "]
pub type DacMuteHwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_mute` reader - "]
pub type DacMuteR = crate::BitReader;
#[doc = "Field `dac_mute` writer - "]
pub type DacMuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn zd_time(&self) -> ZdTimeR {
        ZdTimeR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn zd_en(&self) -> ZdEnR {
        ZdEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dac_mute_hw_ctrl(&self) -> DacMuteHwCtrlR {
        DacMuteHwCtrlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_mute(&self) -> DacMuteR {
        DacMuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn zd_time(&mut self) -> ZdTimeW<'_, DacZd0Spec> {
        ZdTimeW::new(self, 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn zd_en(&mut self) -> ZdEnW<'_, DacZd0Spec> {
        ZdEnW::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dac_mute_hw_ctrl(&mut self) -> DacMuteHwCtrlW<'_, DacZd0Spec> {
        DacMuteHwCtrlW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_mute(&mut self) -> DacMuteW<'_, DacZd0Spec> {
        DacMuteW::new(self, 31)
    }
}
#[doc = "DAC Zero Detection 0: Automatic mute during silence detection\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_zd_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_zd_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacZd0Spec;
impl crate::RegisterSpec for DacZd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_zd_0::R`](R) reader structure"]
impl crate::Readable for DacZd0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_zd_0::W`](W) writer structure"]
impl crate::Writable for DacZd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_zd_0 to value 0"]
impl crate::Resettable for DacZd0Spec {}
