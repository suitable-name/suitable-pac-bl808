#[doc = "Register `CPU_PLL_CFG5` reader"]
pub type R = crate::R<CpuPllCfg5Spec>;
#[doc = "Register `CPU_PLL_CFG5` writer"]
pub type W = crate::W<CpuPllCfg5Spec>;
#[doc = "Field `cpupll_vco_speed` reader - "]
pub type CpupllVcoSpeedR = crate::FieldReader;
#[doc = "Field `cpupll_vco_speed` writer - "]
pub type CpupllVcoSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cpupll_vco_speed(&self) -> CpupllVcoSpeedR {
        CpupllVcoSpeedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cpupll_vco_speed(&mut self) -> CpupllVcoSpeedW<'_, CpuPllCfg5Spec> {
        CpupllVcoSpeedW::new(self, 0)
    }
}
#[doc = "CPU PLL Configuration 5 - VCO Speed\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg5Spec;
impl crate::RegisterSpec for CpuPllCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg5::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg5::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG5 to value 0x03"]
impl crate::Resettable for CpuPllCfg5Spec {
    const RESET_VALUE: u32 = 0x03;
}
