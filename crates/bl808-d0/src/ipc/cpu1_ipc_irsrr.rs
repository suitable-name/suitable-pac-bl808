#[doc = "Register `CPU1_IPC_IRSRR` reader"]
pub type R = crate::R<Cpu1IpcIrsrrSpec>;
#[doc = "Register `CPU1_IPC_IRSRR` writer"]
pub type W = crate::W<Cpu1IpcIrsrrSpec>;
#[doc = "Field `CPU1_IPC_IRSRR` reader - Raw interrupt status for CPU1"]
pub type Cpu1IpcIrsrrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for CPU1"]
    #[inline(always)]
    pub fn cpu1_ipc_irsrr(&self) -> Cpu1IpcIrsrrR {
        Cpu1IpcIrsrrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "CPU1 Interrupt Raw Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_irsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_irsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIrsrrSpec;
impl crate::RegisterSpec for Cpu1IpcIrsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_irsrr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIrsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_irsrr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIrsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_IRSRR to value 0"]
impl crate::Resettable for Cpu1IpcIrsrrSpec {}
