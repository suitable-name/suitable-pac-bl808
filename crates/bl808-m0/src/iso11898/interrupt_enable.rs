#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `receive_int_en` reader - "]
pub type ReceiveIntEnR = crate::BitReader;
#[doc = "Field `receive_int_en` writer - "]
pub type ReceiveIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `transmit_int_en` reader - "]
pub type TransmitIntEnR = crate::BitReader;
#[doc = "Field `transmit_int_en` writer - "]
pub type TransmitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_int_en` reader - "]
pub type ErrorIntEnR = crate::BitReader;
#[doc = "Field `error_int_en` writer - "]
pub type ErrorIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `data_overrun_int_en` reader - "]
pub type DataOverrunIntEnR = crate::BitReader;
#[doc = "Field `data_overrun_int_en` writer - "]
pub type DataOverrunIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error_passive_int_en` reader - "]
pub type ErrorPassiveIntEnR = crate::BitReader;
#[doc = "Field `error_passive_int_en` writer - "]
pub type ErrorPassiveIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `arb_lost_int_en` reader - "]
pub type ArbLostIntEnR = crate::BitReader;
#[doc = "Field `arb_lost_int_en` writer - "]
pub type ArbLostIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bus_error_int_en` reader - "]
pub type BusErrorIntEnR = crate::BitReader;
#[doc = "Field `bus_error_int_en` writer - "]
pub type BusErrorIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn receive_int_en(&self) -> ReceiveIntEnR {
        ReceiveIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn transmit_int_en(&self) -> TransmitIntEnR {
        TransmitIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn error_int_en(&self) -> ErrorIntEnR {
        ErrorIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_overrun_int_en(&self) -> DataOverrunIntEnR {
        DataOverrunIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn error_passive_int_en(&self) -> ErrorPassiveIntEnR {
        ErrorPassiveIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn arb_lost_int_en(&self) -> ArbLostIntEnR {
        ArbLostIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bus_error_int_en(&self) -> BusErrorIntEnR {
        BusErrorIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn receive_int_en(&mut self) -> ReceiveIntEnW<'_, InterruptEnableSpec> {
        ReceiveIntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn transmit_int_en(&mut self) -> TransmitIntEnW<'_, InterruptEnableSpec> {
        TransmitIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn error_int_en(&mut self) -> ErrorIntEnW<'_, InterruptEnableSpec> {
        ErrorIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_overrun_int_en(&mut self) -> DataOverrunIntEnW<'_, InterruptEnableSpec> {
        DataOverrunIntEnW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn error_passive_int_en(&mut self) -> ErrorPassiveIntEnW<'_, InterruptEnableSpec> {
        ErrorPassiveIntEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn arb_lost_int_en(&mut self) -> ArbLostIntEnW<'_, InterruptEnableSpec> {
        ArbLostIntEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bus_error_int_en(&mut self) -> BusErrorIntEnW<'_, InterruptEnableSpec> {
        BusErrorIntEnW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for InterruptEnableSpec {}
