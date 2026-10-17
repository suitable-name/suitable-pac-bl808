#[doc = "Register `DIG_CLK_CFG3` reader"]
pub type R = crate::R<DigClkCfg3Spec>;
#[doc = "Register `DIG_CLK_CFG3` writer"]
pub type W = crate::W<DigClkCfg3Spec>;
#[doc = "Field `dsi_txclkesc_sel` reader - "]
pub type DsiTxclkescSelR = crate::BitReader;
#[doc = "Field `dsi_txclkesc_sel` writer - "]
pub type DsiTxclkescSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csi_txclkesc_sel` reader - "]
pub type CsiTxclkescSelR = crate::BitReader;
#[doc = "Field `csi_txclkesc_sel` writer - "]
pub type CsiTxclkescSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dsi_txclkesc_sel(&self) -> DsiTxclkescSelR {
        DsiTxclkescSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn csi_txclkesc_sel(&self) -> CsiTxclkescSelR {
        CsiTxclkescSelR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dsi_txclkesc_sel(&mut self) -> DsiTxclkescSelW<'_, DigClkCfg3Spec> {
        DsiTxclkescSelW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn csi_txclkesc_sel(&mut self) -> CsiTxclkescSelW<'_, DigClkCfg3Spec> {
        CsiTxclkescSelW::new(self, 1)
    }
}
#[doc = "Digital Clock Config 3 (DSI/CSI Escape Clocks)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DigClkCfg3Spec;
impl crate::RegisterSpec for DigClkCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_clk_cfg3::R`](R) reader structure"]
impl crate::Readable for DigClkCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`dig_clk_cfg3::W`](W) writer structure"]
impl crate::Writable for DigClkCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIG_CLK_CFG3 to value 0"]
impl crate::Resettable for DigClkCfg3Spec {}
