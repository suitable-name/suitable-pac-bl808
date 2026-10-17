#[doc = "Register `MIPI_PLL_CFG7` reader"]
pub type R = crate::R<MipiPllCfg7Spec>;
#[doc = "Register `MIPI_PLL_CFG7` writer"]
pub type W = crate::W<MipiPllCfg7Spec>;
#[doc = "Field `mipipll_sdm_order_sel` reader - "]
pub type MipipllSdmOrderSelR = crate::BitReader;
#[doc = "Field `mipipll_sdm_order_sel` writer - "]
pub type MipipllSdmOrderSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_sdm_dith_sel` reader - "]
pub type MipipllSdmDithSelR = crate::FieldReader;
#[doc = "Field `mipipll_sdm_dith_sel` writer - "]
pub type MipipllSdmDithSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sdm_order_sel(&self) -> MipipllSdmOrderSelR {
        MipipllSdmOrderSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mipipll_sdm_dith_sel(&self) -> MipipllSdmDithSelR {
        MipipllSdmDithSelR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sdm_order_sel(&mut self) -> MipipllSdmOrderSelW<'_, MipiPllCfg7Spec> {
        MipipllSdmOrderSelW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn mipipll_sdm_dith_sel(&mut self) -> MipipllSdmDithSelW<'_, MipiPllCfg7Spec> {
        MipipllSdmDithSelW::new(self, 1)
    }
}
#[doc = "MIPI PLL Configuration 7 - SDM Dither\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg7Spec;
impl crate::RegisterSpec for MipiPllCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg7::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg7::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG7 to value 0"]
impl crate::Resettable for MipiPllCfg7Spec {}
