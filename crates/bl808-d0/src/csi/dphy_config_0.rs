#[doc = "Register `DPHY_CONFIG_0` reader"]
pub type R = crate::R<DphyConfig0Spec>;
#[doc = "Register `DPHY_CONFIG_0` writer"]
pub type W = crate::W<DphyConfig0Spec>;
#[doc = "Field `dl0_enable` reader - Data lane 0 enable"]
pub type Dl0EnableR = crate::BitReader;
#[doc = "Field `dl0_enable` writer - Data lane 0 enable"]
pub type Dl0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl1_enable` reader - Data lane 1 enable"]
pub type Dl1EnableR = crate::BitReader;
#[doc = "Field `dl1_enable` writer - Data lane 1 enable"]
pub type Dl1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cl_enable` reader - Clock lane enable"]
pub type ClEnableR = crate::BitReader;
#[doc = "Field `cl_enable` writer - Clock lane enable"]
pub type ClEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl0_stopstate` reader - Data lane 0 stop state status"]
pub type Dl0StopstateR = crate::BitReader;
#[doc = "Field `dl1_stopstate` reader - Data lane 1 stop state status"]
pub type Dl1StopstateR = crate::BitReader;
#[doc = "Field `cl_stopstate` reader - Clock lane stop state status"]
pub type ClStopstateR = crate::BitReader;
#[doc = "Field `dl0_ulpsactivenot` reader - Data lane 0 ULPS active status"]
pub type Dl0UlpsactivenotR = crate::BitReader;
#[doc = "Field `dl1_ulpsactivenot` reader - Data lane 1 ULPS active status"]
pub type Dl1UlpsactivenotR = crate::BitReader;
#[doc = "Field `cl_ulpsactivenot` reader - Clock lane ULPS active status"]
pub type ClUlpsactivenotR = crate::BitReader;
#[doc = "Field `dl0_forcerxmode` reader - Force Data Lane 0 into RX mode (LP mode)"]
pub type Dl0ForcerxmodeR = crate::BitReader;
#[doc = "Field `dl0_forcerxmode` writer - Force Data Lane 0 into RX mode (LP mode)"]
pub type Dl0ForcerxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl1_forcerxmode` reader - Force Data Lane 1 into RX mode (LP mode)"]
pub type Dl1ForcerxmodeR = crate::BitReader;
#[doc = "Field `dl1_forcerxmode` writer - Force Data Lane 1 into RX mode (LP mode)"]
pub type Dl1ForcerxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cl_rxclkactivehs` reader - Clock lane High Speed active status"]
pub type ClRxclkactivehsR = crate::BitReader;
#[doc = "Field `cl_rxulpsclknot` reader - Clock lane ULPS clock status"]
pub type ClRxulpsclknotR = crate::BitReader;
#[doc = "Field `csi_reset_n` reader - CSI reset control (Active Low)"]
pub type CsiResetNR = crate::BitReader;
#[doc = "Field `csi_reset_n` writer - CSI reset control (Active Low)"]
pub type CsiResetNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data lane 0 enable"]
    #[inline(always)]
    pub fn dl0_enable(&self) -> Dl0EnableR {
        Dl0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data lane 1 enable"]
    #[inline(always)]
    pub fn dl1_enable(&self) -> Dl1EnableR {
        Dl1EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock lane enable"]
    #[inline(always)]
    pub fn cl_enable(&self) -> ClEnableR {
        ClEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Data lane 0 stop state status"]
    #[inline(always)]
    pub fn dl0_stopstate(&self) -> Dl0StopstateR {
        Dl0StopstateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data lane 1 stop state status"]
    #[inline(always)]
    pub fn dl1_stopstate(&self) -> Dl1StopstateR {
        Dl1StopstateR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock lane stop state status"]
    #[inline(always)]
    pub fn cl_stopstate(&self) -> ClStopstateR {
        ClStopstateR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Data lane 0 ULPS active status"]
    #[inline(always)]
    pub fn dl0_ulpsactivenot(&self) -> Dl0UlpsactivenotR {
        Dl0UlpsactivenotR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data lane 1 ULPS active status"]
    #[inline(always)]
    pub fn dl1_ulpsactivenot(&self) -> Dl1UlpsactivenotR {
        Dl1UlpsactivenotR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock lane ULPS active status"]
    #[inline(always)]
    pub fn cl_ulpsactivenot(&self) -> ClUlpsactivenotR {
        ClUlpsactivenotR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Force Data Lane 0 into RX mode (LP mode)"]
    #[inline(always)]
    pub fn dl0_forcerxmode(&self) -> Dl0ForcerxmodeR {
        Dl0ForcerxmodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Data Lane 1 into RX mode (LP mode)"]
    #[inline(always)]
    pub fn dl1_forcerxmode(&self) -> Dl1ForcerxmodeR {
        Dl1ForcerxmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clock lane High Speed active status"]
    #[inline(always)]
    pub fn cl_rxclkactivehs(&self) -> ClRxclkactivehsR {
        ClRxclkactivehsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock lane ULPS clock status"]
    #[inline(always)]
    pub fn cl_rxulpsclknot(&self) -> ClRxulpsclknotR {
        ClRxulpsclknotR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - CSI reset control (Active Low)"]
    #[inline(always)]
    pub fn csi_reset_n(&self) -> CsiResetNR {
        CsiResetNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data lane 0 enable"]
    #[inline(always)]
    pub fn dl0_enable(&mut self) -> Dl0EnableW<'_, DphyConfig0Spec> {
        Dl0EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Data lane 1 enable"]
    #[inline(always)]
    pub fn dl1_enable(&mut self) -> Dl1EnableW<'_, DphyConfig0Spec> {
        Dl1EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock lane enable"]
    #[inline(always)]
    pub fn cl_enable(&mut self) -> ClEnableW<'_, DphyConfig0Spec> {
        ClEnableW::new(self, 2)
    }
    #[doc = "Bit 12 - Force Data Lane 0 into RX mode (LP mode)"]
    #[inline(always)]
    pub fn dl0_forcerxmode(&mut self) -> Dl0ForcerxmodeW<'_, DphyConfig0Spec> {
        Dl0ForcerxmodeW::new(self, 12)
    }
    #[doc = "Bit 13 - Force Data Lane 1 into RX mode (LP mode)"]
    #[inline(always)]
    pub fn dl1_forcerxmode(&mut self) -> Dl1ForcerxmodeW<'_, DphyConfig0Spec> {
        Dl1ForcerxmodeW::new(self, 13)
    }
    #[doc = "Bit 31 - CSI reset control (Active Low)"]
    #[inline(always)]
    pub fn csi_reset_n(&mut self) -> CsiResetNW<'_, DphyConfig0Spec> {
        CsiResetNW::new(self, 31)
    }
}
#[doc = "D-PHY Configuration 0 (Lane/Mode Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig0Spec;
impl crate::RegisterSpec for DphyConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_0::R`](R) reader structure"]
impl crate::Readable for DphyConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_0::W`](W) writer structure"]
impl crate::Writable for DphyConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_0 to value 0x7070"]
impl crate::Resettable for DphyConfig0Spec {
    const RESET_VALUE: u32 = 0x7070;
}
