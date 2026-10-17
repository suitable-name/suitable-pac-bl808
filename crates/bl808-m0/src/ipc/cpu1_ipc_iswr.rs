#[doc = "Register `CPU1_IPC_ISWR` reader"]
pub type R = crate::R<Cpu1IpcIswrSpec>;
#[doc = "Register `CPU1_IPC_ISWR` writer"]
pub type W = crate::W<Cpu1IpcIswrSpec>;
#[doc = "Field `CPU1_IPC_ISWR` writer - Write 1 to set interrupt for CPU1"]
pub type Cpu1IpcIswrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Write 1 to set interrupt for CPU1"]
    #[inline(always)]
    pub fn cpu1_ipc_iswr(&mut self) -> Cpu1IpcIswrW<'_, Cpu1IpcIswrSpec> {
        Cpu1IpcIswrW::new(self, 0)
    }
}
#[doc = "CPU1 Interrupt Set Write Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_iswr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_iswr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIswrSpec;
impl crate::RegisterSpec for Cpu1IpcIswrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_iswr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIswrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_iswr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIswrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_ISWR to value 0"]
impl crate::Resettable for Cpu1IpcIswrSpec {}
