#[doc = "Register `GAUGE` reader"]
pub type R = crate::R<GaugeSpec>;
#[doc = "Register `GAUGE` writer"]
pub type W = crate::W<GaugeSpec>;
#[doc = "Field `gauge_reserve` reader - "]
pub type GaugeReserveR = crate::FieldReader;
#[doc = "Field `gauge_reserve` writer - "]
pub type GaugeReserveW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gauge_ictrl_adc` reader - "]
pub type GaugeIctrlAdcR = crate::FieldReader;
#[doc = "Field `gauge_ictrl_adc` writer - "]
pub type GaugeIctrlAdcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gauge_dem_en` reader - "]
pub type GaugeDemEnR = crate::BitReader;
#[doc = "Field `gauge_dem_en` writer - "]
pub type GaugeDemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_ckb_en` reader - "]
pub type GaugeCkbEnR = crate::BitReader;
#[doc = "Field `gauge_ckb_en` writer - "]
pub type GaugeCkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_chop_phas` reader - "]
pub type GaugeChopPhasR = crate::BitReader;
#[doc = "Field `gauge_chop_phas` writer - "]
pub type GaugeChopPhasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_chop_freq` reader - "]
pub type GaugeChopFreqR = crate::FieldReader;
#[doc = "Field `gauge_chop_freq` writer - "]
pub type GaugeChopFreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gauge_chop_en` reader - "]
pub type GaugeChopEnR = crate::BitReader;
#[doc = "Field `gauge_chop_en` writer - "]
pub type GaugeChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_sel_edge` reader - "]
pub type GaugeSelEdgeR = crate::BitReader;
#[doc = "Field `gauge_sel_edge` writer - "]
pub type GaugeSelEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_quan_gain` reader - "]
pub type GaugeQuanGainR = crate::FieldReader;
#[doc = "Field `gauge_quan_gain` writer - "]
pub type GaugeQuanGainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gauge_sdm_pu` reader - "]
pub type GaugeSdmPuR = crate::BitReader;
#[doc = "Field `gauge_sdm_pu` writer - "]
pub type GaugeSdmPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_channel_sel` reader - "]
pub type GaugeChannelSelR = crate::BitReader;
#[doc = "Field `gauge_channel_sel` writer - "]
pub type GaugeChannelSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_channel_en` reader - "]
pub type GaugeChannelEnR = crate::BitReader;
#[doc = "Field `gauge_channel_en` writer - "]
pub type GaugeChannelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_lp_mode` reader - "]
pub type GaugeLpModeR = crate::BitReader;
#[doc = "Field `gauge_lp_mode` writer - "]
pub type GaugeLpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tmux_gauge_power` reader - "]
pub type TmuxGaugePowerR = crate::FieldReader;
#[doc = "Field `tmux_gauge_power` writer - "]
pub type TmuxGaugePowerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ten_gauge_power` reader - "]
pub type TenGaugePowerR = crate::BitReader;
#[doc = "Field `ten_gauge_power` writer - "]
pub type TenGaugePowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ntc_bias_sel` reader - "]
pub type NtcBiasSelR = crate::FieldReader;
#[doc = "Field `ntc_bias_sel` writer - "]
pub type NtcBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ntc_bias_en` reader - "]
pub type NtcBiasEnR = crate::BitReader;
#[doc = "Field `ntc_bias_en` writer - "]
pub type NtcBiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_ldo_pu` reader - "]
pub type GaugeLdoPuR = crate::BitReader;
#[doc = "Field `gauge_ldo_pu` writer - "]
pub type GaugeLdoPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_vcm_pu` reader - "]
pub type GaugeVcmPuR = crate::BitReader;
#[doc = "Field `gauge_vcm_pu` writer - "]
pub type GaugeVcmPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_bg_pu` reader - "]
pub type GaugeBgPuR = crate::BitReader;
#[doc = "Field `gauge_bg_pu` writer - "]
pub type GaugeBgPuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gauge_reserve(&self) -> GaugeReserveR {
        GaugeReserveR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gauge_ictrl_adc(&self) -> GaugeIctrlAdcR {
        GaugeIctrlAdcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gauge_dem_en(&self) -> GaugeDemEnR {
        GaugeDemEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gauge_ckb_en(&self) -> GaugeCkbEnR {
        GaugeCkbEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gauge_chop_phas(&self) -> GaugeChopPhasR {
        GaugeChopPhasR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gauge_chop_freq(&self) -> GaugeChopFreqR {
        GaugeChopFreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gauge_chop_en(&self) -> GaugeChopEnR {
        GaugeChopEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gauge_sel_edge(&self) -> GaugeSelEdgeR {
        GaugeSelEdgeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn gauge_quan_gain(&self) -> GaugeQuanGainR {
        GaugeQuanGainR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gauge_sdm_pu(&self) -> GaugeSdmPuR {
        GaugeSdmPuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gauge_channel_sel(&self) -> GaugeChannelSelR {
        GaugeChannelSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gauge_channel_en(&self) -> GaugeChannelEnR {
        GaugeChannelEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gauge_lp_mode(&self) -> GaugeLpModeR {
        GaugeLpModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tmux_gauge_power(&self) -> TmuxGaugePowerR {
        TmuxGaugePowerR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_gauge_power(&self) -> TenGaugePowerR {
        TenGaugePowerR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ntc_bias_sel(&self) -> NtcBiasSelR {
        NtcBiasSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ntc_bias_en(&self) -> NtcBiasEnR {
        NtcBiasEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gauge_ldo_pu(&self) -> GaugeLdoPuR {
        GaugeLdoPuR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gauge_vcm_pu(&self) -> GaugeVcmPuR {
        GaugeVcmPuR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gauge_bg_pu(&self) -> GaugeBgPuR {
        GaugeBgPuR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gauge_reserve(&mut self) -> GaugeReserveW<'_, GaugeSpec> {
        GaugeReserveW::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gauge_ictrl_adc(&mut self) -> GaugeIctrlAdcW<'_, GaugeSpec> {
        GaugeIctrlAdcW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gauge_dem_en(&mut self) -> GaugeDemEnW<'_, GaugeSpec> {
        GaugeDemEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gauge_ckb_en(&mut self) -> GaugeCkbEnW<'_, GaugeSpec> {
        GaugeCkbEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gauge_chop_phas(&mut self) -> GaugeChopPhasW<'_, GaugeSpec> {
        GaugeChopPhasW::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gauge_chop_freq(&mut self) -> GaugeChopFreqW<'_, GaugeSpec> {
        GaugeChopFreqW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gauge_chop_en(&mut self) -> GaugeChopEnW<'_, GaugeSpec> {
        GaugeChopEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gauge_sel_edge(&mut self) -> GaugeSelEdgeW<'_, GaugeSpec> {
        GaugeSelEdgeW::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn gauge_quan_gain(&mut self) -> GaugeQuanGainW<'_, GaugeSpec> {
        GaugeQuanGainW::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gauge_sdm_pu(&mut self) -> GaugeSdmPuW<'_, GaugeSpec> {
        GaugeSdmPuW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gauge_channel_sel(&mut self) -> GaugeChannelSelW<'_, GaugeSpec> {
        GaugeChannelSelW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gauge_channel_en(&mut self) -> GaugeChannelEnW<'_, GaugeSpec> {
        GaugeChannelEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gauge_lp_mode(&mut self) -> GaugeLpModeW<'_, GaugeSpec> {
        GaugeLpModeW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tmux_gauge_power(&mut self) -> TmuxGaugePowerW<'_, GaugeSpec> {
        TmuxGaugePowerW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_gauge_power(&mut self) -> TenGaugePowerW<'_, GaugeSpec> {
        TenGaugePowerW::new(self, 23)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ntc_bias_sel(&mut self) -> NtcBiasSelW<'_, GaugeSpec> {
        NtcBiasSelW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ntc_bias_en(&mut self) -> NtcBiasEnW<'_, GaugeSpec> {
        NtcBiasEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gauge_ldo_pu(&mut self) -> GaugeLdoPuW<'_, GaugeSpec> {
        GaugeLdoPuW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gauge_vcm_pu(&mut self) -> GaugeVcmPuW<'_, GaugeSpec> {
        GaugeVcmPuW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gauge_bg_pu(&mut self) -> GaugeBgPuW<'_, GaugeSpec> {
        GaugeBgPuW::new(self, 31)
    }
}
#[doc = "Gauge ADC Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GaugeSpec;
impl crate::RegisterSpec for GaugeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gauge::R`](R) reader structure"]
impl crate::Readable for GaugeSpec {}
#[doc = "`write(|w| ..)` method takes [`gauge::W`](W) writer structure"]
impl crate::Writable for GaugeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAUGE to value 0"]
impl crate::Resettable for GaugeSpec {}
