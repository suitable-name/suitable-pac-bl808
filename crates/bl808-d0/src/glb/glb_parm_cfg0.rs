#[doc = "Register `GLB_PARM_CFG0` reader"]
pub type R = crate::R<GlbParmCfg0Spec>;
#[doc = "Register `GLB_PARM_CFG0` writer"]
pub type W = crate::W<GlbParmCfg0Spec>;
#[doc = "Field `reg_bd_en` reader - "]
pub type RegBdEnR = crate::BitReader;
#[doc = "Field `reg_bd_en` writer - "]
pub type RegBdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart_swap_set` reader - "]
pub type UartSwapSetR = crate::FieldReader;
#[doc = "Field `uart_swap_set` writer - "]
pub type UartSwapSetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `swap_sflash_io_3_io_0` reader - "]
pub type SwapSflashIo3Io0R = crate::BitReader;
#[doc = "Field `swap_sflash_io_3_io_0` writer - "]
pub type SwapSflashIo3Io0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_embedded_sflash` reader - "]
pub type SelEmbeddedSflashR = crate::BitReader;
#[doc = "Field `sel_embedded_sflash` writer - "]
pub type SelEmbeddedSflashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sel_psram0_x16` reader - "]
pub type RegSelPsram0X16R = crate::BitReader;
#[doc = "Field `reg_sel_psram0_x16` writer - "]
pub type RegSelPsram0X16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_0_master_mode` reader - "]
pub type RegSpi0MasterModeR = crate::BitReader;
#[doc = "Field `reg_spi_0_master_mode` writer - "]
pub type RegSpi0MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_0_swap` reader - "]
pub type RegSpi0SwapR = crate::BitReader;
#[doc = "Field `reg_spi_0_swap` writer - "]
pub type RegSpi0SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sel_dbi_type_c` reader - "]
pub type RegSelDbiTypeCR = crate::BitReader;
#[doc = "Field `reg_sel_dbi_type_c` writer - "]
pub type RegSelDbiTypeCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ant_switch_sel` reader - "]
pub type AntSwitchSelR = crate::BitReader;
#[doc = "Field `ant_switch_sel` writer - "]
pub type AntSwitchSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p1_adc_test_with_cci` reader - "]
pub type P1AdcTestWithCciR = crate::BitReader;
#[doc = "Field `p1_adc_test_with_cci` writer - "]
pub type P1AdcTestWithCciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p2_dac_test_with_cci` reader - "]
pub type P2DacTestWithCciR = crate::BitReader;
#[doc = "Field `p2_dac_test_with_cci` writer - "]
pub type P2DacTestWithCciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p3_cci_use_io_2_5` reader - "]
pub type P3CciUseIo2_5R = crate::BitReader;
#[doc = "Field `p3_cci_use_io_2_5` writer - "]
pub type P3CciUseIo2_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p4_adc_test_with_jtag` reader - "]
pub type P4AdcTestWithJtagR = crate::BitReader;
#[doc = "Field `p4_adc_test_with_jtag` writer - "]
pub type P4AdcTestWithJtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p5_dac_test_with_jtag` reader - "]
pub type P5DacTestWithJtagR = crate::BitReader;
#[doc = "Field `p5_dac_test_with_jtag` writer - "]
pub type P5DacTestWithJtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p6_sdh_use_io_0_5` reader - "]
pub type P6SdhUseIo0_5R = crate::BitReader;
#[doc = "Field `p6_sdh_use_io_0_5` writer - "]
pub type P6SdhUseIo0_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p7_jtag_use_io_2_5` reader - "]
pub type P7JtagUseIo2_5R = crate::BitReader;
#[doc = "Field `p7_jtag_use_io_2_5` writer - "]
pub type P7JtagUseIo2_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf1_test_mode` reader - "]
pub type Rf1TestModeR = crate::FieldReader;
#[doc = "Field `rf1_test_mode` writer - "]
pub type Rf1TestModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mm_spi_master_mode` reader - "]
pub type RegMmSpiMasterModeR = crate::BitReader;
#[doc = "Field `reg_mm_spi_master_mode` writer - "]
pub type RegMmSpiMasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mm_spi_swap` reader - "]
pub type RegMmSpiSwapR = crate::BitReader;
#[doc = "Field `reg_mm_spi_swap` writer - "]
pub type RegMmSpiSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `audio_test_mode` reader - "]
pub type AudioTestModeR = crate::BitReader;
#[doc = "Field `audio_test_mode` writer - "]
pub type AudioTestModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_rf_audio_test` reader - "]
pub type SelRfAudioTestR = crate::FieldReader;
#[doc = "Field `sel_rf_audio_test` writer - "]
pub type SelRfAudioTestW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&self) -> RegBdEnR {
        RegBdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UartSwapSetR {
        UartSwapSetR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&self) -> SwapSflashIo3Io0R {
        SwapSflashIo3Io0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&self) -> SelEmbeddedSflashR {
        SelEmbeddedSflashR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_sel_psram0_x16(&self) -> RegSelPsram0X16R {
        RegSelPsram0X16R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> RegSpi0MasterModeR {
        RegSpi0MasterModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> RegSpi0SwapR {
        RegSpi0SwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_sel_dbi_type_c(&self) -> RegSelDbiTypeCR {
        RegSelDbiTypeCR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ant_switch_sel(&self) -> AntSwitchSelR {
        AntSwitchSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&self) -> P1AdcTestWithCciR {
        P1AdcTestWithCciR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&self) -> P2DacTestWithCciR {
        P2DacTestWithCciR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&self) -> P3CciUseIo2_5R {
        P3CciUseIo2_5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&self) -> P4AdcTestWithJtagR {
        P4AdcTestWithJtagR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&self) -> P5DacTestWithJtagR {
        P5DacTestWithJtagR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdh_use_io_0_5(&self) -> P6SdhUseIo0_5R {
        P6SdhUseIo0_5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&self) -> P7JtagUseIo2_5R {
        P7JtagUseIo2_5R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn rf1_test_mode(&self) -> Rf1TestModeR {
        Rf1TestModeR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_mm_spi_master_mode(&self) -> RegMmSpiMasterModeR {
        RegMmSpiMasterModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_mm_spi_swap(&self) -> RegMmSpiSwapR {
        RegMmSpiSwapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn audio_test_mode(&self) -> AudioTestModeR {
        AudioTestModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sel_rf_audio_test(&self) -> SelRfAudioTestR {
        SelRfAudioTestR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&mut self) -> RegBdEnW<'_, GlbParmCfg0Spec> {
        RegBdEnW::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn uart_swap_set(&mut self) -> UartSwapSetW<'_, GlbParmCfg0Spec> {
        UartSwapSetW::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&mut self) -> SwapSflashIo3Io0W<'_, GlbParmCfg0Spec> {
        SwapSflashIo3Io0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&mut self) -> SelEmbeddedSflashW<'_, GlbParmCfg0Spec> {
        SelEmbeddedSflashW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_sel_psram0_x16(&mut self) -> RegSelPsram0X16W<'_, GlbParmCfg0Spec> {
        RegSelPsram0X16W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&mut self) -> RegSpi0MasterModeW<'_, GlbParmCfg0Spec> {
        RegSpi0MasterModeW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&mut self) -> RegSpi0SwapW<'_, GlbParmCfg0Spec> {
        RegSpi0SwapW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_sel_dbi_type_c(&mut self) -> RegSelDbiTypeCW<'_, GlbParmCfg0Spec> {
        RegSelDbiTypeCW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ant_switch_sel(&mut self) -> AntSwitchSelW<'_, GlbParmCfg0Spec> {
        AntSwitchSelW::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&mut self) -> P1AdcTestWithCciW<'_, GlbParmCfg0Spec> {
        P1AdcTestWithCciW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&mut self) -> P2DacTestWithCciW<'_, GlbParmCfg0Spec> {
        P2DacTestWithCciW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&mut self) -> P3CciUseIo2_5W<'_, GlbParmCfg0Spec> {
        P3CciUseIo2_5W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&mut self) -> P4AdcTestWithJtagW<'_, GlbParmCfg0Spec> {
        P4AdcTestWithJtagW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&mut self) -> P5DacTestWithJtagW<'_, GlbParmCfg0Spec> {
        P5DacTestWithJtagW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdh_use_io_0_5(&mut self) -> P6SdhUseIo0_5W<'_, GlbParmCfg0Spec> {
        P6SdhUseIo0_5W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&mut self) -> P7JtagUseIo2_5W<'_, GlbParmCfg0Spec> {
        P7JtagUseIo2_5W::new(self, 23)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn rf1_test_mode(&mut self) -> Rf1TestModeW<'_, GlbParmCfg0Spec> {
        Rf1TestModeW::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_mm_spi_master_mode(&mut self) -> RegMmSpiMasterModeW<'_, GlbParmCfg0Spec> {
        RegMmSpiMasterModeW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_mm_spi_swap(&mut self) -> RegMmSpiSwapW<'_, GlbParmCfg0Spec> {
        RegMmSpiSwapW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn audio_test_mode(&mut self) -> AudioTestModeW<'_, GlbParmCfg0Spec> {
        AudioTestModeW::new(self, 29)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sel_rf_audio_test(&mut self) -> SelRfAudioTestW<'_, GlbParmCfg0Spec> {
        SelRfAudioTestW::new(self, 30)
    }
}
#[doc = "Global Parameter Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_parm_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_parm_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbParmCfg0Spec;
impl crate::RegisterSpec for GlbParmCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_parm_cfg0::R`](R) reader structure"]
impl crate::Readable for GlbParmCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`glb_parm_cfg0::W`](W) writer structure"]
impl crate::Writable for GlbParmCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GLB_PARM_CFG0 to value 0"]
impl crate::Resettable for GlbParmCfg0Spec {}
