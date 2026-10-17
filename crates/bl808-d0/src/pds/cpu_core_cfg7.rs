#[doc = "Register `CPU_CORE_CFG7` reader"]
pub type R = crate::R<CpuCoreCfg7Spec>;
#[doc = "Register `CPU_CORE_CFG7` writer"]
pub type W = crate::W<CpuCoreCfg7Spec>;
#[doc = "Field `reg_pico_div` reader - "]
pub type RegPicoDivR = crate::FieldReader;
#[doc = "Field `reg_pico_div` writer - "]
pub type RegPicoDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `e902_lpmd_b` reader - "]
pub type E902LpmdBR = crate::FieldReader;
#[doc = "Field `pico_rst_mask` reader - "]
pub type PicoRstMaskR = crate::BitReader;
#[doc = "Field `pico_rst_mask` writer - "]
pub type PicoRstMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_pico_div(&self) -> RegPicoDivR {
        RegPicoDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn e902_lpmd_b(&self) -> E902LpmdBR {
        E902LpmdBR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pico_rst_mask(&self) -> PicoRstMaskR {
        PicoRstMaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_pico_div(&mut self) -> RegPicoDivW<'_, CpuCoreCfg7Spec> {
        RegPicoDivW::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pico_rst_mask(&mut self) -> PicoRstMaskW<'_, CpuCoreCfg7Spec> {
        PicoRstMaskW::new(self, 31)
    }
}
#[doc = "E902 (PICO) core clock divider and low power state\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg7Spec;
impl crate::RegisterSpec for CpuCoreCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg7::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg7::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG7 to value 0x01"]
impl crate::Resettable for CpuCoreCfg7Spec {
    const RESET_VALUE: u32 = 0x01;
}
