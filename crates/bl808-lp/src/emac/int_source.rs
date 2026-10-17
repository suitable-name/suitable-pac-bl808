#[doc = "Register `INT_SOURCE` reader"]
pub type R = crate::R<IntSourceSpec>;
#[doc = "Register `INT_SOURCE` writer"]
pub type W = crate::W<IntSourceSpec>;
#[doc = "Field `TXB` reader - Transmit buffer interrupt"]
pub type TxbR = crate::BitReader;
#[doc = "Field `TXB` writer - Transmit buffer interrupt"]
pub type TxbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - Transmit error interrupt"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Transmit error interrupt"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB` reader - Receive frame interrupt"]
pub type RxbR = crate::BitReader;
#[doc = "Field `RXB` writer - Receive frame interrupt"]
pub type RxbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - Receive error interrupt"]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - Receive error interrupt"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Busy (no empty BD) interrupt"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Busy (no empty BD) interrupt"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transmit control frame interrupt"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmit control frame interrupt"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC` reader - Receive control frame interrupt"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXC` writer - Receive control frame interrupt"]
pub type RxcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit buffer interrupt"]
    #[inline(always)]
    pub fn txb(&self) -> TxbR {
        TxbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit error interrupt"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive frame interrupt"]
    #[inline(always)]
    pub fn rxb(&self) -> RxbR {
        RxbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive error interrupt"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy (no empty BD) interrupt"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit control frame interrupt"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive control frame interrupt"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit buffer interrupt"]
    #[inline(always)]
    pub fn txb(&mut self) -> TxbW<'_, IntSourceSpec> {
        TxbW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit error interrupt"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, IntSourceSpec> {
        TxeW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive frame interrupt"]
    #[inline(always)]
    pub fn rxb(&mut self) -> RxbW<'_, IntSourceSpec> {
        RxbW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive error interrupt"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, IntSourceSpec> {
        RxeW::new(self, 3)
    }
    #[doc = "Bit 4 - Busy (no empty BD) interrupt"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<'_, IntSourceSpec> {
        BusyW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit control frame interrupt"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IntSourceSpec> {
        TxcW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive control frame interrupt"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RxcW<'_, IntSourceSpec> {
        RxcW::new(self, 6)
    }
}
#[doc = "Interrupt Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_source::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_source::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSourceSpec;
impl crate::RegisterSpec for IntSourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_source::R`](R) reader structure"]
impl crate::Readable for IntSourceSpec {}
#[doc = "`write(|w| ..)` method takes [`int_source::W`](W) writer structure"]
impl crate::Writable for IntSourceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_SOURCE to value 0"]
impl crate::Resettable for IntSourceSpec {}
