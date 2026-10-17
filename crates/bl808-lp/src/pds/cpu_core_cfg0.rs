#[doc = "Register `CPU_CORE_CFG0` reader"]
pub type R = crate::R<CpuCoreCfg0Spec>;
#[doc = "Register `CPU_CORE_CFG0` writer"]
pub type W = crate::W<CpuCoreCfg0Spec>;
#[doc = "Field `reg_pico_clk_en` reader - "]
pub type RegPicoClkEnR = crate::BitReader;
#[doc = "Field `reg_pico_clk_en` writer - "]
pub type RegPicoClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `e902_dfs_req` reader - "]
pub type E902DfsReqR = crate::BitReader;
#[doc = "Field `e902_dfs_req` writer - "]
pub type E902DfsReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `e902_dfs_ack` reader - "]
pub type E902DfsAckR = crate::BitReader;
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_pico_clk_en(&self) -> RegPicoClkEnR {
        RegPicoClkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn e902_dfs_req(&self) -> E902DfsReqR {
        E902DfsReqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn e902_dfs_ack(&self) -> E902DfsAckR {
        E902DfsAckR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_pico_clk_en(&mut self) -> RegPicoClkEnW<'_, CpuCoreCfg0Spec> {
        RegPicoClkEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn e902_dfs_req(&mut self) -> E902DfsReqW<'_, CpuCoreCfg0Spec> {
        E902DfsReqW::new(self, 29)
    }
}
#[doc = "E902 (PICO) core clock and DFS control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg0Spec;
impl crate::RegisterSpec for CpuCoreCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg0::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg0::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG0 to value 0"]
impl crate::Resettable for CpuCoreCfg0Spec {}
