#[doc = "Register `DPHY_CONFIG_2` reader"]
pub type R = crate::R<DphyConfig2Spec>;
#[doc = "Register `DPHY_CONFIG_2` writer"]
pub type W = crate::W<DphyConfig2Spec>;
#[doc = "Field `reg_ana_lprxen_clk` reader - Analog LP receive enable for clock lane"]
pub type RegAnaLprxenClkR = crate::BitReader;
#[doc = "Field `reg_ana_lprxen_clk` writer - Analog LP receive enable for clock lane"]
pub type RegAnaLprxenClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_hsrxen_clk` reader - Analog HS receive enable for clock lane"]
pub type RegAnaHsrxenClkR = crate::BitReader;
#[doc = "Field `reg_ana_hsrxen_clk` writer - Analog HS receive enable for clock lane"]
pub type RegAnaHsrxenClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_hsrx_stop_state` reader - Analog HS receive stop state control"]
pub type RegAnaHsrxStopStateR = crate::FieldReader;
#[doc = "Field `reg_ana_hsrx_stop_state` writer - Analog HS receive stop state control"]
pub type RegAnaHsrxStopStateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ana_hsrx_sync_en` reader - Analog HS receive synchronization enable"]
pub type RegAnaHsrxSyncEnR = crate::FieldReader;
#[doc = "Field `reg_ana_hsrx_sync_en` writer - Analog HS receive synchronization enable"]
pub type RegAnaHsrxSyncEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ana_lprxen` reader - Analog LP receive enable for data lanes"]
pub type RegAnaLprxenR = crate::FieldReader;
#[doc = "Field `reg_ana_lprxen` writer - Analog LP receive enable for data lanes"]
pub type RegAnaLprxenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ana_hsrxen` reader - Analog HS receive enable for data lanes"]
pub type RegAnaHsrxenR = crate::FieldReader;
#[doc = "Field `reg_ana_hsrxen` writer - Analog HS receive enable for data lanes"]
pub type RegAnaHsrxenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ana_term_en` reader - Analog termination enable control"]
pub type RegAnaTermEnR = crate::FieldReader;
#[doc = "Field `reg_ana_term_en` writer - Analog termination enable control"]
pub type RegAnaTermEnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_ana_test_en` reader - Analog test mode enable"]
pub type RegAnaTestEnR = crate::BitReader;
#[doc = "Field `reg_ana_test_en` writer - Analog test mode enable"]
pub type RegAnaTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_lock_counter` reader - Pattern test lock counter threshold"]
pub type RegPtLockCounterR = crate::FieldReader;
#[doc = "Field `reg_pt_lock_counter` writer - Pattern test lock counter threshold"]
pub type RegPtLockCounterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_pt_prbs_or_jitt` reader - Pattern test select: 0: PRBS, 1: Jitter"]
pub type RegPtPrbsOrJittR = crate::BitReader;
#[doc = "Field `reg_pt_prbs_or_jitt` writer - Pattern test select: 0: PRBS, 1: Jitter"]
pub type RegPtPrbsOrJittW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_lp_mode` reader - Pattern test LP mode enable"]
pub type RegPtLpModeR = crate::BitReader;
#[doc = "Field `reg_pt_lp_mode` writer - Pattern test LP mode enable"]
pub type RegPtLpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_en` reader - Pattern test enable"]
pub type RegPtEnR = crate::BitReader;
#[doc = "Field `reg_pt_en` writer - Pattern test enable"]
pub type RegPtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_lock` reader - Pattern test lock status (read-only)"]
pub type RegPtLockR = crate::BitReader;
#[doc = "Field `reg_pt_pass` reader - Pattern test pass status (read-only)"]
pub type RegPtPassR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog LP receive enable for clock lane"]
    #[inline(always)]
    pub fn reg_ana_lprxen_clk(&self) -> RegAnaLprxenClkR {
        RegAnaLprxenClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog HS receive enable for clock lane"]
    #[inline(always)]
    pub fn reg_ana_hsrxen_clk(&self) -> RegAnaHsrxenClkR {
        RegAnaHsrxenClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Analog HS receive stop state control"]
    #[inline(always)]
    pub fn reg_ana_hsrx_stop_state(&self) -> RegAnaHsrxStopStateR {
        RegAnaHsrxStopStateR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Analog HS receive synchronization enable"]
    #[inline(always)]
    pub fn reg_ana_hsrx_sync_en(&self) -> RegAnaHsrxSyncEnR {
        RegAnaHsrxSyncEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Analog LP receive enable for data lanes"]
    #[inline(always)]
    pub fn reg_ana_lprxen(&self) -> RegAnaLprxenR {
        RegAnaLprxenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Analog HS receive enable for data lanes"]
    #[inline(always)]
    pub fn reg_ana_hsrxen(&self) -> RegAnaHsrxenR {
        RegAnaHsrxenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:14 - Analog termination enable control"]
    #[inline(always)]
    pub fn reg_ana_term_en(&self) -> RegAnaTermEnR {
        RegAnaTermEnR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Analog test mode enable"]
    #[inline(always)]
    pub fn reg_ana_test_en(&self) -> RegAnaTestEnR {
        RegAnaTestEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Pattern test lock counter threshold"]
    #[inline(always)]
    pub fn reg_pt_lock_counter(&self) -> RegPtLockCounterR {
        RegPtLockCounterR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Pattern test select: 0: PRBS, 1: Jitter"]
    #[inline(always)]
    pub fn reg_pt_prbs_or_jitt(&self) -> RegPtPrbsOrJittR {
        RegPtPrbsOrJittR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pattern test LP mode enable"]
    #[inline(always)]
    pub fn reg_pt_lp_mode(&self) -> RegPtLpModeR {
        RegPtLpModeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pattern test enable"]
    #[inline(always)]
    pub fn reg_pt_en(&self) -> RegPtEnR {
        RegPtEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pattern test lock status (read-only)"]
    #[inline(always)]
    pub fn reg_pt_lock(&self) -> RegPtLockR {
        RegPtLockR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pattern test pass status (read-only)"]
    #[inline(always)]
    pub fn reg_pt_pass(&self) -> RegPtPassR {
        RegPtPassR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog LP receive enable for clock lane"]
    #[inline(always)]
    pub fn reg_ana_lprxen_clk(&mut self) -> RegAnaLprxenClkW<'_, DphyConfig2Spec> {
        RegAnaLprxenClkW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog HS receive enable for clock lane"]
    #[inline(always)]
    pub fn reg_ana_hsrxen_clk(&mut self) -> RegAnaHsrxenClkW<'_, DphyConfig2Spec> {
        RegAnaHsrxenClkW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Analog HS receive stop state control"]
    #[inline(always)]
    pub fn reg_ana_hsrx_stop_state(&mut self) -> RegAnaHsrxStopStateW<'_, DphyConfig2Spec> {
        RegAnaHsrxStopStateW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Analog HS receive synchronization enable"]
    #[inline(always)]
    pub fn reg_ana_hsrx_sync_en(&mut self) -> RegAnaHsrxSyncEnW<'_, DphyConfig2Spec> {
        RegAnaHsrxSyncEnW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Analog LP receive enable for data lanes"]
    #[inline(always)]
    pub fn reg_ana_lprxen(&mut self) -> RegAnaLprxenW<'_, DphyConfig2Spec> {
        RegAnaLprxenW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Analog HS receive enable for data lanes"]
    #[inline(always)]
    pub fn reg_ana_hsrxen(&mut self) -> RegAnaHsrxenW<'_, DphyConfig2Spec> {
        RegAnaHsrxenW::new(self, 8)
    }
    #[doc = "Bits 10:14 - Analog termination enable control"]
    #[inline(always)]
    pub fn reg_ana_term_en(&mut self) -> RegAnaTermEnW<'_, DphyConfig2Spec> {
        RegAnaTermEnW::new(self, 10)
    }
    #[doc = "Bit 15 - Analog test mode enable"]
    #[inline(always)]
    pub fn reg_ana_test_en(&mut self) -> RegAnaTestEnW<'_, DphyConfig2Spec> {
        RegAnaTestEnW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Pattern test lock counter threshold"]
    #[inline(always)]
    pub fn reg_pt_lock_counter(&mut self) -> RegPtLockCounterW<'_, DphyConfig2Spec> {
        RegPtLockCounterW::new(self, 16)
    }
    #[doc = "Bit 20 - Pattern test select: 0: PRBS, 1: Jitter"]
    #[inline(always)]
    pub fn reg_pt_prbs_or_jitt(&mut self) -> RegPtPrbsOrJittW<'_, DphyConfig2Spec> {
        RegPtPrbsOrJittW::new(self, 20)
    }
    #[doc = "Bit 21 - Pattern test LP mode enable"]
    #[inline(always)]
    pub fn reg_pt_lp_mode(&mut self) -> RegPtLpModeW<'_, DphyConfig2Spec> {
        RegPtLpModeW::new(self, 21)
    }
    #[doc = "Bit 22 - Pattern test enable"]
    #[inline(always)]
    pub fn reg_pt_en(&mut self) -> RegPtEnW<'_, DphyConfig2Spec> {
        RegPtEnW::new(self, 22)
    }
}
#[doc = "D-PHY Analog Configuration and Test\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig2Spec;
impl crate::RegisterSpec for DphyConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_2::R`](R) reader structure"]
impl crate::Readable for DphyConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_2::W`](W) writer structure"]
impl crate::Writable for DphyConfig2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_2 to value 0"]
impl crate::Resettable for DphyConfig2Spec {}
