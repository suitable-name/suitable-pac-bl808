#[doc = "Register `EF_CFG_0` reader"]
pub type R = crate::R<EfCfg0Spec>;
#[doc = "Register `EF_CFG_0` writer"]
pub type W = crate::W<EfCfg0Spec>;
#[doc = "Field `ef_sf_aes_mode` reader - Flash AES mode: 0:None, 1:AES-128, 2:AES-256, 3:AES-192"]
pub type EfSfAesModeR = crate::FieldReader;
#[doc = "Field `ef_sf_aes_mode` writer - Flash AES mode: 0:None, 1:AES-128, 2:AES-256, 3:AES-192"]
pub type EfSfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_ai_dis` reader - Analog/audio interface disable"]
pub type EfAiDisR = crate::BitReader;
#[doc = "Field `ef_ai_dis` writer - Analog/audio interface disable"]
pub type EfAiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cpu0_dis` reader - CPU0 (M0) disable"]
pub type EfCpu0DisR = crate::BitReader;
#[doc = "Field `ef_cpu0_dis` writer - CPU0 (M0) disable"]
pub type EfCpu0DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sboot_en` reader - Secure boot enable"]
pub type EfSbootEnR = crate::FieldReader;
#[doc = "Field `ef_sboot_en` writer - Secure boot enable"]
pub type EfSbootEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_uart_dis` reader - UART disable mask (4 UARTs)"]
pub type EfUartDisR = crate::FieldReader;
#[doc = "Field `ef_uart_dis` writer - UART disable mask (4 UARTs)"]
pub type EfUartDisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ef_ble2_dis` reader - BLE 2.0 disable"]
pub type EfBle2DisR = crate::BitReader;
#[doc = "Field `ef_ble2_dis` writer - BLE 2.0 disable"]
pub type EfBle2DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_m1542_dis` reader - M154_2 disable"]
pub type EfM1542DisR = crate::BitReader;
#[doc = "Field `ef_m1542_dis` writer - M154_2 disable"]
pub type EfM1542DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_sf_key_re_sel` reader - SF key reuse selection"]
pub type EfSfKeyReSelR = crate::FieldReader;
#[doc = "Field `ef_sf_key_re_sel` writer - SF key reuse selection"]
pub type EfSfKeyReSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_sdu_dis` reader - SDU disable"]
pub type EfSduDisR = crate::BitReader;
#[doc = "Field `ef_sdu_dis` writer - SDU disable"]
pub type EfSduDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_btdm_dis` reader - BTDM disable"]
pub type EfBtdmDisR = crate::BitReader;
#[doc = "Field `ef_btdm_dis` writer - BTDM disable"]
pub type EfBtdmDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_wifi_dis` reader - WiFi disable"]
pub type EfWifiDisR = crate::BitReader;
#[doc = "Field `ef_wifi_dis` writer - WiFi disable"]
pub type EfWifiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_0_key_enc_en` reader - Key encryption enable"]
pub type Ef0KeyEncEnR = crate::BitReader;
#[doc = "Field `ef_0_key_enc_en` writer - Key encryption enable"]
pub type Ef0KeyEncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cam_dis` reader - CAM disable"]
pub type EfCamDisR = crate::BitReader;
#[doc = "Field `ef_cam_dis` writer - CAM disable"]
pub type EfCamDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_m154_dis` reader - M154 disable"]
pub type EfM154DisR = crate::BitReader;
#[doc = "Field `ef_m154_dis` writer - M154 disable"]
pub type EfM154DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cpu1_dis` reader - CPU1 (D0) disable"]
pub type EfCpu1DisR = crate::BitReader;
#[doc = "Field `ef_cpu1_dis` writer - CPU1 (D0) disable"]
pub type EfCpu1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_cpu_rst_dbg_dis` reader - CPU reset debug disable"]
pub type EfCpuRstDbgDisR = crate::BitReader;
#[doc = "Field `ef_cpu_rst_dbg_dis` writer - CPU reset debug disable"]
pub type EfCpuRstDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_se_dbg_dis` reader - Security engine debug disable"]
pub type EfSeDbgDisR = crate::BitReader;
#[doc = "Field `ef_se_dbg_dis` writer - Security engine debug disable"]
pub type EfSeDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_efuse_dbg_dis` reader - eFuse debug disable"]
pub type EfEfuseDbgDisR = crate::BitReader;
#[doc = "Field `ef_efuse_dbg_dis` writer - eFuse debug disable"]
pub type EfEfuseDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_dbg_jtag_1_dis` reader - JTAG 1 disable status"]
pub type EfDbgJtag1DisR = crate::FieldReader;
#[doc = "Field `ef_dbg_jtag_1_dis` writer - JTAG 1 disable status"]
pub type EfDbgJtag1DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_dbg_jtag_0_dis` reader - JTAG 0 disable status"]
pub type EfDbgJtag0DisR = crate::FieldReader;
#[doc = "Field `ef_dbg_jtag_0_dis` writer - JTAG 0 disable status"]
pub type EfDbgJtag0DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_dbg_mode` reader - Debug mode selection"]
pub type EfDbgModeR = crate::FieldReader;
#[doc = "Field `ef_dbg_mode` writer - Debug mode selection"]
pub type EfDbgModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Flash AES mode: 0:None, 1:AES-128, 2:AES-256, 3:AES-192"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&self) -> EfSfAesModeR {
        EfSfAesModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Analog/audio interface disable"]
    #[inline(always)]
    pub fn ef_ai_dis(&self) -> EfAiDisR {
        EfAiDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU0 (M0) disable"]
    #[inline(always)]
    pub fn ef_cpu0_dis(&self) -> EfCpu0DisR {
        EfCpu0DisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Secure boot enable"]
    #[inline(always)]
    pub fn ef_sboot_en(&self) -> EfSbootEnR {
        EfSbootEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - UART disable mask (4 UARTs)"]
    #[inline(always)]
    pub fn ef_uart_dis(&self) -> EfUartDisR {
        EfUartDisR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - BLE 2.0 disable"]
    #[inline(always)]
    pub fn ef_ble2_dis(&self) -> EfBle2DisR {
        EfBle2DisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - M154_2 disable"]
    #[inline(always)]
    pub fn ef_m1542_dis(&self) -> EfM1542DisR {
        EfM1542DisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - SF key reuse selection"]
    #[inline(always)]
    pub fn ef_sf_key_re_sel(&self) -> EfSfKeyReSelR {
        EfSfKeyReSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - SDU disable"]
    #[inline(always)]
    pub fn ef_sdu_dis(&self) -> EfSduDisR {
        EfSduDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BTDM disable"]
    #[inline(always)]
    pub fn ef_btdm_dis(&self) -> EfBtdmDisR {
        EfBtdmDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WiFi disable"]
    #[inline(always)]
    pub fn ef_wifi_dis(&self) -> EfWifiDisR {
        EfWifiDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key encryption enable"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&self) -> Ef0KeyEncEnR {
        Ef0KeyEncEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CAM disable"]
    #[inline(always)]
    pub fn ef_cam_dis(&self) -> EfCamDisR {
        EfCamDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - M154 disable"]
    #[inline(always)]
    pub fn ef_m154_dis(&self) -> EfM154DisR {
        EfM154DisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU1 (D0) disable"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&self) -> EfCpu1DisR {
        EfCpu1DisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU reset debug disable"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&self) -> EfCpuRstDbgDisR {
        EfCpuRstDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security engine debug disable"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&self) -> EfSeDbgDisR {
        EfSeDbgDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - eFuse debug disable"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&self) -> EfEfuseDbgDisR {
        EfEfuseDbgDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - JTAG 1 disable status"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&self) -> EfDbgJtag1DisR {
        EfDbgJtag1DisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - JTAG 0 disable status"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&self) -> EfDbgJtag0DisR {
        EfDbgJtag0DisR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Debug mode selection"]
    #[inline(always)]
    pub fn ef_dbg_mode(&self) -> EfDbgModeR {
        EfDbgModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash AES mode: 0:None, 1:AES-128, 2:AES-256, 3:AES-192"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&mut self) -> EfSfAesModeW<'_, EfCfg0Spec> {
        EfSfAesModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Analog/audio interface disable"]
    #[inline(always)]
    pub fn ef_ai_dis(&mut self) -> EfAiDisW<'_, EfCfg0Spec> {
        EfAiDisW::new(self, 2)
    }
    #[doc = "Bit 3 - CPU0 (M0) disable"]
    #[inline(always)]
    pub fn ef_cpu0_dis(&mut self) -> EfCpu0DisW<'_, EfCfg0Spec> {
        EfCpu0DisW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Secure boot enable"]
    #[inline(always)]
    pub fn ef_sboot_en(&mut self) -> EfSbootEnW<'_, EfCfg0Spec> {
        EfSbootEnW::new(self, 4)
    }
    #[doc = "Bits 6:9 - UART disable mask (4 UARTs)"]
    #[inline(always)]
    pub fn ef_uart_dis(&mut self) -> EfUartDisW<'_, EfCfg0Spec> {
        EfUartDisW::new(self, 6)
    }
    #[doc = "Bit 10 - BLE 2.0 disable"]
    #[inline(always)]
    pub fn ef_ble2_dis(&mut self) -> EfBle2DisW<'_, EfCfg0Spec> {
        EfBle2DisW::new(self, 10)
    }
    #[doc = "Bit 11 - M154_2 disable"]
    #[inline(always)]
    pub fn ef_m1542_dis(&mut self) -> EfM1542DisW<'_, EfCfg0Spec> {
        EfM1542DisW::new(self, 11)
    }
    #[doc = "Bits 12:13 - SF key reuse selection"]
    #[inline(always)]
    pub fn ef_sf_key_re_sel(&mut self) -> EfSfKeyReSelW<'_, EfCfg0Spec> {
        EfSfKeyReSelW::new(self, 12)
    }
    #[doc = "Bit 14 - SDU disable"]
    #[inline(always)]
    pub fn ef_sdu_dis(&mut self) -> EfSduDisW<'_, EfCfg0Spec> {
        EfSduDisW::new(self, 14)
    }
    #[doc = "Bit 15 - BTDM disable"]
    #[inline(always)]
    pub fn ef_btdm_dis(&mut self) -> EfBtdmDisW<'_, EfCfg0Spec> {
        EfBtdmDisW::new(self, 15)
    }
    #[doc = "Bit 16 - WiFi disable"]
    #[inline(always)]
    pub fn ef_wifi_dis(&mut self) -> EfWifiDisW<'_, EfCfg0Spec> {
        EfWifiDisW::new(self, 16)
    }
    #[doc = "Bit 17 - Key encryption enable"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&mut self) -> Ef0KeyEncEnW<'_, EfCfg0Spec> {
        Ef0KeyEncEnW::new(self, 17)
    }
    #[doc = "Bit 18 - CAM disable"]
    #[inline(always)]
    pub fn ef_cam_dis(&mut self) -> EfCamDisW<'_, EfCfg0Spec> {
        EfCamDisW::new(self, 18)
    }
    #[doc = "Bit 19 - M154 disable"]
    #[inline(always)]
    pub fn ef_m154_dis(&mut self) -> EfM154DisW<'_, EfCfg0Spec> {
        EfM154DisW::new(self, 19)
    }
    #[doc = "Bit 20 - CPU1 (D0) disable"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&mut self) -> EfCpu1DisW<'_, EfCfg0Spec> {
        EfCpu1DisW::new(self, 20)
    }
    #[doc = "Bit 21 - CPU reset debug disable"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&mut self) -> EfCpuRstDbgDisW<'_, EfCfg0Spec> {
        EfCpuRstDbgDisW::new(self, 21)
    }
    #[doc = "Bit 22 - Security engine debug disable"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&mut self) -> EfSeDbgDisW<'_, EfCfg0Spec> {
        EfSeDbgDisW::new(self, 22)
    }
    #[doc = "Bit 23 - eFuse debug disable"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&mut self) -> EfEfuseDbgDisW<'_, EfCfg0Spec> {
        EfEfuseDbgDisW::new(self, 23)
    }
    #[doc = "Bits 24:25 - JTAG 1 disable status"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&mut self) -> EfDbgJtag1DisW<'_, EfCfg0Spec> {
        EfDbgJtag1DisW::new(self, 24)
    }
    #[doc = "Bits 26:27 - JTAG 0 disable status"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&mut self) -> EfDbgJtag0DisW<'_, EfCfg0Spec> {
        EfDbgJtag0DisW::new(self, 26)
    }
    #[doc = "Bits 28:31 - Debug mode selection"]
    #[inline(always)]
    pub fn ef_dbg_mode(&mut self) -> EfDbgModeW<'_, EfCfg0Spec> {
        EfDbgModeW::new(self, 28)
    }
}
#[doc = "eFuse Configuration Settings (Bitstream Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_cfg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_cfg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCfg0Spec;
impl crate::RegisterSpec for EfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_cfg_0::R`](R) reader structure"]
impl crate::Readable for EfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_cfg_0::W`](W) writer structure"]
impl crate::Writable for EfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CFG_0 to value 0"]
impl crate::Resettable for EfCfg0Spec {}
