#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `RXEN` reader - Receiver enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmitter enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOPRE` reader - No preamble mode"]
pub type NopreR = crate::BitReader;
#[doc = "Field `NOPRE` writer - No preamble mode"]
pub type NopreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRO` reader - Broadcast address enable"]
pub type BroR = crate::BitReader;
#[doc = "Field `BRO` writer - Broadcast address enable"]
pub type BroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO` reader - Promiscuous mode enable"]
pub type ProR = crate::BitReader;
#[doc = "Field `PRO` writer - Promiscuous mode enable"]
pub type ProW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Inter-frame gap check"]
pub type IfgR = crate::BitReader;
#[doc = "Field `IFG` writer - Inter-frame gap check"]
pub type IfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLD` reader - Full duplex mode"]
pub type FulldR = crate::BitReader;
#[doc = "Field `FULLD` writer - Full duplex mode"]
pub type FulldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC Enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC Enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUGEN` reader - Huge frames enable"]
pub type HugenR = crate::BitReader;
#[doc = "Field `HUGEN` writer - Huge frames enable"]
pub type HugenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD` reader - Padding enable"]
pub type PadR = crate::BitReader;
#[doc = "Field `PAD` writer - Padding enable"]
pub type PadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECSMALL` reader - Receive small frame enable"]
pub type RecsmallR = crate::BitReader;
#[doc = "Field `RECSMALL` writer - Receive small frame enable"]
pub type RecsmallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMII_EN` reader - RMII mode enable"]
pub type RmiiEnR = crate::BitReader;
#[doc = "Field `RMII_EN` writer - RMII mode enable"]
pub type RmiiEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No preamble mode"]
    #[inline(always)]
    pub fn nopre(&self) -> NopreR {
        NopreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Broadcast address enable"]
    #[inline(always)]
    pub fn bro(&self) -> BroR {
        BroR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Promiscuous mode enable"]
    #[inline(always)]
    pub fn pro(&self) -> ProR {
        ProR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Inter-frame gap check"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Full duplex mode"]
    #[inline(always)]
    pub fn fulld(&self) -> FulldR {
        FulldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC Enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Huge frames enable"]
    #[inline(always)]
    pub fn hugen(&self) -> HugenR {
        HugenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Padding enable"]
    #[inline(always)]
    pub fn pad(&self) -> PadR {
        PadR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive small frame enable"]
    #[inline(always)]
    pub fn recsmall(&self) -> RecsmallR {
        RecsmallR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RMII mode enable"]
    #[inline(always)]
    pub fn rmii_en(&self) -> RmiiEnR {
        RmiiEnR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, ModeSpec> {
        RxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, ModeSpec> {
        TxenW::new(self, 1)
    }
    #[doc = "Bit 2 - No preamble mode"]
    #[inline(always)]
    pub fn nopre(&mut self) -> NopreW<'_, ModeSpec> {
        NopreW::new(self, 2)
    }
    #[doc = "Bit 3 - Broadcast address enable"]
    #[inline(always)]
    pub fn bro(&mut self) -> BroW<'_, ModeSpec> {
        BroW::new(self, 3)
    }
    #[doc = "Bit 5 - Promiscuous mode enable"]
    #[inline(always)]
    pub fn pro(&mut self) -> ProW<'_, ModeSpec> {
        ProW::new(self, 5)
    }
    #[doc = "Bit 6 - Inter-frame gap check"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IfgW<'_, ModeSpec> {
        IfgW::new(self, 6)
    }
    #[doc = "Bit 10 - Full duplex mode"]
    #[inline(always)]
    pub fn fulld(&mut self) -> FulldW<'_, ModeSpec> {
        FulldW::new(self, 10)
    }
    #[doc = "Bit 13 - CRC Enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, ModeSpec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Huge frames enable"]
    #[inline(always)]
    pub fn hugen(&mut self) -> HugenW<'_, ModeSpec> {
        HugenW::new(self, 14)
    }
    #[doc = "Bit 15 - Padding enable"]
    #[inline(always)]
    pub fn pad(&mut self) -> PadW<'_, ModeSpec> {
        PadW::new(self, 15)
    }
    #[doc = "Bit 16 - Receive small frame enable"]
    #[inline(always)]
    pub fn recsmall(&mut self) -> RecsmallW<'_, ModeSpec> {
        RecsmallW::new(self, 16)
    }
    #[doc = "Bit 17 - RMII mode enable"]
    #[inline(always)]
    pub fn rmii_en(&mut self) -> RmiiEnW<'_, ModeSpec> {
        RmiiEnW::new(self, 17)
    }
}
#[doc = "MAC Mode Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0xa008"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0xa008;
}
