#[doc = "Register `CPU0_IPC_IUCR` reader"]
pub type R = crate::R<Cpu0IpcIucrSpec>;
#[doc = "Register `CPU0_IPC_IUCR` writer"]
pub type W = crate::W<Cpu0IpcIucrSpec>;
#[doc = "Field `CPU0_IPC_IUCR` writer - Write 1 to clear unmask for CPU0"]
pub type Cpu0IpcIucrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write 1 to clear unmask for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_iucr(&mut self) -> Cpu0IpcIucrW<'_, Cpu0IpcIucrSpec> {
        Cpu0IpcIucrW::new(self, 0)
    }
}
#[doc = "CPU0 Interrupt Unmask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_iucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_iucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIucrSpec;
impl crate::RegisterSpec for Cpu0IpcIucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_iucr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIucrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_iucr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIucrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_IUCR to value 0"]
impl crate::Resettable for Cpu0IpcIucrSpec {}
