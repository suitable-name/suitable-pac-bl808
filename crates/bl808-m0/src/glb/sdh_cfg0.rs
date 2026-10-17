#[doc = "Register `SDH_CFG0` reader"]
pub type R = crate::R<SdhCfg0Spec>;
#[doc = "Register `SDH_CFG0` writer"]
pub type W = crate::W<SdhCfg0Spec>;
#[doc = "Field `reg_sdh_clk_div` reader - "]
pub type RegSdhClkDivR = crate::FieldReader;
#[doc = "Field `reg_sdh_clk_div` writer - "]
pub type RegSdhClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_sdh_clk_sel` reader - "]
pub type RegSdhClkSelR = crate::BitReader;
#[doc = "Field `reg_sdh_clk_sel` writer - "]
pub type RegSdhClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sdh_clk_en` reader - "]
pub type RegSdhClkEnR = crate::BitReader;
#[doc = "Field `reg_sdh_clk_en` writer - "]
pub type RegSdhClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reg_sdh_clk_div(&self) -> RegSdhClkDivR {
        RegSdhClkDivR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_sdh_clk_sel(&self) -> RegSdhClkSelR {
        RegSdhClkSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_sdh_clk_en(&self) -> RegSdhClkEnR {
        RegSdhClkEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reg_sdh_clk_div(&mut self) -> RegSdhClkDivW<'_, SdhCfg0Spec> {
        RegSdhClkDivW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_sdh_clk_sel(&mut self) -> RegSdhClkSelW<'_, SdhCfg0Spec> {
        RegSdhClkSelW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_sdh_clk_en(&mut self) -> RegSdhClkEnW<'_, SdhCfg0Spec> {
        RegSdhClkEnW::new(self, 13)
    }
}
#[doc = "SDH Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sdh_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdh_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhCfg0Spec;
impl crate::RegisterSpec for SdhCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdh_cfg0::R`](R) reader structure"]
impl crate::Readable for SdhCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sdh_cfg0::W`](W) writer structure"]
impl crate::Writable for SdhCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDH_CFG0 to value 0"]
impl crate::Resettable for SdhCfg0Spec {}
