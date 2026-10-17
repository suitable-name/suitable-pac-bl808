#[doc = "Register `PDS_CTL` reader"]
pub type R = crate::R<PdsCtlSpec>;
#[doc = "Register `PDS_CTL` writer"]
pub type W = crate::W<PdsCtlSpec>;
#[doc = "Field `pds_start_ps` writer - PDS start power save trigger"]
pub type PdsStartPsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_sleep_forever` reader - "]
pub type CrSleepForeverR = crate::BitReader;
#[doc = "Field `cr_sleep_forever` writer - "]
pub type CrSleepForeverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_xtal_force_off` reader - "]
pub type CrXtalForceOffR = crate::BitReader;
#[doc = "Field `cr_xtal_force_off` writer - "]
pub type CrXtalForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wifi_save_state` reader - "]
pub type CrPdsWifiSaveStateR = crate::BitReader;
#[doc = "Field `cr_pds_wifi_save_state` writer - "]
pub type CrPdsWifiSaveStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_dcdc11` reader - "]
pub type CrPdsPdDcdc11R = crate::BitReader;
#[doc = "Field `cr_pds_pd_dcdc11` writer - "]
pub type CrPdsPdDcdc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_bg_sys` reader - "]
pub type CrPdsPdBgSysR = crate::BitReader;
#[doc = "Field `cr_pds_pd_bg_sys` writer - "]
pub type CrPdsPdBgSysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_gpio_ie_pu_pd` reader - "]
pub type CrPdsCtrlGpioIePuPdR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_gpio_ie_pu_pd` writer - "]
pub type CrPdsCtrlGpioIePuPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_dcdc18` reader - "]
pub type CrPdsPdDcdc18R = crate::BitReader;
#[doc = "Field `cr_pds_pd_dcdc18` writer - "]
pub type CrPdsPdDcdc18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gate_clk` reader - "]
pub type CrPdsGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_gate_clk` writer - "]
pub type CrPdsGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mem_stby` reader - "]
pub type CrPdsMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_mem_stby` writer - "]
pub type CrPdsMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_glb_reg_reset_protect` reader - "]
pub type CrPdsGlbRegResetProtectR = crate::BitReader;
#[doc = "Field `cr_pds_glb_reg_reset_protect` writer - "]
pub type CrPdsGlbRegResetProtectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_iso_en` reader - "]
pub type CrPdsIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_iso_en` writer - "]
pub type CrPdsIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wait_xtal_rdy` reader - "]
pub type CrPdsWaitXtalRdyR = crate::BitReader;
#[doc = "Field `cr_pds_wait_xtal_rdy` writer - "]
pub type CrPdsWaitXtalRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pwr_off` reader - "]
pub type CrPdsPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_pwr_off` writer - "]
pub type CrPdsPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_xtal` reader - "]
pub type CrPdsPdXtalR = crate::BitReader;
#[doc = "Field `cr_pds_pd_xtal` writer - "]
pub type CrPdsPdXtalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_soc_enb` reader - "]
pub type CrPdsCtrlSocEnbR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_soc_enb` writer - "]
pub type CrPdsCtrlSocEnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rst_soc` reader - "]
pub type CrPdsRstSocR = crate::BitReader;
#[doc = "Field `cr_pds_rst_soc` writer - "]
pub type CrPdsRstSocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rc32m_off_dis` reader - "]
pub type CrPdsRc32mOffDisR = crate::BitReader;
#[doc = "Field `cr_pds_rc32m_off_dis` writer - "]
pub type CrPdsRc32mOffDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_dcdc11_vsel_en` reader - "]
pub type CrPdsDcdc11VselEnR = crate::BitReader;
#[doc = "Field `cr_pds_dcdc11_vsel_en` writer - "]
pub type CrPdsDcdc11VselEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_usbpll_pd` reader - "]
pub type CrPdsCtrlUsbpllPdR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_usbpll_pd` writer - "]
pub type CrPdsCtrlUsbpllPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_aupll_pd` reader - "]
pub type CrPdsCtrlAupllPdR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_aupll_pd` writer - "]
pub type CrPdsCtrlAupllPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_cpupll_pd` reader - "]
pub type CrPdsCtrlCpupllPdR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_cpupll_pd` writer - "]
pub type CrPdsCtrlCpupllPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_wifipll_pd` reader - "]
pub type CrPdsCtrlWifipllPdR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_wifipll_pd` writer - "]
pub type CrPdsCtrlWifipllPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_dcdc11_vol` reader - "]
pub type CrPdsDcdc11VolR = crate::FieldReader;
#[doc = "Field `cr_pds_dcdc11_vol` writer - "]
pub type CrPdsDcdc11VolW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cr_pds_ctrl_rf` reader - "]
pub type CrPdsCtrlRfR = crate::FieldReader;
#[doc = "Field `cr_pds_ctrl_rf` writer - "]
pub type CrPdsCtrlRfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_pds_start_use_tbtt_sleep` reader - "]
pub type CrPdsStartUseTbttSleepR = crate::BitReader;
#[doc = "Field `cr_pds_start_use_tbtt_sleep` writer - "]
pub type CrPdsStartUseTbttSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gpio_iso_mode` reader - "]
pub type CrPdsGpioIsoModeR = crate::BitReader;
#[doc = "Field `cr_pds_gpio_iso_mode` writer - "]
pub type CrPdsGpioIsoModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&self) -> CrSleepForeverR {
        CrSleepForeverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&self) -> CrXtalForceOffR {
        CrXtalForceOffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_wifi_save_state(&self) -> CrPdsWifiSaveStateR {
        CrPdsWifiSaveStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc11(&self) -> CrPdsPdDcdc11R {
        CrPdsPdDcdc11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&self) -> CrPdsPdBgSysR {
        CrPdsPdBgSysR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_ctrl_gpio_ie_pu_pd(&self) -> CrPdsCtrlGpioIePuPdR {
        CrPdsCtrlGpioIePuPdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&self) -> CrPdsPdDcdc18R {
        CrPdsPdDcdc18R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&self) -> CrPdsGateClkR {
        CrPdsGateClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&self) -> CrPdsMemStbyR {
        CrPdsMemStbyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_glb_reg_reset_protect(&self) -> CrPdsGlbRegResetProtectR {
        CrPdsGlbRegResetProtectR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&self) -> CrPdsIsoEnR {
        CrPdsIsoEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&self) -> CrPdsWaitXtalRdyR {
        CrPdsWaitXtalRdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&self) -> CrPdsPwrOffR {
        CrPdsPwrOffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&self) -> CrPdsPdXtalR {
        CrPdsPdXtalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_ctrl_soc_enb(&self) -> CrPdsCtrlSocEnbR {
        CrPdsCtrlSocEnbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc(&self) -> CrPdsRstSocR {
        CrPdsRstSocR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&self) -> CrPdsRc32mOffDisR {
        CrPdsRc32mOffDisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_dcdc11_vsel_en(&self) -> CrPdsDcdc11VselEnR {
        CrPdsDcdc11VselEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usbpll_pd(&self) -> CrPdsCtrlUsbpllPdR {
        CrPdsCtrlUsbpllPdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_ctrl_aupll_pd(&self) -> CrPdsCtrlAupllPdR {
        CrPdsCtrlAupllPdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_ctrl_cpupll_pd(&self) -> CrPdsCtrlCpupllPdR {
        CrPdsCtrlCpupllPdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wifipll_pd(&self) -> CrPdsCtrlWifipllPdR {
        CrPdsCtrlWifipllPdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn cr_pds_dcdc11_vol(&self) -> CrPdsDcdc11VolR {
        CrPdsDcdc11VolR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&self) -> CrPdsCtrlRfR {
        CrPdsCtrlRfR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_start_use_tbtt_sleep(&self) -> CrPdsStartUseTbttSleepR {
        CrPdsStartUseTbttSleepR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_gpio_iso_mode(&self) -> CrPdsGpioIsoModeR {
        CrPdsGpioIsoModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDS start power save trigger"]
    #[inline(always)]
    pub fn pds_start_ps(&mut self) -> PdsStartPsW<'_, PdsCtlSpec> {
        PdsStartPsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&mut self) -> CrSleepForeverW<'_, PdsCtlSpec> {
        CrSleepForeverW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&mut self) -> CrXtalForceOffW<'_, PdsCtlSpec> {
        CrXtalForceOffW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_wifi_save_state(&mut self) -> CrPdsWifiSaveStateW<'_, PdsCtlSpec> {
        CrPdsWifiSaveStateW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc11(&mut self) -> CrPdsPdDcdc11W<'_, PdsCtlSpec> {
        CrPdsPdDcdc11W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&mut self) -> CrPdsPdBgSysW<'_, PdsCtlSpec> {
        CrPdsPdBgSysW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_ctrl_gpio_ie_pu_pd(&mut self) -> CrPdsCtrlGpioIePuPdW<'_, PdsCtlSpec> {
        CrPdsCtrlGpioIePuPdW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&mut self) -> CrPdsPdDcdc18W<'_, PdsCtlSpec> {
        CrPdsPdDcdc18W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&mut self) -> CrPdsGateClkW<'_, PdsCtlSpec> {
        CrPdsGateClkW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&mut self) -> CrPdsMemStbyW<'_, PdsCtlSpec> {
        CrPdsMemStbyW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_glb_reg_reset_protect(&mut self) -> CrPdsGlbRegResetProtectW<'_, PdsCtlSpec> {
        CrPdsGlbRegResetProtectW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&mut self) -> CrPdsIsoEnW<'_, PdsCtlSpec> {
        CrPdsIsoEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&mut self) -> CrPdsWaitXtalRdyW<'_, PdsCtlSpec> {
        CrPdsWaitXtalRdyW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&mut self) -> CrPdsPwrOffW<'_, PdsCtlSpec> {
        CrPdsPwrOffW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&mut self) -> CrPdsPdXtalW<'_, PdsCtlSpec> {
        CrPdsPdXtalW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_ctrl_soc_enb(&mut self) -> CrPdsCtrlSocEnbW<'_, PdsCtlSpec> {
        CrPdsCtrlSocEnbW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc(&mut self) -> CrPdsRstSocW<'_, PdsCtlSpec> {
        CrPdsRstSocW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&mut self) -> CrPdsRc32mOffDisW<'_, PdsCtlSpec> {
        CrPdsRc32mOffDisW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_dcdc11_vsel_en(&mut self) -> CrPdsDcdc11VselEnW<'_, PdsCtlSpec> {
        CrPdsDcdc11VselEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usbpll_pd(&mut self) -> CrPdsCtrlUsbpllPdW<'_, PdsCtlSpec> {
        CrPdsCtrlUsbpllPdW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_ctrl_aupll_pd(&mut self) -> CrPdsCtrlAupllPdW<'_, PdsCtlSpec> {
        CrPdsCtrlAupllPdW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_ctrl_cpupll_pd(&mut self) -> CrPdsCtrlCpupllPdW<'_, PdsCtlSpec> {
        CrPdsCtrlCpupllPdW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wifipll_pd(&mut self) -> CrPdsCtrlWifipllPdW<'_, PdsCtlSpec> {
        CrPdsCtrlWifipllPdW::new(self, 22)
    }
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn cr_pds_dcdc11_vol(&mut self) -> CrPdsDcdc11VolW<'_, PdsCtlSpec> {
        CrPdsDcdc11VolW::new(self, 23)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&mut self) -> CrPdsCtrlRfW<'_, PdsCtlSpec> {
        CrPdsCtrlRfW::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_start_use_tbtt_sleep(&mut self) -> CrPdsStartUseTbttSleepW<'_, PdsCtlSpec> {
        CrPdsStartUseTbttSleepW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_gpio_iso_mode(&mut self) -> CrPdsGpioIsoModeW<'_, PdsCtlSpec> {
        CrPdsGpioIsoModeW::new(self, 31)
    }
}
#[doc = "PDS Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtlSpec;
impl crate::RegisterSpec for PdsCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl::R`](R) reader structure"]
impl crate::Readable for PdsCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl::W`](W) writer structure"]
impl crate::Writable for PdsCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL to value 0x1400_6b00"]
impl crate::Resettable for PdsCtlSpec {
    const RESET_VALUE: u32 = 0x1400_6b00;
}
