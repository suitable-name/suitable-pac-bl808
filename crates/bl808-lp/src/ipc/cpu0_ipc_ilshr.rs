#[doc = "Register `CPU0_IPC_ILSHR` reader"]
pub type R = crate::R<Cpu0IpcIlshrSpec>;
#[doc = "Register `CPU0_IPC_ILSHR` writer"]
pub type W = crate::W<Cpu0IpcIlshrSpec>;
#[doc = "Field `CPU0_IPC_ILSHR` reader - Interrupt line selection high"]
pub type Cpu0IpcIlshrR = crate::FieldReader<u32>;
#[doc = "Field `CPU0_IPC_ILSHR` writer - Interrupt line selection high"]
pub type Cpu0IpcIlshrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt line selection high"]
    #[inline(always)]
    pub fn cpu0_ipc_ilshr(&self) -> Cpu0IpcIlshrR {
        Cpu0IpcIlshrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt line selection high"]
    #[inline(always)]
    pub fn cpu0_ipc_ilshr(&mut self) -> Cpu0IpcIlshrW<'_, Cpu0IpcIlshrSpec> {
        Cpu0IpcIlshrW::new(self, 0)
    }
}
#[doc = "CPU0 Interrupt Line Sel High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_ilshr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_ilshr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu0IpcIlshrSpec;
impl crate::RegisterSpec for Cpu0IpcIlshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu0_ipc_ilshr::R`](R) reader structure"]
impl crate::Readable for Cpu0IpcIlshrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu0_ipc_ilshr::W`](W) writer structure"]
impl crate::Writable for Cpu0IpcIlshrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU0_IPC_ILSHR to value 0"]
impl crate::Resettable for Cpu0IpcIlshrSpec {}
