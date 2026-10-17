#[doc = "Register `MIPI_PLL_CFG4` reader"]
pub type R = crate::R<MipiPllCfg4Spec>;
#[doc = "Register `MIPI_PLL_CFG4` writer"]
pub type W = crate::W<MipiPllCfg4Spec>;
#[doc = "Field `mipipll_sel_sample_clk` reader - "]
pub type MipipllSelSampleClkR = crate::FieldReader;
#[doc = "Field `mipipll_sel_sample_clk` writer - "]
pub type MipipllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_sel_fb_clk` reader - "]
pub type MipipllSelFbClkR = crate::FieldReader;
#[doc = "Field `mipipll_sel_fb_clk` writer - "]
pub type MipipllSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_lock_det_en` reader - "]
pub type MipipllLockDetEnR = crate::BitReader;
#[doc = "Field `mipipll_lock_det_en` writer - "]
pub type MipipllLockDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_lock_clk_sel` reader - "]
pub type MipipllLockClkSelR = crate::FieldReader;
#[doc = "Field `mipipll_lock_clk_sel` writer - "]
pub type MipipllLockClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_lock_clk_inv_en` reader - "]
pub type MipipllLockClkInvEnR = crate::BitReader;
#[doc = "Field `mipipll_lock_clk_inv_en` writer - "]
pub type MipipllLockClkInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_lock_win_sel` reader - "]
pub type MipipllLockWinSelR = crate::FieldReader;
#[doc = "Field `mipipll_lock_win_sel` writer - "]
pub type MipipllLockWinSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mipipll_sel_sample_clk(&self) -> MipipllSelSampleClkR {
        MipipllSelSampleClkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mipipll_sel_fb_clk(&self) -> MipipllSelFbClkR {
        MipipllSelFbClkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_lock_det_en(&self) -> MipipllLockDetEnR {
        MipipllLockDetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn mipipll_lock_clk_sel(&self) -> MipipllLockClkSelR {
        MipipllLockClkSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mipipll_lock_clk_inv_en(&self) -> MipipllLockClkInvEnR {
        MipipllLockClkInvEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn mipipll_lock_win_sel(&self) -> MipipllLockWinSelR {
        MipipllLockWinSelR::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mipipll_sel_sample_clk(&mut self) -> MipipllSelSampleClkW<'_, MipiPllCfg4Spec> {
        MipipllSelSampleClkW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mipipll_sel_fb_clk(&mut self) -> MipipllSelFbClkW<'_, MipiPllCfg4Spec> {
        MipipllSelFbClkW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_lock_det_en(&mut self) -> MipipllLockDetEnW<'_, MipiPllCfg4Spec> {
        MipipllLockDetEnW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn mipipll_lock_clk_sel(&mut self) -> MipipllLockClkSelW<'_, MipiPllCfg4Spec> {
        MipipllLockClkSelW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mipipll_lock_clk_inv_en(&mut self) -> MipipllLockClkInvEnW<'_, MipiPllCfg4Spec> {
        MipipllLockClkInvEnW::new(self, 12)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn mipipll_lock_win_sel(&mut self) -> MipipllLockWinSelW<'_, MipiPllCfg4Spec> {
        MipipllLockWinSelW::new(self, 15)
    }
}
#[doc = "MIPI PLL Configuration 4 - Lock Detector\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg4Spec;
impl crate::RegisterSpec for MipiPllCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg4::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg4::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG4 to value 0"]
impl crate::Resettable for MipiPllCfg4Spec {}
