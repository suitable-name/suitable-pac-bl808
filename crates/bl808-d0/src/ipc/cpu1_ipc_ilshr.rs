#[doc = "Register `CPU1_IPC_ILSHR` reader"]
pub type R = crate::R<Cpu1IpcIlshrSpec>;
#[doc = "Register `CPU1_IPC_ILSHR` writer"]
pub type W = crate::W<Cpu1IpcIlshrSpec>;
#[doc = "Field `CPU1_IPC_ILSHR` reader - Interrupt line selection high"]
pub type Cpu1IpcIlshrR = crate::FieldReader<u32>;
#[doc = "Field `CPU1_IPC_ILSHR` writer - Interrupt line selection high"]
pub type Cpu1IpcIlshrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt line selection high"]
    #[inline(always)]
    pub fn cpu1_ipc_ilshr(&self) -> Cpu1IpcIlshrR {
        Cpu1IpcIlshrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt line selection high"]
    #[inline(always)]
    pub fn cpu1_ipc_ilshr(&mut self) -> Cpu1IpcIlshrW<'_, Cpu1IpcIlshrSpec> {
        Cpu1IpcIlshrW::new(self, 0)
    }
}
#[doc = "CPU1 Interrupt Line Sel High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_ilshr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_ilshr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1IpcIlshrSpec;
impl crate::RegisterSpec for Cpu1IpcIlshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_ipc_ilshr::R`](R) reader structure"]
impl crate::Readable for Cpu1IpcIlshrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_ipc_ilshr::W`](W) writer structure"]
impl crate::Writable for Cpu1IpcIlshrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU1_IPC_ILSHR to value 0"]
impl crate::Resettable for Cpu1IpcIlshrSpec {}
