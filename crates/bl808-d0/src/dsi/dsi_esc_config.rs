#[doc = "Register `DSI_ESC_CONFIG` reader"]
pub type R = crate::R<DsiEscConfigSpec>;
#[doc = "Register `DSI_ESC_CONFIG` writer"]
pub type W = crate::W<DsiEscConfigSpec>;
#[doc = "Field `cr_esc_tx_en` writer - Escape TX enable pulse"]
pub type CrEscTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_esc_tx_mode` reader - Escape TX mode"]
pub type CrEscTxModeR = crate::FieldReader;
#[doc = "Field `cr_esc_tx_mode` writer - Escape TX mode"]
pub type CrEscTxModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_esc_tx_ulps_exit` writer - Escape TX ULPS exit pulse"]
pub type CrEscTxUlpsExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_esc_tx_trig` reader - Escape TX trigger"]
pub type CrEscTxTrigR = crate::FieldReader;
#[doc = "Field `cr_esc_tx_trig` writer - Escape TX trigger"]
pub type CrEscTxTrigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_esc_rx_en` reader - Escape RX enable"]
pub type CrEscRxEnR = crate::BitReader;
#[doc = "Field `cr_esc_rx_en` writer - Escape RX enable"]
pub type CrEscRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `st_esc_rx_len` reader - Escape RX received length"]
pub type StEscRxLenR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:2 - Escape TX mode"]
    #[inline(always)]
    pub fn cr_esc_tx_mode(&self) -> CrEscTxModeR {
        CrEscTxModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Escape TX trigger"]
    #[inline(always)]
    pub fn cr_esc_tx_trig(&self) -> CrEscTxTrigR {
        CrEscTxTrigR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Escape RX enable"]
    #[inline(always)]
    pub fn cr_esc_rx_en(&self) -> CrEscRxEnR {
        CrEscRxEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Escape RX received length"]
    #[inline(always)]
    pub fn st_esc_rx_len(&self) -> StEscRxLenR {
        StEscRxLenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Escape TX enable pulse"]
    #[inline(always)]
    pub fn cr_esc_tx_en(&mut self) -> CrEscTxEnW<'_, DsiEscConfigSpec> {
        CrEscTxEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Escape TX mode"]
    #[inline(always)]
    pub fn cr_esc_tx_mode(&mut self) -> CrEscTxModeW<'_, DsiEscConfigSpec> {
        CrEscTxModeW::new(self, 1)
    }
    #[doc = "Bit 3 - Escape TX ULPS exit pulse"]
    #[inline(always)]
    pub fn cr_esc_tx_ulps_exit(&mut self) -> CrEscTxUlpsExitW<'_, DsiEscConfigSpec> {
        CrEscTxUlpsExitW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Escape TX trigger"]
    #[inline(always)]
    pub fn cr_esc_tx_trig(&mut self) -> CrEscTxTrigW<'_, DsiEscConfigSpec> {
        CrEscTxTrigW::new(self, 4)
    }
    #[doc = "Bit 16 - Escape RX enable"]
    #[inline(always)]
    pub fn cr_esc_rx_en(&mut self) -> CrEscRxEnW<'_, DsiEscConfigSpec> {
        CrEscRxEnW::new(self, 16)
    }
}
#[doc = "Escape Mode Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_esc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_esc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiEscConfigSpec;
impl crate::RegisterSpec for DsiEscConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_esc_config::R`](R) reader structure"]
impl crate::Readable for DsiEscConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_esc_config::W`](W) writer structure"]
impl crate::Writable for DsiEscConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_ESC_CONFIG to value 0"]
impl crate::Resettable for DsiEscConfigSpec {}
