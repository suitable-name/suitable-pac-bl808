#[doc = "Register `DPHY_CONFIG_0` reader"]
pub type R = crate::R<DphyConfig0Spec>;
#[doc = "Register `DPHY_CONFIG_0` writer"]
pub type W = crate::W<DphyConfig0Spec>;
#[doc = "Field `cl_enable` reader - "]
pub type ClEnableR = crate::BitReader;
#[doc = "Field `cl_enable` writer - "]
pub type ClEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cl_txrequesths` reader - "]
pub type ClTxrequesthsR = crate::BitReader;
#[doc = "Field `cl_txrequesths` writer - "]
pub type ClTxrequesthsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cl_txulpsclk` reader - "]
pub type ClTxulpsclkR = crate::BitReader;
#[doc = "Field `cl_txulpsclk` writer - "]
pub type ClTxulpsclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cl_txulpsexit` writer - "]
pub type ClTxulpsexitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cl_stopstate` reader - "]
pub type ClStopstateR = crate::BitReader;
#[doc = "Field `cl_ulpsactivenot` reader - "]
pub type ClUlpsactivenotR = crate::BitReader;
#[doc = "Field `dl0_enable` reader - "]
pub type Dl0EnableR = crate::BitReader;
#[doc = "Field `dl0_enable` writer - "]
pub type Dl0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl1_enable` reader - "]
pub type Dl1EnableR = crate::BitReader;
#[doc = "Field `dl1_enable` writer - "]
pub type Dl1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl2_enable` reader - "]
pub type Dl2EnableR = crate::BitReader;
#[doc = "Field `dl2_enable` writer - "]
pub type Dl2EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl3_enable` reader - "]
pub type Dl3EnableR = crate::BitReader;
#[doc = "Field `dl3_enable` writer - "]
pub type Dl3EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl0_stopstate` reader - "]
pub type Dl0StopstateR = crate::BitReader;
#[doc = "Field `dl1_stopstate` reader - "]
pub type Dl1StopstateR = crate::BitReader;
#[doc = "Field `dl2_stopstate` reader - "]
pub type Dl2StopstateR = crate::BitReader;
#[doc = "Field `dl3_stopstate` reader - "]
pub type Dl3StopstateR = crate::BitReader;
#[doc = "Field `dl0_ulpsactivenot` reader - "]
pub type Dl0UlpsactivenotR = crate::BitReader;
#[doc = "Field `dl1_ulpsactivenot` reader - "]
pub type Dl1UlpsactivenotR = crate::BitReader;
#[doc = "Field `dl2_ulpsactivenot` reader - "]
pub type Dl2UlpsactivenotR = crate::BitReader;
#[doc = "Field `dl3_ulpsactivenot` reader - "]
pub type Dl3UlpsactivenotR = crate::BitReader;
#[doc = "Field `dl0_forcerxmode` reader - "]
pub type Dl0ForcerxmodeR = crate::BitReader;
#[doc = "Field `dl0_forcerxmode` writer - "]
pub type Dl0ForcerxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl1_forcerxmode` reader - "]
pub type Dl1ForcerxmodeR = crate::BitReader;
#[doc = "Field `dl1_forcerxmode` writer - "]
pub type Dl1ForcerxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl2_forcerxmode` reader - "]
pub type Dl2ForcerxmodeR = crate::BitReader;
#[doc = "Field `dl2_forcerxmode` writer - "]
pub type Dl2ForcerxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl3_forcerxmode` reader - "]
pub type Dl3ForcerxmodeR = crate::BitReader;
#[doc = "Field `dl3_forcerxmode` writer - "]
pub type Dl3ForcerxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl0_forcetxstopmode` reader - "]
pub type Dl0ForcetxstopmodeR = crate::BitReader;
#[doc = "Field `dl0_forcetxstopmode` writer - "]
pub type Dl0ForcetxstopmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl1_forcetxstopmode` reader - "]
pub type Dl1ForcetxstopmodeR = crate::BitReader;
#[doc = "Field `dl1_forcetxstopmode` writer - "]
pub type Dl1ForcetxstopmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl2_forcetxstopmode` reader - "]
pub type Dl2ForcetxstopmodeR = crate::BitReader;
#[doc = "Field `dl2_forcetxstopmode` writer - "]
pub type Dl2ForcetxstopmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl3_forcetxstopmode` reader - "]
pub type Dl3ForcetxstopmodeR = crate::BitReader;
#[doc = "Field `dl3_forcetxstopmode` writer - "]
pub type Dl3ForcetxstopmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl0_turnesc` writer - "]
pub type Dl0TurnescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dsi_reset_n` reader - "]
pub type DsiResetNR = crate::BitReader;
#[doc = "Field `dsi_reset_n` writer - "]
pub type DsiResetNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cl_enable(&self) -> ClEnableR {
        ClEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cl_txrequesths(&self) -> ClTxrequesthsR {
        ClTxrequesthsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cl_txulpsclk(&self) -> ClTxulpsclkR {
        ClTxulpsclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cl_stopstate(&self) -> ClStopstateR {
        ClStopstateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cl_ulpsactivenot(&self) -> ClUlpsactivenotR {
        ClUlpsactivenotR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dl0_enable(&self) -> Dl0EnableR {
        Dl0EnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dl1_enable(&self) -> Dl1EnableR {
        Dl1EnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dl2_enable(&self) -> Dl2EnableR {
        Dl2EnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dl3_enable(&self) -> Dl3EnableR {
        Dl3EnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dl0_stopstate(&self) -> Dl0StopstateR {
        Dl0StopstateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dl1_stopstate(&self) -> Dl1StopstateR {
        Dl1StopstateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dl2_stopstate(&self) -> Dl2StopstateR {
        Dl2StopstateR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dl3_stopstate(&self) -> Dl3StopstateR {
        Dl3StopstateR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dl0_ulpsactivenot(&self) -> Dl0UlpsactivenotR {
        Dl0UlpsactivenotR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dl1_ulpsactivenot(&self) -> Dl1UlpsactivenotR {
        Dl1UlpsactivenotR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dl2_ulpsactivenot(&self) -> Dl2UlpsactivenotR {
        Dl2UlpsactivenotR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dl3_ulpsactivenot(&self) -> Dl3UlpsactivenotR {
        Dl3UlpsactivenotR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dl0_forcerxmode(&self) -> Dl0ForcerxmodeR {
        Dl0ForcerxmodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dl1_forcerxmode(&self) -> Dl1ForcerxmodeR {
        Dl1ForcerxmodeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dl2_forcerxmode(&self) -> Dl2ForcerxmodeR {
        Dl2ForcerxmodeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dl3_forcerxmode(&self) -> Dl3ForcerxmodeR {
        Dl3ForcerxmodeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dl0_forcetxstopmode(&self) -> Dl0ForcetxstopmodeR {
        Dl0ForcetxstopmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dl1_forcetxstopmode(&self) -> Dl1ForcetxstopmodeR {
        Dl1ForcetxstopmodeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dl2_forcetxstopmode(&self) -> Dl2ForcetxstopmodeR {
        Dl2ForcetxstopmodeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dl3_forcetxstopmode(&self) -> Dl3ForcetxstopmodeR {
        Dl3ForcetxstopmodeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dsi_reset_n(&self) -> DsiResetNR {
        DsiResetNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cl_enable(&mut self) -> ClEnableW<'_, DphyConfig0Spec> {
        ClEnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cl_txrequesths(&mut self) -> ClTxrequesthsW<'_, DphyConfig0Spec> {
        ClTxrequesthsW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cl_txulpsclk(&mut self) -> ClTxulpsclkW<'_, DphyConfig0Spec> {
        ClTxulpsclkW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cl_txulpsexit(&mut self) -> ClTxulpsexitW<'_, DphyConfig0Spec> {
        ClTxulpsexitW::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dl0_enable(&mut self) -> Dl0EnableW<'_, DphyConfig0Spec> {
        Dl0EnableW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dl1_enable(&mut self) -> Dl1EnableW<'_, DphyConfig0Spec> {
        Dl1EnableW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dl2_enable(&mut self) -> Dl2EnableW<'_, DphyConfig0Spec> {
        Dl2EnableW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dl3_enable(&mut self) -> Dl3EnableW<'_, DphyConfig0Spec> {
        Dl3EnableW::new(self, 11)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dl0_forcerxmode(&mut self) -> Dl0ForcerxmodeW<'_, DphyConfig0Spec> {
        Dl0ForcerxmodeW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dl1_forcerxmode(&mut self) -> Dl1ForcerxmodeW<'_, DphyConfig0Spec> {
        Dl1ForcerxmodeW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dl2_forcerxmode(&mut self) -> Dl2ForcerxmodeW<'_, DphyConfig0Spec> {
        Dl2ForcerxmodeW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dl3_forcerxmode(&mut self) -> Dl3ForcerxmodeW<'_, DphyConfig0Spec> {
        Dl3ForcerxmodeW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dl0_forcetxstopmode(&mut self) -> Dl0ForcetxstopmodeW<'_, DphyConfig0Spec> {
        Dl0ForcetxstopmodeW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dl1_forcetxstopmode(&mut self) -> Dl1ForcetxstopmodeW<'_, DphyConfig0Spec> {
        Dl1ForcetxstopmodeW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dl2_forcetxstopmode(&mut self) -> Dl2ForcetxstopmodeW<'_, DphyConfig0Spec> {
        Dl2ForcetxstopmodeW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dl3_forcetxstopmode(&mut self) -> Dl3ForcetxstopmodeW<'_, DphyConfig0Spec> {
        Dl3ForcetxstopmodeW::new(self, 27)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dl0_turnesc(&mut self) -> Dl0TurnescW<'_, DphyConfig0Spec> {
        Dl0TurnescW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dsi_reset_n(&mut self) -> DsiResetNW<'_, DphyConfig0Spec> {
        DsiResetNW::new(self, 31)
    }
}
#[doc = "DPHY Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DPHY_CONFIG_0 to value 0x000f_f030"]
impl crate::Resettable for DphyConfig0Spec {
    const RESET_VALUE: u32 = 0x000f_f030;
}
