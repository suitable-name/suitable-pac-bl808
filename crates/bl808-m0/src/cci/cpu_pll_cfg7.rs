#[doc = "Register `CPU_PLL_CFG7` reader"]
pub type R = crate::R<CpuPllCfg7Spec>;
#[doc = "Register `CPU_PLL_CFG7` writer"]
pub type W = crate::W<CpuPllCfg7Spec>;
#[doc = "Field `cpupll_sdm_order_sel` reader - "]
pub type CpupllSdmOrderSelR = crate::BitReader;
#[doc = "Field `cpupll_sdm_order_sel` writer - "]
pub type CpupllSdmOrderSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_sdm_sig_dith_sel` reader - "]
pub type CpupllSdmSigDithSelR = crate::FieldReader;
#[doc = "Field `cpupll_sdm_sig_dith_sel` writer - "]
pub type CpupllSdmSigDithSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sdm_order_sel(&self) -> CpupllSdmOrderSelR {
        CpupllSdmOrderSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cpupll_sdm_sig_dith_sel(&self) -> CpupllSdmSigDithSelR {
        CpupllSdmSigDithSelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sdm_order_sel(&mut self) -> CpupllSdmOrderSelW<'_, CpuPllCfg7Spec> {
        CpupllSdmOrderSelW::new(self, 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cpupll_sdm_sig_dith_sel(&mut self) -> CpupllSdmSigDithSelW<'_, CpuPllCfg7Spec> {
        CpupllSdmSigDithSelW::new(self, 16)
    }
}
#[doc = "CPU PLL Configuration 7 - SDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg7Spec;
impl crate::RegisterSpec for CpuPllCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg7::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg7::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG7 to value 0x01"]
impl crate::Resettable for CpuPllCfg7Spec {
    const RESET_VALUE: u32 = 0x01;
}
