#[doc = "Register `CPU1_IPC_ILSLR` reader"]
pub type R = crate::R<Cpu1IpcIlslrSpec>;
#[doc = "Register `CPU1_IPC_ILSLR` writer"]
pub type W = crate::W<Cpu1IpcIlslrSpec>;
#[doc = "Field `CPU1_IPC_ILSLR` reader - Interrupt line selection low"]
pub type Cpu1IpcIlslrR = crate::FieldReader<u32>;
#[doc = "Field `CPU1_IPC_ILSLR` writer - Interrupt line selection low"]
pub type Cpu1IpcIlslrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt line selection low"]
    #[inline(always)]
    pub fn cpu1_ipc_ilslr(&self) -> Cpu1IpcIlslrR {
        Cpu1IpcIlslrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt line selection low"]
    #[inline(always)]
    pub fn cpu1_ipc_ilslr(&mut self) -> Cpu1IpcIlslrW<'_, Cpu1IpcIlslrSpec> {
        Cpu1IpcIlslrW::new(self, 0)
    }
}
#[doc = "CPU1 Interrupt Line Sel Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_ilslr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_ilslr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIlslrSpec;
impl crate::RegisterSpec for Cpu1IpcIlslrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_ilslr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIlslrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_ilslr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIlslrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_ILSLR to value 0"]
impl crate::Resettable for Cpu1IpcIlslrSpec {}
