#[doc = "Register `UHS_PLL_CFG7` reader"]
pub type R = crate::R<UhsPllCfg7Spec>;
#[doc = "Register `UHS_PLL_CFG7` writer"]
pub type W = crate::W<UhsPllCfg7Spec>;
#[doc = "Field `uhspll_sdm_order_sel` reader - "]
pub type UhspllSdmOrderSelR = crate::BitReader;
#[doc = "Field `uhspll_sdm_order_sel` writer - "]
pub type UhspllSdmOrderSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_sdm_dith_sel` reader - "]
pub type UhspllSdmDithSelR = crate::FieldReader;
#[doc = "Field `uhspll_sdm_dith_sel` writer - "]
pub type UhspllSdmDithSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_sdm_order_sel(&self) -> UhspllSdmOrderSelR {
        UhspllSdmOrderSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn uhspll_sdm_dith_sel(&self) -> UhspllSdmDithSelR {
        UhspllSdmDithSelR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_sdm_order_sel(&mut self) -> UhspllSdmOrderSelW<'_, UhsPllCfg7Spec> {
        UhspllSdmOrderSelW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn uhspll_sdm_dith_sel(&mut self) -> UhspllSdmDithSelW<'_, UhsPllCfg7Spec> {
        UhspllSdmDithSelW::new(self, 1)
    }
}
#[doc = "UHS PLL Configuration 7 - SDM Dither\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg7Spec;
impl crate::RegisterSpec for UhsPllCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg7::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg7::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG7 to value 0"]
impl crate::Resettable for UhsPllCfg7Spec {}
