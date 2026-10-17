#[doc = "Register `PSRAM_CFG0` reader"]
pub type R = crate::R<PsramCfg0Spec>;
#[doc = "Register `PSRAM_CFG0` writer"]
pub type W = crate::W<PsramCfg0Spec>;
#[doc = "Field `reg_psramb_clk_en` reader - "]
pub type RegPsrambClkEnR = crate::BitReader;
#[doc = "Field `reg_psramb_clk_en` writer - "]
pub type RegPsrambClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psramb_clk_sel` reader - "]
pub type RegPsrambClkSelR = crate::FieldReader;
#[doc = "Field `reg_psramb_clk_sel` writer - "]
pub type RegPsrambClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_psramb_clk_div` reader - "]
pub type RegPsrambClkDivR = crate::FieldReader;
#[doc = "Field `reg_psramb_clk_div` writer - "]
pub type RegPsrambClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_psramb_clk_en(&self) -> RegPsrambClkEnR {
        RegPsrambClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_psramb_clk_sel(&self) -> RegPsrambClkSelR {
        RegPsrambClkSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_psramb_clk_div(&self) -> RegPsrambClkDivR {
        RegPsrambClkDivR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_psramb_clk_en(&mut self) -> RegPsrambClkEnW<'_, PsramCfg0Spec> {
        RegPsrambClkEnW::new(self, 27)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_psramb_clk_sel(&mut self) -> RegPsrambClkSelW<'_, PsramCfg0Spec> {
        RegPsrambClkSelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_psramb_clk_div(&mut self) -> RegPsrambClkDivW<'_, PsramCfg0Spec> {
        RegPsrambClkDivW::new(self, 30)
    }
}
#[doc = "PSRAM Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramCfg0Spec;
impl crate::RegisterSpec for PsramCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_cfg0::R`](R) reader structure"]
impl crate::Readable for PsramCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_cfg0::W`](W) writer structure"]
impl crate::Writable for PsramCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_CFG0 to value 0"]
impl crate::Resettable for PsramCfg0Spec {}
