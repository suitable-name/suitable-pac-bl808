#[doc = "Register `CPU1_IPC_ISR` reader"]
pub type R = crate::R<Cpu1IpcIsrSpec>;
#[doc = "Register `CPU1_IPC_ISR` writer"]
pub type W = crate::W<Cpu1IpcIsrSpec>;
#[doc = "Field `CPU1_IPC_ISR` reader - Interrupt status for CPU1"]
pub type Cpu1IpcIsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt status for CPU1"]
    #[inline(always)]
    pub fn cpu1_ipc_isr(&self) -> Cpu1IpcIsrR {
        Cpu1IpcIsrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "CPU1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIsrSpec;
impl crate::RegisterSpec for Cpu1IpcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_isr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_isr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_ISR to value 0"]
impl crate::Resettable for Cpu1IpcIsrSpec {}
