#[doc = "Register `CPU0_IPC_ISR` reader"]
pub type R = crate::R<Cpu0IpcIsrSpec>;
#[doc = "Register `CPU0_IPC_ISR` writer"]
pub type W = crate::W<Cpu0IpcIsrSpec>;
#[doc = "Field `CPU0_IPC_ISR` reader - Interrupt status for CPU0"]
pub type Cpu0IpcIsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt status for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_isr(&self) -> Cpu0IpcIsrR {
        Cpu0IpcIsrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "CPU0 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIsrSpec;
impl crate::RegisterSpec for Cpu0IpcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_isr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_isr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_ISR to value 0"]
impl crate::Resettable for Cpu0IpcIsrSpec {}
