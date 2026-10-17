#[doc = "Register `CPU0_IPC_IRSRR` reader"]
pub type R = crate::R<Cpu0IpcIrsrrSpec>;
#[doc = "Register `CPU0_IPC_IRSRR` writer"]
pub type W = crate::W<Cpu0IpcIrsrrSpec>;
#[doc = "Field `CPU0_IPC_IRSRR` reader - Raw interrupt status for CPU0"]
pub type Cpu0IpcIrsrrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_irsrr(&self) -> Cpu0IpcIrsrrR {
        Cpu0IpcIrsrrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "CPU0 Interrupt Raw Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_irsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_irsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIrsrrSpec;
impl crate::RegisterSpec for Cpu0IpcIrsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_irsrr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIrsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_irsrr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIrsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_IRSRR to value 0"]
impl crate::Resettable for Cpu0IpcIrsrrSpec {}
