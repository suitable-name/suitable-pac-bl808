#[doc = "Register `CPU_CORE_CFG13` reader"]
pub type R = crate::R<CpuCoreCfg13Spec>;
#[doc = "Register `CPU_CORE_CFG13` writer"]
pub type W = crate::W<CpuCoreCfg13Spec>;
#[doc = "Field `e902_rst_addr` reader - "]
pub type E902RstAddrR = crate::FieldReader<u32>;
#[doc = "Field `e902_rst_addr` writer - "]
pub type E902RstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn e902_rst_addr(&self) -> E902RstAddrR {
        E902RstAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn e902_rst_addr(&mut self) -> E902RstAddrW<'_, CpuCoreCfg13Spec> {
        E902RstAddrW::new(self, 0)
    }
}
#[doc = "E902 Reset Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg13Spec;
impl crate::RegisterSpec for CpuCoreCfg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg13::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg13Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg13::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG13 to value 0x2201_0000"]
impl crate::Resettable for CpuCoreCfg13Spec {
    const RESET_VALUE: u32 = 0x2201_0000;
}
