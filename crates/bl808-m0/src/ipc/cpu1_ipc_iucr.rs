#[doc = "Register `CPU1_IPC_IUCR` reader"]
pub type R = crate::R<Cpu1IpcIucrSpec>;
#[doc = "Register `CPU1_IPC_IUCR` writer"]
pub type W = crate::W<Cpu1IpcIucrSpec>;
#[doc = "Field `CPU1_IPC_IUCR` writer - Write 1 to clear unmask for CPU1"]
pub type Cpu1IpcIucrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write 1 to clear unmask for CPU1"]
    #[inline(always)]
    pub fn cpu1_ipc_iucr(&mut self) -> Cpu1IpcIucrW<'_, Cpu1IpcIucrSpec> {
        Cpu1IpcIucrW::new(self, 0)
    }
}
#[doc = "CPU1 Interrupt Unmask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_iucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_iucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIucrSpec;
impl crate::RegisterSpec for Cpu1IpcIucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_iucr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIucrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_iucr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIucrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_IUCR to value 0"]
impl crate::Resettable for Cpu1IpcIucrSpec {}
