#[doc = "Register `CPU1_IPC_IUSR` reader"]
pub type R = crate::R<Cpu1IpcIusrSpec>;
#[doc = "Register `CPU1_IPC_IUSR` writer"]
pub type W = crate::W<Cpu1IpcIusrSpec>;
#[doc = "Field `CPU1_IPC_IUSR` reader - Interrupt unmask set for CPU1"]
pub type Cpu1IpcIusrR = crate::FieldReader<u16>;
#[doc = "Field `CPU1_IPC_IUSR` writer - Interrupt unmask set for CPU1"]
pub type Cpu1IpcIusrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt unmask set for CPU1"]
    #[inline(always)]
    pub fn cpu1_ipc_iusr(&self) -> Cpu1IpcIusrR {
        Cpu1IpcIusrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt unmask set for CPU1"]
    #[inline(always)]
    pub fn cpu1_ipc_iusr(&mut self) -> Cpu1IpcIusrW<'_, Cpu1IpcIusrSpec> {
        Cpu1IpcIusrW::new(self, 0)
    }
}
#[doc = "CPU1 Interrupt Unmask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_iusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_iusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIusrSpec;
impl crate::RegisterSpec for Cpu1IpcIusrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_iusr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIusrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_iusr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIusrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_IUSR to value 0"]
impl crate::Resettable for Cpu1IpcIusrSpec {}
