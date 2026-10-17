#[doc = "Register `EF_CTRL_SW_CFG_0` reader"]
pub type R = crate::R<EfCtrlSwCfg0Spec>;
#[doc = "Register `EF_CTRL_SW_CFG_0` writer"]
pub type W = crate::W<EfCtrlSwCfg0Spec>;
#[doc = "Field `ef_sw_sf_aes_mode` reader - Flash AES mode software override"]
pub type EfSwSfAesModeR = crate::FieldReader;
#[doc = "Field `ef_sw_sf_aes_mode` writer - Flash AES mode software override"]
pub type EfSwSfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_ai_dis` reader - Analog/audio interface disable software override"]
pub type EfSwAiDisR = crate::BitReader;
#[doc = "Field `ef_sw_ai_dis` writer - Analog/audio interface disable software override"]
pub type EfSwAiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cpu0_dis` reader - CPU0 disable software override"]
pub type EfSwCpu0DisR = crate::BitReader;
#[doc = "Field `ef_sw_cpu0_dis` writer - CPU0 disable software override"]
pub type EfSwCpu0DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_sboot_en` reader - Secure boot enable software override"]
pub type EfSwSbootEnR = crate::FieldReader;
#[doc = "Field `ef_sw_sboot_en` writer - Secure boot enable software override"]
pub type EfSwSbootEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_uart_dis` reader - UART disable software override"]
pub type EfSwUartDisR = crate::FieldReader;
#[doc = "Field `ef_sw_uart_dis` writer - UART disable software override"]
pub type EfSwUartDisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ef_sw_ble2_dis` reader - BLE2 disable software override"]
pub type EfSwBle2DisR = crate::BitReader;
#[doc = "Field `ef_sw_ble2_dis` writer - BLE2 disable software override"]
pub type EfSwBle2DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_m1542_dis` reader - M154_2 disable software override"]
pub type EfSwM1542DisR = crate::BitReader;
#[doc = "Field `ef_sw_m1542_dis` writer - M154_2 disable software override"]
pub type EfSwM1542DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_sf_key_re_sel` reader - SF key reuse selection software override"]
pub type EfSwSfKeyReSelR = crate::FieldReader;
#[doc = "Field `ef_sw_sf_key_re_sel` writer - SF key reuse selection software override"]
pub type EfSwSfKeyReSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_sdu_dis` reader - SDU disable software override"]
pub type EfSwSduDisR = crate::BitReader;
#[doc = "Field `ef_sw_sdu_dis` writer - SDU disable software override"]
pub type EfSwSduDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_btdm_dis` reader - BTDM disable software override"]
pub type EfSwBtdmDisR = crate::BitReader;
#[doc = "Field `ef_sw_btdm_dis` writer - BTDM disable software override"]
pub type EfSwBtdmDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_wifi_dis` reader - WiFi disable software override"]
pub type EfSwWifiDisR = crate::BitReader;
#[doc = "Field `ef_sw_wifi_dis` writer - WiFi disable software override"]
pub type EfSwWifiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_0_key_enc_en` reader - Key encryption enable software override"]
pub type EfSw0KeyEncEnR = crate::BitReader;
#[doc = "Field `ef_sw_0_key_enc_en` writer - Key encryption enable software override"]
pub type EfSw0KeyEncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cam_dis` reader - CAM disable software override"]
pub type EfSwCamDisR = crate::BitReader;
#[doc = "Field `ef_sw_cam_dis` writer - CAM disable software override"]
pub type EfSwCamDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_m154_dis` reader - M154 disable software override"]
pub type EfSwM154DisR = crate::BitReader;
#[doc = "Field `ef_sw_m154_dis` writer - M154 disable software override"]
pub type EfSwM154DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cpu1_dis` reader - CPU1 disable software override"]
pub type EfSwCpu1DisR = crate::BitReader;
#[doc = "Field `ef_sw_cpu1_dis` writer - CPU1 disable software override"]
pub type EfSwCpu1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_cpu_rst_dbg_dis` reader - CPU reset debug disable software override"]
pub type EfSwCpuRstDbgDisR = crate::BitReader;
#[doc = "Field `ef_sw_cpu_rst_dbg_dis` writer - CPU reset debug disable software override"]
pub type EfSwCpuRstDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_se_dbg_dis` reader - Security engine debug disable software override"]
pub type EfSwSeDbgDisR = crate::BitReader;
#[doc = "Field `ef_sw_se_dbg_dis` writer - Security engine debug disable software override"]
pub type EfSwSeDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_efuse_dbg_dis` reader - eFuse debug disable software override"]
pub type EfSwEfuseDbgDisR = crate::BitReader;
#[doc = "Field `ef_sw_efuse_dbg_dis` writer - eFuse debug disable software override"]
pub type EfSwEfuseDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sw_dbg_jtag_1_dis` reader - JTAG 1 disable software override"]
pub type EfSwDbgJtag1DisR = crate::FieldReader;
#[doc = "Field `ef_sw_dbg_jtag_1_dis` writer - JTAG 1 disable software override"]
pub type EfSwDbgJtag1DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_dbg_jtag_0_dis` reader - JTAG 0 disable software override"]
pub type EfSwDbgJtag0DisR = crate::FieldReader;
#[doc = "Field `ef_sw_dbg_jtag_0_dis` writer - JTAG 0 disable software override"]
pub type EfSwDbgJtag0DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sw_dbg_mode` reader - Debug mode software override"]
pub type EfSwDbgModeR = crate::FieldReader;
#[doc = "Field `ef_sw_dbg_mode` writer - Debug mode software override"]
pub type EfSwDbgModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Flash AES mode software override"]
    #[inline(always)]
    pub fn ef_sw_sf_aes_mode(&self) -> EfSwSfAesModeR {
        EfSwSfAesModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Analog/audio interface disable software override"]
    #[inline(always)]
    pub fn ef_sw_ai_dis(&self) -> EfSwAiDisR {
        EfSwAiDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU0 disable software override"]
    #[inline(always)]
    pub fn ef_sw_cpu0_dis(&self) -> EfSwCpu0DisR {
        EfSwCpu0DisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Secure boot enable software override"]
    #[inline(always)]
    pub fn ef_sw_sboot_en(&self) -> EfSwSbootEnR {
        EfSwSbootEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - UART disable software override"]
    #[inline(always)]
    pub fn ef_sw_uart_dis(&self) -> EfSwUartDisR {
        EfSwUartDisR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - BLE2 disable software override"]
    #[inline(always)]
    pub fn ef_sw_ble2_dis(&self) -> EfSwBle2DisR {
        EfSwBle2DisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - M154_2 disable software override"]
    #[inline(always)]
    pub fn ef_sw_m1542_dis(&self) -> EfSwM1542DisR {
        EfSwM1542DisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - SF key reuse selection software override"]
    #[inline(always)]
    pub fn ef_sw_sf_key_re_sel(&self) -> EfSwSfKeyReSelR {
        EfSwSfKeyReSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - SDU disable software override"]
    #[inline(always)]
    pub fn ef_sw_sdu_dis(&self) -> EfSwSduDisR {
        EfSwSduDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BTDM disable software override"]
    #[inline(always)]
    pub fn ef_sw_btdm_dis(&self) -> EfSwBtdmDisR {
        EfSwBtdmDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WiFi disable software override"]
    #[inline(always)]
    pub fn ef_sw_wifi_dis(&self) -> EfSwWifiDisR {
        EfSwWifiDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key encryption enable software override"]
    #[inline(always)]
    pub fn ef_sw_0_key_enc_en(&self) -> EfSw0KeyEncEnR {
        EfSw0KeyEncEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CAM disable software override"]
    #[inline(always)]
    pub fn ef_sw_cam_dis(&self) -> EfSwCamDisR {
        EfSwCamDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - M154 disable software override"]
    #[inline(always)]
    pub fn ef_sw_m154_dis(&self) -> EfSwM154DisR {
        EfSwM154DisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU1 disable software override"]
    #[inline(always)]
    pub fn ef_sw_cpu1_dis(&self) -> EfSwCpu1DisR {
        EfSwCpu1DisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU reset debug disable software override"]
    #[inline(always)]
    pub fn ef_sw_cpu_rst_dbg_dis(&self) -> EfSwCpuRstDbgDisR {
        EfSwCpuRstDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security engine debug disable software override"]
    #[inline(always)]
    pub fn ef_sw_se_dbg_dis(&self) -> EfSwSeDbgDisR {
        EfSwSeDbgDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - eFuse debug disable software override"]
    #[inline(always)]
    pub fn ef_sw_efuse_dbg_dis(&self) -> EfSwEfuseDbgDisR {
        EfSwEfuseDbgDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - JTAG 1 disable software override"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_1_dis(&self) -> EfSwDbgJtag1DisR {
        EfSwDbgJtag1DisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - JTAG 0 disable software override"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_0_dis(&self) -> EfSwDbgJtag0DisR {
        EfSwDbgJtag0DisR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Debug mode software override"]
    #[inline(always)]
    pub fn ef_sw_dbg_mode(&self) -> EfSwDbgModeR {
        EfSwDbgModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash AES mode software override"]
    #[inline(always)]
    pub fn ef_sw_sf_aes_mode(&mut self) -> EfSwSfAesModeW<'_, EfCtrlSwCfg0Spec> {
        EfSwSfAesModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Analog/audio interface disable software override"]
    #[inline(always)]
    pub fn ef_sw_ai_dis(&mut self) -> EfSwAiDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwAiDisW::new(self, 2)
    }
    #[doc = "Bit 3 - CPU0 disable software override"]
    #[inline(always)]
    pub fn ef_sw_cpu0_dis(&mut self) -> EfSwCpu0DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwCpu0DisW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Secure boot enable software override"]
    #[inline(always)]
    pub fn ef_sw_sboot_en(&mut self) -> EfSwSbootEnW<'_, EfCtrlSwCfg0Spec> {
        EfSwSbootEnW::new(self, 4)
    }
    #[doc = "Bits 6:9 - UART disable software override"]
    #[inline(always)]
    pub fn ef_sw_uart_dis(&mut self) -> EfSwUartDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwUartDisW::new(self, 6)
    }
    #[doc = "Bit 10 - BLE2 disable software override"]
    #[inline(always)]
    pub fn ef_sw_ble2_dis(&mut self) -> EfSwBle2DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwBle2DisW::new(self, 10)
    }
    #[doc = "Bit 11 - M154_2 disable software override"]
    #[inline(always)]
    pub fn ef_sw_m1542_dis(&mut self) -> EfSwM1542DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwM1542DisW::new(self, 11)
    }
    #[doc = "Bits 12:13 - SF key reuse selection software override"]
    #[inline(always)]
    pub fn ef_sw_sf_key_re_sel(&mut self) -> EfSwSfKeyReSelW<'_, EfCtrlSwCfg0Spec> {
        EfSwSfKeyReSelW::new(self, 12)
    }
    #[doc = "Bit 14 - SDU disable software override"]
    #[inline(always)]
    pub fn ef_sw_sdu_dis(&mut self) -> EfSwSduDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwSduDisW::new(self, 14)
    }
    #[doc = "Bit 15 - BTDM disable software override"]
    #[inline(always)]
    pub fn ef_sw_btdm_dis(&mut self) -> EfSwBtdmDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwBtdmDisW::new(self, 15)
    }
    #[doc = "Bit 16 - WiFi disable software override"]
    #[inline(always)]
    pub fn ef_sw_wifi_dis(&mut self) -> EfSwWifiDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwWifiDisW::new(self, 16)
    }
    #[doc = "Bit 17 - Key encryption enable software override"]
    #[inline(always)]
    pub fn ef_sw_0_key_enc_en(&mut self) -> EfSw0KeyEncEnW<'_, EfCtrlSwCfg0Spec> {
        EfSw0KeyEncEnW::new(self, 17)
    }
    #[doc = "Bit 18 - CAM disable software override"]
    #[inline(always)]
    pub fn ef_sw_cam_dis(&mut self) -> EfSwCamDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwCamDisW::new(self, 18)
    }
    #[doc = "Bit 19 - M154 disable software override"]
    #[inline(always)]
    pub fn ef_sw_m154_dis(&mut self) -> EfSwM154DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwM154DisW::new(self, 19)
    }
    #[doc = "Bit 20 - CPU1 disable software override"]
    #[inline(always)]
    pub fn ef_sw_cpu1_dis(&mut self) -> EfSwCpu1DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwCpu1DisW::new(self, 20)
    }
    #[doc = "Bit 21 - CPU reset debug disable software override"]
    #[inline(always)]
    pub fn ef_sw_cpu_rst_dbg_dis(&mut self) -> EfSwCpuRstDbgDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwCpuRstDbgDisW::new(self, 21)
    }
    #[doc = "Bit 22 - Security engine debug disable software override"]
    #[inline(always)]
    pub fn ef_sw_se_dbg_dis(&mut self) -> EfSwSeDbgDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwSeDbgDisW::new(self, 22)
    }
    #[doc = "Bit 23 - eFuse debug disable software override"]
    #[inline(always)]
    pub fn ef_sw_efuse_dbg_dis(&mut self) -> EfSwEfuseDbgDisW<'_, EfCtrlSwCfg0Spec> {
        EfSwEfuseDbgDisW::new(self, 23)
    }
    #[doc = "Bits 24:25 - JTAG 1 disable software override"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_1_dis(&mut self) -> EfSwDbgJtag1DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwDbgJtag1DisW::new(self, 24)
    }
    #[doc = "Bits 26:27 - JTAG 0 disable software override"]
    #[inline(always)]
    pub fn ef_sw_dbg_jtag_0_dis(&mut self) -> EfSwDbgJtag0DisW<'_, EfCtrlSwCfg0Spec> {
        EfSwDbgJtag0DisW::new(self, 26)
    }
    #[doc = "Bits 28:31 - Debug mode software override"]
    #[inline(always)]
    pub fn ef_sw_dbg_mode(&mut self) -> EfSwDbgModeW<'_, EfCtrlSwCfg0Spec> {
        EfSwDbgModeW::new(self, 28)
    }
}
#[doc = "eFuse Software Override Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_sw_cfg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_sw_cfg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlSwCfg0Spec;
impl crate::RegisterSpec for EfCtrlSwCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_sw_cfg_0::R`](R) reader structure"]
impl crate::Readable for EfCtrlSwCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_sw_cfg_0::W`](W) writer structure"]
impl crate::Writable for EfCtrlSwCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_SW_CFG_0 to value 0"]
impl crate::Resettable for EfCtrlSwCfg0Spec {}
