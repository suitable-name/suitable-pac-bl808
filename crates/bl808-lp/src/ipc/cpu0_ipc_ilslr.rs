#[doc = "Register `CPU0_IPC_ILSLR` reader"]
pub type R = crate::R<Cpu0IpcIlslrSpec>;
#[doc = "Register `CPU0_IPC_ILSLR` writer"]
pub type W = crate::W<Cpu0IpcIlslrSpec>;
#[doc = "Field `CPU0_IPC_ILSLR` reader - Interrupt line selection low"]
pub type Cpu0IpcIlslrR = crate::FieldReader<u32>;
#[doc = "Field `CPU0_IPC_ILSLR` writer - Interrupt line selection low"]
pub type Cpu0IpcIlslrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt line selection low"]
    #[inline(always)]
    pub fn cpu0_ipc_ilslr(&self) -> Cpu0IpcIlslrR {
        Cpu0IpcIlslrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt line selection low"]
    #[inline(always)]
    pub fn cpu0_ipc_ilslr(&mut self) -> Cpu0IpcIlslrW<'_, Cpu0IpcIlslrSpec> {
        Cpu0IpcIlslrW::new(self, 0)
    }
}
#[doc = "CPU0 Interrupt Line Sel Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_ilslr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_ilslr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIlslrSpec;
impl crate::RegisterSpec for Cpu0IpcIlslrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_ilslr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIlslrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_ilslr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIlslrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_ILSLR to value 0"]
impl crate::Resettable for Cpu0IpcIlslrSpec {}
