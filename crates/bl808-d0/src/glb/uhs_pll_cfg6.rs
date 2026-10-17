#[doc = "Register `UHS_PLL_CFG6` reader"]
pub type R = crate::R<UhsPllCfg6Spec>;
#[doc = "Register `UHS_PLL_CFG6` writer"]
pub type W = crate::W<UhsPllCfg6Spec>;
#[doc = "Field `uhspll_sdmin` reader - "]
pub type UhspllSdminR = crate::FieldReader<u32>;
#[doc = "Field `uhspll_sdmin` writer - "]
pub type UhspllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `uhspll_sdm_bypass` reader - "]
pub type UhspllSdmBypassR = crate::BitReader;
#[doc = "Field `uhspll_sdm_bypass` writer - "]
pub type UhspllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn uhspll_sdmin(&self) -> UhspllSdminR {
        UhspllSdminR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uhspll_sdm_bypass(&self) -> UhspllSdmBypassR {
        UhspllSdmBypassR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn uhspll_sdmin(&mut self) -> UhspllSdminW<'_, UhsPllCfg6Spec> {
        UhspllSdminW::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uhspll_sdm_bypass(&mut self) -> UhspllSdmBypassW<'_, UhsPllCfg6Spec> {
        UhspllSdmBypassW::new(self, 24)
    }
}
#[doc = "UHS PLL Configuration 6 - SDM\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg6Spec;
impl crate::RegisterSpec for UhsPllCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg6::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg6::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG6 to value 0"]
impl crate::Resettable for UhsPllCfg6Spec {}
