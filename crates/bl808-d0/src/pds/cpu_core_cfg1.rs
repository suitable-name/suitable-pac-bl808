#[doc = "Register `CPU_CORE_CFG1` reader"]
pub type R = crate::R<CpuCoreCfg1Spec>;
#[doc = "Register `CPU_CORE_CFG1` writer"]
pub type W = crate::W<CpuCoreCfg1Spec>;
#[doc = "Field `reg_pll_sel` reader - "]
pub type RegPllSelR = crate::FieldReader;
#[doc = "Field `reg_pll_sel` writer - "]
pub type RegPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mcu1_clk_en` reader - "]
pub type RegMcu1ClkEnR = crate::BitReader;
#[doc = "Field `reg_mcu1_clk_en` writer - "]
pub type RegMcu1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&self) -> RegPllSelR {
        RegPllSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu1_clk_en(&self) -> RegMcu1ClkEnR {
        RegMcu1ClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&mut self) -> RegPllSelW<'_, CpuCoreCfg1Spec> {
        RegPllSelW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu1_clk_en(&mut self) -> RegMcu1ClkEnW<'_, CpuCoreCfg1Spec> {
        RegMcu1ClkEnW::new(self, 8)
    }
}
#[doc = "C906 core clock enable and PLL selection\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg1Spec;
impl crate::RegisterSpec for CpuCoreCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg1::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg1::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG1 to value 0x0130"]
impl crate::Resettable for CpuCoreCfg1Spec {
    const RESET_VALUE: u32 = 0x0130;
}
