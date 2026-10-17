#[doc = "Register `CPU0_IPC_IUSR` reader"]
pub type R = crate::R<Cpu0IpcIusrSpec>;
#[doc = "Register `CPU0_IPC_IUSR` writer"]
pub type W = crate::W<Cpu0IpcIusrSpec>;
#[doc = "Field `CPU0_IPC_IUSR` reader - Interrupt unmask set for CPU0"]
pub type Cpu0IpcIusrR = crate::FieldReader<u16>;
#[doc = "Field `CPU0_IPC_IUSR` writer - Interrupt unmask set for CPU0"]
pub type Cpu0IpcIusrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt unmask set for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_iusr(&self) -> Cpu0IpcIusrR {
        Cpu0IpcIusrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt unmask set for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_iusr(&mut self) -> Cpu0IpcIusrW<'_, Cpu0IpcIusrSpec> {
        Cpu0IpcIusrW::new(self, 0)
    }
}
#[doc = "CPU0 Interrupt Unmask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_iusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_iusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIusrSpec;
impl crate::RegisterSpec for Cpu0IpcIusrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_iusr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIusrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_iusr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIusrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_IUSR to value 0"]
impl crate::Resettable for Cpu0IpcIusrSpec {}
