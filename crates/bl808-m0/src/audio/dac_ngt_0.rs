#[doc = "Register `dac_ngt_0` reader"]
pub type R = crate::R<DacNgt0Spec>;
#[doc = "Register `dac_ngt_0` writer"]
pub type W = crate::W<DacNgt0Spec>;
#[doc = "Field `ngt_amute_on` reader - "]
pub type NgtAmuteOnR = crate::BitReader;
#[doc = "Field `ngt_amute_on` writer - "]
pub type NgtAmuteOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ngt_dmute_on` reader - "]
pub type NgtDmuteOnR = crate::BitReader;
#[doc = "Field `ngt_dmute_on` writer - "]
pub type NgtDmuteOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ngt_signal_hold_time_k` reader - "]
pub type NgtSignalHoldTimeKR = crate::FieldReader;
#[doc = "Field `ngt_signal_hold_time_k` writer - "]
pub type NgtSignalHoldTimeKW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ngt_signal_hold_en` reader - "]
pub type NgtSignalHoldEnR = crate::BitReader;
#[doc = "Field `ngt_signal_hold_en` writer - "]
pub type NgtSignalHoldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ngt_noise_hold_time_k` reader - "]
pub type NgtNoiseHoldTimeKR = crate::FieldReader;
#[doc = "Field `ngt_noise_hold_time_k` writer - "]
pub type NgtNoiseHoldTimeKW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ngt_noise_hold_en` reader - "]
pub type NgtNoiseHoldEnR = crate::BitReader;
#[doc = "Field `ngt_noise_hold_en` writer - "]
pub type NgtNoiseHoldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ngt_hysteresis` reader - "]
pub type NgtHysteresisR = crate::FieldReader;
#[doc = "Field `ngt_hysteresis` writer - "]
pub type NgtHysteresisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ngt_threshold` reader - "]
pub type NgtThresholdR = crate::FieldReader;
#[doc = "Field `ngt_threshold` writer - "]
pub type NgtThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ngt_avg_time` reader - "]
pub type NgtAvgTimeR = crate::FieldReader;
#[doc = "Field `ngt_avg_time` writer - "]
pub type NgtAvgTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ngt_src_sel` reader - "]
pub type NgtSrcSelR = crate::FieldReader;
#[doc = "Field `ngt_src_sel` writer - "]
pub type NgtSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ngt_en` reader - "]
pub type NgtEnR = crate::BitReader;
#[doc = "Field `ngt_en` writer - "]
pub type NgtEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ngt_amute_on(&self) -> NgtAmuteOnR {
        NgtAmuteOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ngt_dmute_on(&self) -> NgtDmuteOnR {
        NgtDmuteOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn ngt_signal_hold_time_k(&self) -> NgtSignalHoldTimeKR {
        NgtSignalHoldTimeKR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ngt_signal_hold_en(&self) -> NgtSignalHoldEnR {
        NgtSignalHoldEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn ngt_noise_hold_time_k(&self) -> NgtNoiseHoldTimeKR {
        NgtNoiseHoldTimeKR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ngt_noise_hold_en(&self) -> NgtNoiseHoldEnR {
        NgtNoiseHoldEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ngt_hysteresis(&self) -> NgtHysteresisR {
        NgtHysteresisR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ngt_threshold(&self) -> NgtThresholdR {
        NgtThresholdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn ngt_avg_time(&self) -> NgtAvgTimeR {
        NgtAvgTimeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn ngt_src_sel(&self) -> NgtSrcSelR {
        NgtSrcSelR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ngt_en(&self) -> NgtEnR {
        NgtEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ngt_amute_on(&mut self) -> NgtAmuteOnW<'_, DacNgt0Spec> {
        NgtAmuteOnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ngt_dmute_on(&mut self) -> NgtDmuteOnW<'_, DacNgt0Spec> {
        NgtDmuteOnW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn ngt_signal_hold_time_k(&mut self) -> NgtSignalHoldTimeKW<'_, DacNgt0Spec> {
        NgtSignalHoldTimeKW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ngt_signal_hold_en(&mut self) -> NgtSignalHoldEnW<'_, DacNgt0Spec> {
        NgtSignalHoldEnW::new(self, 8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn ngt_noise_hold_time_k(&mut self) -> NgtNoiseHoldTimeKW<'_, DacNgt0Spec> {
        NgtNoiseHoldTimeKW::new(self, 9)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ngt_noise_hold_en(&mut self) -> NgtNoiseHoldEnW<'_, DacNgt0Spec> {
        NgtNoiseHoldEnW::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ngt_hysteresis(&mut self) -> NgtHysteresisW<'_, DacNgt0Spec> {
        NgtHysteresisW::new(self, 14)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ngt_threshold(&mut self) -> NgtThresholdW<'_, DacNgt0Spec> {
        NgtThresholdW::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn ngt_avg_time(&mut self) -> NgtAvgTimeW<'_, DacNgt0Spec> {
        NgtAvgTimeW::new(self, 24)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn ngt_src_sel(&mut self) -> NgtSrcSelW<'_, DacNgt0Spec> {
        NgtSrcSelW::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ngt_en(&mut self) -> NgtEnW<'_, DacNgt0Spec> {
        NgtEnW::new(self, 31)
    }
}
#[doc = "DAC Noise Gate 0: Hysteresis and source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_ngt_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ngt_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacNgt0Spec;
impl crate::RegisterSpec for DacNgt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_ngt_0::R`](R) reader structure"]
impl crate::Readable for DacNgt0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_ngt_0::W`](W) writer structure"]
impl crate::Writable for DacNgt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_ngt_0 to value 0"]
impl crate::Resettable for DacNgt0Spec {}
