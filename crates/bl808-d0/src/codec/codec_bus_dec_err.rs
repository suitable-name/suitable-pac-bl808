#[doc = "Register `CODEC_BUS_DEC_ERR` reader"]
pub type R = crate::R<CodecBusDecErrSpec>;
#[doc = "Register `CODEC_BUS_DEC_ERR` writer"]
pub type W = crate::W<CodecBusDecErrSpec>;
#[doc = "Field `REG_DEC_ERR_CLR` reader - Clear decode error status"]
pub type RegDecErrClrR = crate::BitReader;
#[doc = "Field `REG_DEC_ERR_CLR` writer - Clear decode error status"]
pub type RegDecErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DEC_ERR_LATCH_LAST` reader - Latch only the last decode error (write 1 to latch)"]
pub type RegDecErrLatchLastR = crate::BitReader;
#[doc = "Field `REG_DEC_ERR_LATCH_LAST` writer - Latch only the last decode error (write 1 to latch)"]
pub type RegDecErrLatchLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEC_HS_DEC_ERR_LAT` reader - Decode error latched status"]
pub type CodecHsDecErrLatR = crate::BitReader;
#[doc = "Field `CODEC_HS_DEC_ERR_WRITE` reader - Indicates if the error was on a write (1) or read (0) cycle"]
pub type CodecHsDecErrWriteR = crate::BitReader;
#[doc = "Field `CODEC_HS_DEC_ERR_SRC` reader - Source of the decode error"]
pub type CodecHsDecErrSrcR = crate::FieldReader;
#[doc = "Field `CODEC_HS_DEC_ERR_ID` reader - ID of the master that caused the decode error"]
pub type CodecHsDecErrIdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear decode error status"]
    #[inline(always)]
    pub fn reg_dec_err_clr(&self) -> RegDecErrClrR {
        RegDecErrClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latch only the last decode error (write 1 to latch)"]
    #[inline(always)]
    pub fn reg_dec_err_latch_last(&self) -> RegDecErrLatchLastR {
        RegDecErrLatchLastR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Decode error latched status"]
    #[inline(always)]
    pub fn codec_hs_dec_err_lat(&self) -> CodecHsDecErrLatR {
        CodecHsDecErrLatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if the error was on a write (1) or read (0) cycle"]
    #[inline(always)]
    pub fn codec_hs_dec_err_write(&self) -> CodecHsDecErrWriteR {
        CodecHsDecErrWriteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Source of the decode error"]
    #[inline(always)]
    pub fn codec_hs_dec_err_src(&self) -> CodecHsDecErrSrcR {
        CodecHsDecErrSrcR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - ID of the master that caused the decode error"]
    #[inline(always)]
    pub fn codec_hs_dec_err_id(&self) -> CodecHsDecErrIdR {
        CodecHsDecErrIdR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear decode error status"]
    #[inline(always)]
    pub fn reg_dec_err_clr(&mut self) -> RegDecErrClrW<'_, CodecBusDecErrSpec> {
        RegDecErrClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Latch only the last decode error (write 1 to latch)"]
    #[inline(always)]
    pub fn reg_dec_err_latch_last(&mut self) -> RegDecErrLatchLastW<'_, CodecBusDecErrSpec> {
        RegDecErrLatchLastW::new(self, 1)
    }
}
#[doc = "Codec Bus Decode Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_dec_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_bus_dec_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecBusDecErrSpec;
impl crate::RegisterSpec for CodecBusDecErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_bus_dec_err::R`](R) reader structure"]
impl crate::Readable for CodecBusDecErrSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_bus_dec_err::W`](W) writer structure"]
impl crate::Writable for CodecBusDecErrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEC_BUS_DEC_ERR to value 0"]
impl crate::Resettable for CodecBusDecErrSpec {}
