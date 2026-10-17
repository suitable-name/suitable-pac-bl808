#[doc = "Register `Control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `Control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `TransferSize` reader - Total number of transfers"]
pub type TransferSizeR = crate::FieldReader<u16>;
#[doc = "Field `TransferSize` writer - Total number of transfers"]
pub type TransferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SBSize` reader - Source burst size"]
pub type SbsizeR = crate::FieldReader;
#[doc = "Field `SBSize` writer - Source burst size"]
pub type SbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_MIN_MODE` reader - "]
pub type DstMinModeR = crate::BitReader;
#[doc = "Field `DST_MIN_MODE` writer - "]
pub type DstMinModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSize` reader - Destination burst size"]
pub type DbsizeR = crate::FieldReader;
#[doc = "Field `DBSize` writer - Destination burst size"]
pub type DbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_ADD_MODE` reader - "]
pub type DstAddModeR = crate::BitReader;
#[doc = "Field `DST_ADD_MODE` writer - "]
pub type DstAddModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWidth` reader - Source transfer width"]
pub type SwidthR = crate::FieldReader;
#[doc = "Field `SWidth` writer - Source transfer width"]
pub type SwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DWidth` reader - Destination transfer width"]
pub type DwidthR = crate::FieldReader;
#[doc = "Field `DWidth` writer - Destination transfer width"]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIX_CNT` reader - "]
pub type FixCntR = crate::FieldReader;
#[doc = "Field `FIX_CNT` writer - "]
pub type FixCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bits 0:11 - Total number of transfers"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TransferSizeR {
        TransferSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Source burst size"]
    #[inline(always)]
    pub fn sbsize(&self) -> SbsizeR {
        SbsizeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dst_min_mode(&self) -> DstMinModeR {
        DstMinModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Destination burst size"]
    #[inline(always)]
    pub fn dbsize(&self) -> DbsizeR {
        DbsizeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dst_add_mode(&self) -> DstAddModeR {
        DstAddModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Source transfer width"]
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Destination transfer width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn fix_cnt(&self) -> FixCntR {
        FixCntR::new(((self.bits >> 23) & 7) as u8)
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
    #[doc = "Bits 0:11 - Total number of transfers"]
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TransferSizeW<'_, ControlSpec> {
        TransferSizeW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Source burst size"]
    #[inline(always)]
    pub fn sbsize(&mut self) -> SbsizeW<'_, ControlSpec> {
        SbsizeW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dst_min_mode(&mut self) -> DstMinModeW<'_, ControlSpec> {
        DstMinModeW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Destination burst size"]
    #[inline(always)]
    pub fn dbsize(&mut self) -> DbsizeW<'_, ControlSpec> {
        DbsizeW::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dst_add_mode(&mut self) -> DstAddModeW<'_, ControlSpec> {
        DstAddModeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Source transfer width"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SwidthW<'_, ControlSpec> {
        SwidthW::new(self, 18)
    }
    #[doc = "Bits 21:22 - Destination transfer width"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<'_, ControlSpec> {
        DwidthW::new(self, 21)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn fix_cnt(&mut self) -> FixCntW<'_, ControlSpec> {
        FixCntW::new(self, 23)
    }
    #[doc = "Bit 26 - Source increment"]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, ControlSpec> {
        SiW::new(self, 26)
    }
    #[doc = "Bit 27 - Destination increment"]
    #[inline(always)]
    pub fn di(&mut self) -> DiW<'_, ControlSpec> {
        DiW::new(self, 27)
    }
    #[doc = "Bits 28:30 - Protection control"]
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<'_, ControlSpec> {
        ProtW::new(self, 28)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable"]
    #[inline(always)]
    pub fn i(&mut self) -> IW<'_, ControlSpec> {
        IW::new(self, 31)
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Control to value 0x0c48_9000"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x0c48_9000;
}
