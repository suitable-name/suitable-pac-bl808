#[doc = "Register `PHY_CFG_48` reader"]
pub type R = crate::R<PhyCfg48Spec>;
#[doc = "Register `PHY_CFG_48` writer"]
pub type W = crate::W<PhyCfg48Spec>;
#[doc = "Field `tmux` reader - Test Mux selection"]
pub type TmuxR = crate::FieldReader;
#[doc = "Field `tmux` writer - Test Mux selection"]
pub type TmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `chip_en_33` reader - Chip 3.3V enable"]
pub type ChipEn33R = crate::BitReader;
#[doc = "Field `chip_en_33` writer - Chip 3.3V enable"]
pub type ChipEn33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tmux_uhs_phy_dig` reader - Digital test mux selection"]
pub type TmuxUhsPhyDigR = crate::FieldReader;
#[doc = "Field `tmux_uhs_phy_dig` writer - Digital test mux selection"]
pub type TmuxUhsPhyDigW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `psram_type` reader - PSRAM type selection"]
pub type PsramTypeR = crate::FieldReader;
#[doc = "Field `psram_type` writer - PSRAM type selection"]
pub type PsramTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pu_uhs_pw1p8` reader - UHS 1.8V power up"]
pub type PuUhsPw1p8R = crate::BitReader;
#[doc = "Field `pu_uhs_pw1p8` writer - UHS 1.8V power up"]
pub type PuUhsPw1p8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_test_div_sel` reader - Test divider selection"]
pub type RegTestDivSelR = crate::FieldReader;
#[doc = "Field `reg_test_div_sel` writer - Test divider selection"]
pub type RegTestDivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `en_rx_fe_hw` reader - Hardware RX FE enable"]
pub type EnRxFeHwR = crate::BitReader;
#[doc = "Field `en_rx_fe_hw` writer - Hardware RX FE enable"]
pub type EnRxFeHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_test_mux_sel` reader - Test multiplexer selection"]
pub type RegTestMuxSelR = crate::FieldReader;
#[doc = "Field `reg_test_mux_sel` writer - Test multiplexer selection"]
pub type RegTestMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `force_fsm` reader - Force FSM state"]
pub type ForceFsmR = crate::BitReader;
#[doc = "Field `force_fsm` writer - Force FSM state"]
pub type ForceFsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_rx_fe_dly` reader - RX FE delay enable"]
pub type EnRxFeDlyR = crate::FieldReader;
#[doc = "Field `en_rx_fe_dly` writer - RX FE delay enable"]
pub type EnRxFeDlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Test Mux selection"]
    #[inline(always)]
    pub fn tmux(&self) -> TmuxR {
        TmuxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Chip 3.3V enable"]
    #[inline(always)]
    pub fn chip_en_33(&self) -> ChipEn33R {
        ChipEn33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Digital test mux selection"]
    #[inline(always)]
    pub fn tmux_uhs_phy_dig(&self) -> TmuxUhsPhyDigR {
        TmuxUhsPhyDigR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - PSRAM type selection"]
    #[inline(always)]
    pub fn psram_type(&self) -> PsramTypeR {
        PsramTypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UHS 1.8V power up"]
    #[inline(always)]
    pub fn pu_uhs_pw1p8(&self) -> PuUhsPw1p8R {
        PuUhsPw1p8R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Test divider selection"]
    #[inline(always)]
    pub fn reg_test_div_sel(&self) -> RegTestDivSelR {
        RegTestDivSelR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Hardware RX FE enable"]
    #[inline(always)]
    pub fn en_rx_fe_hw(&self) -> EnRxFeHwR {
        EnRxFeHwR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Test multiplexer selection"]
    #[inline(always)]
    pub fn reg_test_mux_sel(&self) -> RegTestMuxSelR {
        RegTestMuxSelR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Force FSM state"]
    #[inline(always)]
    pub fn force_fsm(&self) -> ForceFsmR {
        ForceFsmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - RX FE delay enable"]
    #[inline(always)]
    pub fn en_rx_fe_dly(&self) -> EnRxFeDlyR {
        EnRxFeDlyR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Test Mux selection"]
    #[inline(always)]
    pub fn tmux(&mut self) -> TmuxW<'_, PhyCfg48Spec> {
        TmuxW::new(self, 0)
    }
    #[doc = "Bit 3 - Chip 3.3V enable"]
    #[inline(always)]
    pub fn chip_en_33(&mut self) -> ChipEn33W<'_, PhyCfg48Spec> {
        ChipEn33W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Digital test mux selection"]
    #[inline(always)]
    pub fn tmux_uhs_phy_dig(&mut self) -> TmuxUhsPhyDigW<'_, PhyCfg48Spec> {
        TmuxUhsPhyDigW::new(self, 4)
    }
    #[doc = "Bits 8:9 - PSRAM type selection"]
    #[inline(always)]
    pub fn psram_type(&mut self) -> PsramTypeW<'_, PhyCfg48Spec> {
        PsramTypeW::new(self, 8)
    }
    #[doc = "Bit 11 - UHS 1.8V power up"]
    #[inline(always)]
    pub fn pu_uhs_pw1p8(&mut self) -> PuUhsPw1p8W<'_, PhyCfg48Spec> {
        PuUhsPw1p8W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Test divider selection"]
    #[inline(always)]
    pub fn reg_test_div_sel(&mut self) -> RegTestDivSelW<'_, PhyCfg48Spec> {
        RegTestDivSelW::new(self, 12)
    }
    #[doc = "Bit 15 - Hardware RX FE enable"]
    #[inline(always)]
    pub fn en_rx_fe_hw(&mut self) -> EnRxFeHwW<'_, PhyCfg48Spec> {
        EnRxFeHwW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Test multiplexer selection"]
    #[inline(always)]
    pub fn reg_test_mux_sel(&mut self) -> RegTestMuxSelW<'_, PhyCfg48Spec> {
        RegTestMuxSelW::new(self, 16)
    }
    #[doc = "Bit 19 - Force FSM state"]
    #[inline(always)]
    pub fn force_fsm(&mut self) -> ForceFsmW<'_, PhyCfg48Spec> {
        ForceFsmW::new(self, 19)
    }
    #[doc = "Bits 20:23 - RX FE delay enable"]
    #[inline(always)]
    pub fn en_rx_fe_dly(&mut self) -> EnRxFeDlyW<'_, PhyCfg48Spec> {
        EnRxFeDlyW::new(self, 20)
    }
}
#[doc = "PHY Test and Multiplexer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg48Spec;
impl crate::RegisterSpec for PhyCfg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_48::R`](R) reader structure"]
impl crate::Readable for PhyCfg48Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_48::W`](W) writer structure"]
impl crate::Writable for PhyCfg48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_48 to value 0x0020_8908"]
impl crate::Resettable for PhyCfg48Spec {
    const RESET_VALUE: u32 = 0x0020_8908;
}
