#[doc = "Register `CPU_CORE_CFG14` reader"]
pub type R = crate::R<CpuCoreCfg14Spec>;
#[doc = "Register `CPU_CORE_CFG14` writer"]
pub type W = crate::W<CpuCoreCfg14Spec>;
#[doc = "Field `e906_rst_addr` reader - "]
pub type E906RstAddrR = crate::FieldReader<u32>;
#[doc = "Field `e906_rst_addr` writer - "]
pub type E906RstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn e906_rst_addr(&self) -> E906RstAddrR {
        E906RstAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn e906_rst_addr(&mut self) -> E906RstAddrW<'_, CpuCoreCfg14Spec> {
        E906RstAddrW::new(self, 0)
    }
}
#[doc = "E906/C906 Reset Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg14Spec;
impl crate::RegisterSpec for CpuCoreCfg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg14::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg14Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg14::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG14 to value 0x9000_0000"]
impl crate::Resettable for CpuCoreCfg14Spec {
    const RESET_VALUE: u32 = 0x9000_0000;
}
