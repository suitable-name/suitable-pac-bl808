#[doc = "Register `CPU0_IPC_ICR` reader"]
pub type R = crate::R<Cpu0IpcIcrSpec>;
#[doc = "Register `CPU0_IPC_ICR` writer"]
pub type W = crate::W<Cpu0IpcIcrSpec>;
#[doc = "Field `CPU0_IPC_ICR` writer - Write 1 to clear interrupt for CPU0"]
pub type Cpu0IpcIcrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write 1 to clear interrupt for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_icr(&mut self) -> Cpu0IpcIcrW<'_, Cpu0IpcIcrSpec> {
        Cpu0IpcIcrW::new(self, 0)
    }
}
#[doc = "CPU0 Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIcrSpec;
impl crate::RegisterSpec for Cpu0IpcIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_icr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_icr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_ICR to value 0"]
impl crate::Resettable for Cpu0IpcIcrSpec {}
