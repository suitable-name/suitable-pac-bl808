#[doc = "Register `EF_CTRL_IF_CFG_0` reader"]
pub type R = crate::R<EfCtrlIfCfg0Spec>;
#[doc = "Register `EF_CTRL_IF_CFG_0` writer"]
pub type W = crate::W<EfCtrlIfCfg0Spec>;
#[doc = "Field `ef_if_sf_aes_mode` reader - Flash AES mode read from eFuse"]
pub type EfIfSfAesModeR = crate::FieldReader;
#[doc = "Field `ef_if_ai_dis` reader - Analog/audio interface disable read from eFuse"]
pub type EfIfAiDisR = crate::BitReader;
#[doc = "Field `ef_if_cpu0_dis` reader - CPU0 disable read from eFuse"]
pub type EfIfCpu0DisR = crate::BitReader;
#[doc = "Field `ef_if_sboot_en` reader - Secure boot enable read from eFuse"]
pub type EfIfSbootEnR = crate::FieldReader;
#[doc = "Field `ef_if_uart_dis` reader - UART disable read from eFuse"]
pub type EfIfUartDisR = crate::FieldReader;
#[doc = "Field `ef_if_ble2_dis` reader - BLE2 disable read from eFuse"]
pub type EfIfBle2DisR = crate::BitReader;
#[doc = "Field `ef_if_m1542_dis` reader - M154_2 disable read from eFuse"]
pub type EfIfM1542DisR = crate::BitReader;
#[doc = "Field `ef_if_sf_key_re_sel` reader - SF key reuse selection read from eFuse"]
pub type EfIfSfKeyReSelR = crate::FieldReader;
#[doc = "Field `ef_if_sdu_dis` reader - SDU disable read from eFuse"]
pub type EfIfSduDisR = crate::BitReader;
#[doc = "Field `ef_if_btdm_dis` reader - BTDM disable read from eFuse"]
pub type EfIfBtdmDisR = crate::BitReader;
#[doc = "Field `ef_if_wifi_dis` reader - WiFi disable read from eFuse"]
pub type EfIfWifiDisR = crate::BitReader;
#[doc = "Field `ef_if_0_key_enc_en` reader - Key encryption enable read from eFuse"]
pub type EfIf0KeyEncEnR = crate::BitReader;
#[doc = "Field `ef_if_cam_dis` reader - CAM disable read from eFuse"]
pub type EfIfCamDisR = crate::BitReader;
#[doc = "Field `ef_if_m154_dis` reader - M154 disable read from eFuse"]
pub type EfIfM154DisR = crate::BitReader;
#[doc = "Field `ef_if_cpu1_dis` reader - CPU1 disable read from eFuse"]
pub type EfIfCpu1DisR = crate::BitReader;
#[doc = "Field `ef_if_cpu_rst_dbg_dis` reader - CPU reset debug disable read from eFuse"]
pub type EfIfCpuRstDbgDisR = crate::BitReader;
#[doc = "Field `ef_if_se_dbg_dis` reader - Security engine debug disable read from eFuse"]
pub type EfIfSeDbgDisR = crate::BitReader;
#[doc = "Field `ef_if_efuse_dbg_dis` reader - eFuse debug disable read from eFuse"]
pub type EfIfEfuseDbgDisR = crate::BitReader;
#[doc = "Field `ef_if_dbg_jtag_1_dis` reader - JTAG 1 disable read from eFuse"]
pub type EfIfDbgJtag1DisR = crate::FieldReader;
#[doc = "Field `ef_if_dbg_jtag_0_dis` reader - JTAG 0 disable read from eFuse"]
pub type EfIfDbgJtag0DisR = crate::FieldReader;
#[doc = "Field `ef_if_dbg_mode` reader - Debug mode read from eFuse"]
pub type EfIfDbgModeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Flash AES mode read from eFuse"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&self) -> EfIfSfAesModeR {
        EfIfSfAesModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Analog/audio interface disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_ai_dis(&self) -> EfIfAiDisR {
        EfIfAiDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU0 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_cpu0_dis(&self) -> EfIfCpu0DisR {
        EfIfCpu0DisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Secure boot enable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&self) -> EfIfSbootEnR {
        EfIfSbootEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - UART disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_uart_dis(&self) -> EfIfUartDisR {
        EfIfUartDisR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - BLE2 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_ble2_dis(&self) -> EfIfBle2DisR {
        EfIfBle2DisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - M154_2 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_m1542_dis(&self) -> EfIfM1542DisR {
        EfIfM1542DisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - SF key reuse selection read from eFuse"]
    #[inline(always)]
    pub fn ef_if_sf_key_re_sel(&self) -> EfIfSfKeyReSelR {
        EfIfSfKeyReSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - SDU disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&self) -> EfIfSduDisR {
        EfIfSduDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BTDM disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_btdm_dis(&self) -> EfIfBtdmDisR {
        EfIfBtdmDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WiFi disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&self) -> EfIfWifiDisR {
        EfIfWifiDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key encryption enable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&self) -> EfIf0KeyEncEnR {
        EfIf0KeyEncEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CAM disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&self) -> EfIfCamDisR {
        EfIfCamDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - M154 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_m154_dis(&self) -> EfIfM154DisR {
        EfIfM154DisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU1 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&self) -> EfIfCpu1DisR {
        EfIfCpu1DisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU reset debug disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&self) -> EfIfCpuRstDbgDisR {
        EfIfCpuRstDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security engine debug disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&self) -> EfIfSeDbgDisR {
        EfIfSeDbgDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - eFuse debug disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&self) -> EfIfEfuseDbgDisR {
        EfIfEfuseDbgDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - JTAG 1 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&self) -> EfIfDbgJtag1DisR {
        EfIfDbgJtag1DisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - JTAG 0 disable read from eFuse"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&self) -> EfIfDbgJtag0DisR {
        EfIfDbgJtag0DisR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Debug mode read from eFuse"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&self) -> EfIfDbgModeR {
        EfIfDbgModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "eFuse Interface Configuration (Read values from array)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_cfg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_cfg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfCfg0Spec;
impl crate::RegisterSpec for EfCtrlIfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_cfg_0::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_cfg_0::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_CFG_0 to value 0"]
impl crate::Resettable for EfCtrlIfCfg0Spec {}
