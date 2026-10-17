#[doc = "Register `CPU0_IPC_ISWR` reader"]
pub type R = crate::R<Cpu0IpcIswrSpec>;
#[doc = "Register `CPU0_IPC_ISWR` writer"]
pub type W = crate::W<Cpu0IpcIswrSpec>;
#[doc = "Field `CPU0_IPC_ISWR` writer - Write 1 to set interrupt for CPU0"]
pub type Cpu0IpcIswrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write 1 to set interrupt for CPU0"]
    #[inline(always)]
    pub fn cpu0_ipc_iswr(&mut self) -> Cpu0IpcIswrW<'_, Cpu0IpcIswrSpec> {
        Cpu0IpcIswrW::new(self, 0)
    }
}
#[doc = "CPU0 Interrupt Set Write Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_iswr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_iswr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIswrSpec;
impl crate::RegisterSpec for Cpu0IpcIswrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_iswr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIswrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_iswr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIswrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_ISWR to value 0"]
impl crate::Resettable for Cpu0IpcIswrSpec {}
