#[doc = "Register `MIPI_PLL_CFG6` reader"]
pub type R = crate::R<MipiPllCfg6Spec>;
#[doc = "Register `MIPI_PLL_CFG6` writer"]
pub type W = crate::W<MipiPllCfg6Spec>;
#[doc = "Field `mipipll_sdmin` reader - "]
pub type MipipllSdminR = crate::FieldReader<u32>;
#[doc = "Field `mipipll_sdmin` writer - "]
pub type MipipllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `mipipll_sdm_bypass` reader - "]
pub type MipipllSdmBypassR = crate::BitReader;
#[doc = "Field `mipipll_sdm_bypass` writer - "]
pub type MipipllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn mipipll_sdmin(&self) -> MipipllSdminR {
        MipipllSdminR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn mipipll_sdm_bypass(&self) -> MipipllSdmBypassR {
        MipipllSdmBypassR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn mipipll_sdmin(&mut self) -> MipipllSdminW<'_, MipiPllCfg6Spec> {
        MipipllSdminW::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn mipipll_sdm_bypass(&mut self) -> MipipllSdmBypassW<'_, MipiPllCfg6Spec> {
        MipipllSdmBypassW::new(self, 24)
    }
}
#[doc = "MIPI PLL Configuration 6 - SDM\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg6Spec;
impl crate::RegisterSpec for MipiPllCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg6::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg6::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG6 to value 0"]
impl crate::Resettable for MipiPllCfg6Spec {}
