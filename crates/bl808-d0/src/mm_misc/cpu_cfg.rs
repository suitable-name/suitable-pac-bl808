#[doc = "Register `CPU_CFG` reader"]
pub type R = crate::R<CpuCfgSpec>;
#[doc = "Register `CPU_CFG` writer"]
pub type W = crate::W<CpuCfgSpec>;
#[doc = "Field `reg_cpu0_apb_base` reader - D0 APB base address pointer for core-local peripherals."]
pub type RegCpu0ApbBaseR = crate::FieldReader<u16>;
#[doc = "Field `reg_cpu0_apb_base` writer - D0 APB base address pointer for core-local peripherals."]
pub type RegCpu0ApbBaseW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `cpu0_ndm_rstn_en` reader - D0 Non-Debug Mode Reset Enable."]
pub type Cpu0NdmRstnEnR = crate::BitReader;
#[doc = "Field `cpu0_ndm_rstn_en` writer - D0 Non-Debug Mode Reset Enable."]
pub type Cpu0NdmRstnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu0_hart_rstn_en` reader - D0 Core HART Reset Enable."]
pub type Cpu0HartRstnEnR = crate::BitReader;
#[doc = "Field `cpu0_hart_rstn_en` writer - D0 Core HART Reset Enable."]
pub type Cpu0HartRstnEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - D0 APB base address pointer for core-local peripherals."]
    #[inline(always)]
    pub fn reg_cpu0_apb_base(&self) -> RegCpu0ApbBaseR {
        RegCpu0ApbBaseR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 28 - D0 Non-Debug Mode Reset Enable."]
    #[inline(always)]
    pub fn cpu0_ndm_rstn_en(&self) -> Cpu0NdmRstnEnR {
        Cpu0NdmRstnEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - D0 Core HART Reset Enable."]
    #[inline(always)]
    pub fn cpu0_hart_rstn_en(&self) -> Cpu0HartRstnEnR {
        Cpu0HartRstnEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - D0 APB base address pointer for core-local peripherals."]
    #[inline(always)]
    pub fn reg_cpu0_apb_base(&mut self) -> RegCpu0ApbBaseW<'_, CpuCfgSpec> {
        RegCpu0ApbBaseW::new(self, 0)
    }
    #[doc = "Bit 28 - D0 Non-Debug Mode Reset Enable."]
    #[inline(always)]
    pub fn cpu0_ndm_rstn_en(&mut self) -> Cpu0NdmRstnEnW<'_, CpuCfgSpec> {
        Cpu0NdmRstnEnW::new(self, 28)
    }
    #[doc = "Bit 29 - D0 Core HART Reset Enable."]
    #[inline(always)]
    pub fn cpu0_hart_rstn_en(&mut self) -> Cpu0HartRstnEnW<'_, CpuCfgSpec> {
        Cpu0HartRstnEnW::new(self, 29)
    }
}
#[doc = "D0 Core Configuration and Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCfgSpec;
impl crate::RegisterSpec for CpuCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_cfg::R`](R) reader structure"]
impl crate::Readable for CpuCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_cfg::W`](W) writer structure"]
impl crate::Writable for CpuCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CFG to value 0x1c"]
impl crate::Resettable for CpuCfgSpec {
    const RESET_VALUE: u32 = 0x1c;
}
