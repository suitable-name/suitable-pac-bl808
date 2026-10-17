#[doc = "Register `CPU0_BOOT` reader"]
pub type R = crate::R<Cpu0BootSpec>;
#[doc = "Register `CPU0_BOOT` writer"]
pub type W = crate::W<Cpu0BootSpec>;
#[doc = "Field `reg_cpu0_rvba` reader - RISC-V Reset Vector Base Address for D0 core. Determines where the C906 starts execution after reset."]
pub type RegCpu0RvbaR = crate::FieldReader<u32>;
#[doc = "Field `reg_cpu0_rvba` writer - RISC-V Reset Vector Base Address for D0 core. Determines where the C906 starts execution after reset."]
pub type RegCpu0RvbaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RISC-V Reset Vector Base Address for D0 core. Determines where the C906 starts execution after reset."]
    #[inline(always)]
    pub fn reg_cpu0_rvba(&self) -> RegCpu0RvbaR {
        RegCpu0RvbaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RISC-V Reset Vector Base Address for D0 core. Determines where the C906 starts execution after reset."]
    #[inline(always)]
    pub fn reg_cpu0_rvba(&mut self) -> RegCpu0RvbaW<'_, Cpu0BootSpec> {
        RegCpu0RvbaW::new(self, 0)
    }
}
#[doc = "D0 Core (C906) Boot Vector Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_boot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_boot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0BootSpec;
impl crate::RegisterSpec for Cpu0BootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_boot::R`](R) reader structure"]
impl crate::Readable for Cpu0BootSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_boot::W`](W) writer structure"]
impl crate::Writable for Cpu0BootSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_BOOT to value 0x3eff_0000"]
impl crate::Resettable for Cpu0BootSpec {
    const RESET_VALUE: u32 = 0x3eff_0000;
}
