#[doc = "Register `EMI_CFG0` reader"]
pub type R = crate::R<EmiCfg0Spec>;
#[doc = "Register `EMI_CFG0` writer"]
pub type W = crate::W<EmiCfg0Spec>;
#[doc = "Field `reg_emi_clk_en` reader - "]
pub type RegEmiClkEnR = crate::BitReader;
#[doc = "Field `reg_emi_clk_en` writer - "]
pub type RegEmiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_emi_clk_sel` reader - "]
pub type RegEmiClkSelR = crate::FieldReader;
#[doc = "Field `reg_emi_clk_sel` writer - "]
pub type RegEmiClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_emi_clk_div` reader - "]
pub type RegEmiClkDivR = crate::FieldReader;
#[doc = "Field `reg_emi_clk_div` writer - "]
pub type RegEmiClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_emi_clk_en(&self) -> RegEmiClkEnR {
        RegEmiClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn reg_emi_clk_sel(&self) -> RegEmiClkSelR {
        RegEmiClkSelR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reg_emi_clk_div(&self) -> RegEmiClkDivR {
        RegEmiClkDivR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_emi_clk_en(&mut self) -> RegEmiClkEnW<'_, EmiCfg0Spec> {
        RegEmiClkEnW::new(self, 9)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn reg_emi_clk_sel(&mut self) -> RegEmiClkSelW<'_, EmiCfg0Spec> {
        RegEmiClkSelW::new(self, 14)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reg_emi_clk_div(&mut self) -> RegEmiClkDivW<'_, EmiCfg0Spec> {
        RegEmiClkDivW::new(self, 22)
    }
}
#[doc = "EMI Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmiCfg0Spec;
impl crate::RegisterSpec for EmiCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emi_cfg0::R`](R) reader structure"]
impl crate::Readable for EmiCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`emi_cfg0::W`](W) writer structure"]
impl crate::Writable for EmiCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMI_CFG0 to value 0"]
impl crate::Resettable for EmiCfg0Spec {}
