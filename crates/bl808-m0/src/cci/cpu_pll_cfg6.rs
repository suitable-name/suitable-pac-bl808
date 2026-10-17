#[doc = "Register `CPU_PLL_CFG6` reader"]
pub type R = crate::R<CpuPllCfg6Spec>;
#[doc = "Register `CPU_PLL_CFG6` writer"]
pub type W = crate::W<CpuPllCfg6Spec>;
#[doc = "Field `cpupll_sdmin` reader - "]
pub type CpupllSdminR = crate::FieldReader<u32>;
#[doc = "Field `cpupll_sdmin` writer - "]
pub type CpupllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `cpupll_sdm_bypass` reader - "]
pub type CpupllSdmBypassR = crate::BitReader;
#[doc = "Field `cpupll_sdm_bypass` writer - "]
pub type CpupllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn cpupll_sdmin(&self) -> CpupllSdminR {
        CpupllSdminR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cpupll_sdm_bypass(&self) -> CpupllSdmBypassR {
        CpupllSdmBypassR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn cpupll_sdmin(&mut self) -> CpupllSdminW<'_, CpuPllCfg6Spec> {
        CpupllSdminW::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cpupll_sdm_bypass(&mut self) -> CpupllSdmBypassW<'_, CpuPllCfg6Spec> {
        CpupllSdmBypassW::new(self, 24)
    }
}
#[doc = "CPU PLL Configuration 6 - SDM Input\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg6Spec;
impl crate::RegisterSpec for CpuPllCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg6::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg6::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG6 to value 0x0001_61e5"]
impl crate::Resettable for CpuPllCfg6Spec {
    const RESET_VALUE: u32 = 0x0001_61e5;
}
