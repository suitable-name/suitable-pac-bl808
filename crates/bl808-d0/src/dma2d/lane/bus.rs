#[doc = "Register `BUS` reader"]
pub type R = crate::R<BusSpec>;
#[doc = "Register `BUS` writer"]
pub type W = crate::W<BusSpec>;
#[doc = "Field `TransferSize` reader - Number of transfers (bursts or singles)"]
pub type TransferSizeR = crate::FieldReader<u16>;
#[doc = "Field `TransferSize` writer - Number of transfers (bursts or singles)"]
pub type TransferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SRC_BURST` reader - Source burst size: 0:1, 1:4, 2:8, 3:16"]
pub type SrcBurstR = crate::FieldReader;
#[doc = "Field `SRC_BURST` writer - Source burst size: 0:1, 1:4, 2:8, 3:16"]
pub type SrcBurstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_BURST` reader - Destination burst size: 0:1, 1:4, 2:8, 3:16"]
pub type DstBurstR = crate::FieldReader;
#[doc = "Field `DST_BURST` writer - Destination burst size: 0:1, 1:4, 2:8, 3:16"]
pub type DstBurstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRC_SIZE` reader - Source transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
pub type SrcSizeR = crate::FieldReader;
#[doc = "Field `SRC_SIZE` writer - Source transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
pub type SrcSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_SIZE` reader - Destination transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
pub type DstSizeR = crate::FieldReader;
#[doc = "Field `DST_SIZE` writer - Destination transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
pub type DstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SI` reader - Source increment"]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - Source increment"]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI` reader - Destination increment"]
pub type DiR = crate::BitReader;
#[doc = "Field `DI` writer - Destination increment"]
pub type DiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Prot` reader - Protection control"]
pub type ProtR = crate::FieldReader;
#[doc = "Field `Prot` writer - Protection control"]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I` reader - Terminal count interrupt enable"]
pub type IR = crate::BitReader;
#[doc = "Field `I` writer - Terminal count interrupt enable"]
pub type IW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Number of transfers (bursts or singles)"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TransferSizeR {
        TransferSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Source burst size: 0:1, 1:4, 2:8, 3:16"]
    #[inline(always)]
    pub fn src_burst(&self) -> SrcBurstR {
        SrcBurstR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Destination burst size: 0:1, 1:4, 2:8, 3:16"]
    #[inline(always)]
    pub fn dst_burst(&self) -> DstBurstR {
        DstBurstR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Source transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
    #[inline(always)]
    pub fn src_size(&self) -> SrcSizeR {
        SrcSizeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Destination transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
    #[inline(always)]
    pub fn dst_size(&self) -> DstSizeR {
        DstSizeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 26 - Source increment"]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Destination increment"]
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Protection control"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable"]
    #[inline(always)]
    pub fn i(&self) -> IR {
        IR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of transfers (bursts or singles)"]
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TransferSizeW<'_, BusSpec> {
        TransferSizeW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Source burst size: 0:1, 1:4, 2:8, 3:16"]
    #[inline(always)]
    pub fn src_burst(&mut self) -> SrcBurstW<'_, BusSpec> {
        SrcBurstW::new(self, 12)
    }
    #[doc = "Bits 15:16 - Destination burst size: 0:1, 1:4, 2:8, 3:16"]
    #[inline(always)]
    pub fn dst_burst(&mut self) -> DstBurstW<'_, BusSpec> {
        DstBurstW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Source transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
    #[inline(always)]
    pub fn src_size(&mut self) -> SrcSizeW<'_, BusSpec> {
        SrcSizeW::new(self, 18)
    }
    #[doc = "Bits 21:22 - Destination transfer width: 0:8-bit, 1:16-bit, 2:32-bit"]
    #[inline(always)]
    pub fn dst_size(&mut self) -> DstSizeW<'_, BusSpec> {
        DstSizeW::new(self, 21)
    }
    #[doc = "Bit 26 - Source increment"]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, BusSpec> {
        SiW::new(self, 26)
    }
    #[doc = "Bit 27 - Destination increment"]
    #[inline(always)]
    pub fn di(&mut self) -> DiW<'_, BusSpec> {
        DiW::new(self, 27)
    }
    #[doc = "Bits 28:30 - Protection control"]
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<'_, BusSpec> {
        ProtW::new(self, 28)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable"]
    #[inline(always)]
    pub fn i(&mut self) -> IW<'_, BusSpec> {
        IW::new(self, 31)
    }
}
#[doc = "Lane Bus and Burst Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusSpec;
impl crate::RegisterSpec for BusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus::R`](R) reader structure"]
impl crate::Readable for BusSpec {}
#[doc = "`write(|w| ..)` method takes [`bus::W`](W) writer structure"]
impl crate::Writable for BusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS to value 0x0c48_9000"]
impl crate::Resettable for BusSpec {
    const RESET_VALUE: u32 = 0x0c48_9000;
}
