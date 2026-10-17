#[doc = "Register `audio_status` reader"]
pub type R = crate::R<AudioStatusSpec>;
#[doc = "Register `audio_status` writer"]
pub type W = crate::W<AudioStatusSpec>;
#[doc = "Field `adc_h0_busy` reader - "]
pub type AdcH0BusyR = crate::BitReader;
#[doc = "Field `adc_h0_mute_done` reader - "]
pub type AdcH0MuteDoneR = crate::BitReader;
#[doc = "Field `adc_h1_busy` reader - "]
pub type AdcH1BusyR = crate::BitReader;
#[doc = "Field `adc_h1_mute_done` reader - "]
pub type AdcH1MuteDoneR = crate::BitReader;
#[doc = "Field `adc_h2_busy` reader - "]
pub type AdcH2BusyR = crate::BitReader;
#[doc = "Field `adc_h2_mute_done` reader - "]
pub type AdcH2MuteDoneR = crate::BitReader;
#[doc = "Field `adc_s0_int` reader - "]
pub type AdcS0IntR = crate::BitReader;
#[doc = "Field `adc_s0_int_clr` writer - "]
pub type AdcS0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `adc_s1_int` reader - "]
pub type AdcS1IntR = crate::BitReader;
#[doc = "Field `adc_s1_int_clr` writer - "]
pub type AdcS1IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `adc_s2_int` reader - "]
pub type AdcS2IntR = crate::BitReader;
#[doc = "Field `adc_s2_int_clr` writer - "]
pub type AdcS2IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `dac_h0_busy` reader - "]
pub type DacH0BusyR = crate::BitReader;
#[doc = "Field `dac_h0_mute_done` reader - "]
pub type DacH0MuteDoneR = crate::BitReader;
#[doc = "Field `dac_h1_busy` reader - "]
pub type DacH1BusyR = crate::BitReader;
#[doc = "Field `dac_h1_mute_done` reader - "]
pub type DacH1MuteDoneR = crate::BitReader;
#[doc = "Field `dac_s0_int` reader - "]
pub type DacS0IntR = crate::BitReader;
#[doc = "Field `dac_s0_int_clr` writer - "]
pub type DacS0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `dac_s1_int` reader - "]
pub type DacS1IntR = crate::BitReader;
#[doc = "Field `dac_s1_int_clr` writer - "]
pub type DacS1IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ngt_amute` reader - "]
pub type NgtAmuteR = crate::BitReader;
#[doc = "Field `ngt_dmute` reader - "]
pub type NgtDmuteR = crate::BitReader;
#[doc = "Field `ngt_dmute_done` reader - "]
pub type NgtDmuteDoneR = crate::BitReader;
#[doc = "Field `zd_amute` reader - "]
pub type ZdAmuteR = crate::BitReader;
#[doc = "Field `audio_int_all` reader - "]
pub type AudioIntAllR = crate::BitReader;
#[doc = "Field `vad_int` reader - "]
pub type VadIntR = crate::BitReader;
#[doc = "Field `vad_int_clr` writer - "]
pub type VadIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `vad_int_set` reader - "]
pub type VadIntSetR = crate::BitReader;
#[doc = "Field `vad_int_set` writer - "]
pub type VadIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_h0_busy(&self) -> AdcH0BusyR {
        AdcH0BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_h0_mute_done(&self) -> AdcH0MuteDoneR {
        AdcH0MuteDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_h1_busy(&self) -> AdcH1BusyR {
        AdcH1BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adc_h1_mute_done(&self) -> AdcH1MuteDoneR {
        AdcH1MuteDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adc_h2_busy(&self) -> AdcH2BusyR {
        AdcH2BusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adc_h2_mute_done(&self) -> AdcH2MuteDoneR {
        AdcH2MuteDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adc_s0_int(&self) -> AdcS0IntR {
        AdcS0IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adc_s1_int(&self) -> AdcS1IntR {
        AdcS1IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adc_s2_int(&self) -> AdcS2IntR {
        AdcS2IntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_h0_busy(&self) -> DacH0BusyR {
        DacH0BusyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_h0_mute_done(&self) -> DacH0MuteDoneR {
        DacH0MuteDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dac_h1_busy(&self) -> DacH1BusyR {
        DacH1BusyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dac_h1_mute_done(&self) -> DacH1MuteDoneR {
        DacH1MuteDoneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dac_s0_int(&self) -> DacS0IntR {
        DacS0IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dac_s1_int(&self) -> DacS1IntR {
        DacS1IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ngt_amute(&self) -> NgtAmuteR {
        NgtAmuteR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ngt_dmute(&self) -> NgtDmuteR {
        NgtDmuteR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ngt_dmute_done(&self) -> NgtDmuteDoneR {
        NgtDmuteDoneR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn zd_amute(&self) -> ZdAmuteR {
        ZdAmuteR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn audio_int_all(&self) -> AudioIntAllR {
        AudioIntAllR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn vad_int(&self) -> VadIntR {
        VadIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn vad_int_set(&self) -> VadIntSetR {
        VadIntSetR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adc_s0_int_clr(&mut self) -> AdcS0IntClrW<'_, AudioStatusSpec> {
        AdcS0IntClrW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc_s1_int_clr(&mut self) -> AdcS1IntClrW<'_, AudioStatusSpec> {
        AdcS1IntClrW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adc_s2_int_clr(&mut self) -> AdcS2IntClrW<'_, AudioStatusSpec> {
        AdcS2IntClrW::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dac_s0_int_clr(&mut self) -> DacS0IntClrW<'_, AudioStatusSpec> {
        DacS0IntClrW::new(self, 17)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dac_s1_int_clr(&mut self) -> DacS1IntClrW<'_, AudioStatusSpec> {
        DacS1IntClrW::new(self, 19)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vad_int_clr(&mut self) -> VadIntClrW<'_, AudioStatusSpec> {
        VadIntClrW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn vad_int_set(&mut self) -> VadIntSetW<'_, AudioStatusSpec> {
        VadIntSetW::new(self, 27)
    }
}
#[doc = "Audio Master Status and Interrupt: Clearing bits for all ADC/DAC events\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioStatusSpec;
impl crate::RegisterSpec for AudioStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_status::R`](R) reader structure"]
impl crate::Readable for AudioStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`audio_status::W`](W) writer structure"]
impl crate::Writable for AudioStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x040a_0a80;
}
#[doc = "`reset()` method sets audio_status to value 0"]
impl crate::Resettable for AudioStatusSpec {}
