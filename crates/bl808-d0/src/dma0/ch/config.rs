#[doc = "Register `Config` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `Config` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `E` reader - Channel enable"]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - Channel enable"]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SrcPeripheral` reader - "]
pub type SrcPeripheralR = crate::FieldReader;
#[doc = "Field `SrcPeripheral` writer - "]
pub type SrcPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DstPeripheral` reader - "]
pub type DstPeripheralR = crate::FieldReader;
#[doc = "Field `DstPeripheral` writer - "]
pub type DstPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FlowCntrl` reader - 0:M2M, 1:M2P, 2:P2M, 3:P2P"]
pub type FlowCntrlR = crate::FieldReader;
#[doc = "Field `FlowCntrl` writer - 0:M2M, 1:M2P, 2:P2M, 3:P2P"]
pub type FlowCntrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IE` reader - Interrupt error mask"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt error mask"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITC` reader - Terminal count interrupt mask"]
pub type ItcR = crate::BitReader;
#[doc = "Field `ITC` writer - Terminal count interrupt mask"]
pub type ItcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L` reader - Lock"]
pub type LR = crate::BitReader;
#[doc = "Field `L` writer - Lock"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A` reader - Active status"]
pub type AR = crate::BitReader;
#[doc = "Field `H` reader - Halt"]
pub type HR = crate::BitReader;
#[doc = "Field `H` writer - Halt"]
pub type HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLICounter` reader - "]
pub type LlicounterR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SrcPeripheralR {
        SrcPeripheralR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DstPeripheralR {
        DstPeripheralR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13 - 0:M2M, 1:M2P, 2:P2M, 3:P2P"]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FlowCntrlR {
        FlowCntrlR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Interrupt error mask"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask"]
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Active status"]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Halt"]
    #[inline(always)]
    pub fn h(&self) -> HR {
        HR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn llicounter(&self) -> LlicounterR {
        LlicounterR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, ConfigSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&mut self) -> SrcPeripheralW<'_, ConfigSpec> {
        SrcPeripheralW::new(self, 1)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&mut self) -> DstPeripheralW<'_, ConfigSpec> {
        DstPeripheralW::new(self, 6)
    }
    #[doc = "Bits 11:13 - 0:M2M, 1:M2P, 2:P2M, 3:P2P"]
    #[inline(always)]
    pub fn flow_cntrl(&mut self) -> FlowCntrlW<'_, ConfigSpec> {
        FlowCntrlW::new(self, 11)
    }
    #[doc = "Bit 14 - Interrupt error mask"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, ConfigSpec> {
        IeW::new(self, 14)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask"]
    #[inline(always)]
    pub fn itc(&mut self) -> ItcW<'_, ConfigSpec> {
        ItcW::new(self, 15)
    }
    #[doc = "Bit 16 - Lock"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, ConfigSpec> {
        LW::new(self, 16)
    }
    #[doc = "Bit 18 - Halt"]
    #[inline(always)]
    pub fn h(&mut self) -> HW<'_, ConfigSpec> {
        HW::new(self, 18)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Config to value 0"]
impl crate::Resettable for ConfigSpec {}
