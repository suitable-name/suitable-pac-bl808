#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `transmit_request` writer - "]
pub type TransmitRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_transmission` writer - "]
pub type AbortTransmissionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `release_receive_buffer` writer - "]
pub type ReleaseReceiveBufferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clear_data_overrun` writer - "]
pub type ClearDataOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `self_request` writer - "]
pub type SelfRequestW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn transmit_request(&mut self) -> TransmitRequestW<'_, CommandSpec> {
        TransmitRequestW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn abort_transmission(&mut self) -> AbortTransmissionW<'_, CommandSpec> {
        AbortTransmissionW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn release_receive_buffer(&mut self) -> ReleaseReceiveBufferW<'_, CommandSpec> {
        ReleaseReceiveBufferW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clear_data_overrun(&mut self) -> ClearDataOverrunW<'_, CommandSpec> {
        ClearDataOverrunW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn self_request(&mut self) -> SelfRequestW<'_, CommandSpec> {
        SelfRequestW::new(self, 4)
    }
}
#[doc = "Command Register (Write Only)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {}
