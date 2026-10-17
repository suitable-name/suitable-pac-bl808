#[doc = "Register `UHS_PLL_CFG4` reader"]
pub type R = crate::R<UhsPllCfg4Spec>;
#[doc = "Register `UHS_PLL_CFG4` writer"]
pub type W = crate::W<UhsPllCfg4Spec>;
#[doc = "Field `uhspll_sel_sample_clk` reader - "]
pub type UhspllSelSampleClkR = crate::FieldReader;
#[doc = "Field `uhspll_sel_sample_clk` writer - "]
pub type UhspllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_sel_fb_clk` reader - "]
pub type UhspllSelFbClkR = crate::FieldReader;
#[doc = "Field `uhspll_sel_fb_clk` writer - "]
pub type UhspllSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_lock_det_en` reader - "]
pub type UhspllLockDetEnR = crate::BitReader;
#[doc = "Field `uhspll_lock_det_en` writer - "]
pub type UhspllLockDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_lock_clk_sel` reader - "]
pub type UhspllLockClkSelR = crate::FieldReader;
#[doc = "Field `uhspll_lock_clk_sel` writer - "]
pub type UhspllLockClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_lock_clk_inv_en` reader - "]
pub type UhspllLockClkInvEnR = crate::BitReader;
#[doc = "Field `uhspll_lock_clk_inv_en` writer - "]
pub type UhspllLockClkInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_lock_win_sel` reader - "]
pub type UhspllLockWinSelR = crate::FieldReader;
#[doc = "Field `uhspll_lock_win_sel` writer - "]
pub type UhspllLockWinSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn uhspll_sel_sample_clk(&self) -> UhspllSelSampleClkR {
        UhspllSelSampleClkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_sel_fb_clk(&self) -> UhspllSelFbClkR {
        UhspllSelFbClkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_lock_det_en(&self) -> UhspllLockDetEnR {
        UhspllLockDetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn uhspll_lock_clk_sel(&self) -> UhspllLockClkSelR {
        UhspllLockClkSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhspll_lock_clk_inv_en(&self) -> UhspllLockClkInvEnR {
        UhspllLockClkInvEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn uhspll_lock_win_sel(&self) -> UhspllLockWinSelR {
        UhspllLockWinSelR::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn uhspll_sel_sample_clk(&mut self) -> UhspllSelSampleClkW<'_, UhsPllCfg4Spec> {
        UhspllSelSampleClkW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_sel_fb_clk(&mut self) -> UhspllSelFbClkW<'_, UhsPllCfg4Spec> {
        UhspllSelFbClkW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_lock_det_en(&mut self) -> UhspllLockDetEnW<'_, UhsPllCfg4Spec> {
        UhspllLockDetEnW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn uhspll_lock_clk_sel(&mut self) -> UhspllLockClkSelW<'_, UhsPllCfg4Spec> {
        UhspllLockClkSelW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhspll_lock_clk_inv_en(&mut self) -> UhspllLockClkInvEnW<'_, UhsPllCfg4Spec> {
        UhspllLockClkInvEnW::new(self, 12)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn uhspll_lock_win_sel(&mut self) -> UhspllLockWinSelW<'_, UhsPllCfg4Spec> {
        UhspllLockWinSelW::new(self, 15)
    }
}
#[doc = "UHS PLL Configuration 4 - Lock Detector\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg4Spec;
impl crate::RegisterSpec for UhsPllCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg4::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg4::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG4 to value 0"]
impl crate::Resettable for UhsPllCfg4Spec {}
