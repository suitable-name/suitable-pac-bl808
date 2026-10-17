#[doc = "Register `SYS_CFG0` reader"]
pub type R = crate::R<SysCfg0Spec>;
#[doc = "Register `SYS_CFG0` writer"]
pub type W = crate::W<SysCfg0Spec>;
#[doc = "Field `reg_pll_en` reader - "]
pub type RegPllEnR = crate::BitReader;
#[doc = "Field `reg_pll_en` writer - "]
pub type RegPllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_fclk_en` reader - "]
pub type RegFclkEnR = crate::BitReader;
#[doc = "Field `reg_fclk_en` writer - "]
pub type RegFclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_hclk_en` reader - "]
pub type RegHclkEnR = crate::BitReader;
#[doc = "Field `reg_hclk_en` writer - "]
pub type RegHclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk_en` reader - "]
pub type RegBclkEnR = crate::BitReader;
#[doc = "Field `reg_bclk_en` writer - "]
pub type RegBclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub type HbnRootClkSelR = crate::FieldReader;
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub type HbnRootClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_hclk_div` reader - "]
pub type RegHclkDivR = crate::FieldReader;
#[doc = "Field `reg_hclk_div` writer - "]
pub type RegHclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_bclk_div` reader - "]
pub type RegBclkDivR = crate::FieldReader;
#[doc = "Field `reg_bclk_div` writer - "]
pub type RegBclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pll_en(&self) -> RegPllEnR {
        RegPllEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&self) -> RegFclkEnR {
        RegFclkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_hclk_en(&self) -> RegHclkEnR {
        RegHclkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_bclk_en(&self) -> RegBclkEnR {
        RegBclkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HbnRootClkSelR {
        HbnRootClkSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_hclk_div(&self) -> RegHclkDivR {
        RegHclkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_bclk_div(&self) -> RegBclkDivR {
        RegBclkDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pll_en(&mut self) -> RegPllEnW<'_, SysCfg0Spec> {
        RegPllEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&mut self) -> RegFclkEnW<'_, SysCfg0Spec> {
        RegFclkEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_hclk_en(&mut self) -> RegHclkEnW<'_, SysCfg0Spec> {
        RegHclkEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_bclk_en(&mut self) -> RegBclkEnW<'_, SysCfg0Spec> {
        RegBclkEnW::new(self, 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&mut self) -> HbnRootClkSelW<'_, SysCfg0Spec> {
        HbnRootClkSelW::new(self, 6)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_hclk_div(&mut self) -> RegHclkDivW<'_, SysCfg0Spec> {
        RegHclkDivW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_bclk_div(&mut self) -> RegBclkDivW<'_, SysCfg0Spec> {
        RegBclkDivW::new(self, 16)
    }
}
#[doc = "System Clock Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCfg0Spec;
impl crate::RegisterSpec for SysCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_cfg0::R`](R) reader structure"]
impl crate::Readable for SysCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_cfg0::W`](W) writer structure"]
impl crate::Writable for SysCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CFG0 to value 0"]
impl crate::Resettable for SysCfg0Spec {}
