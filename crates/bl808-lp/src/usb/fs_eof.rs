#[doc = "Register `FS_EOF` reader"]
pub type R = crate::R<FsEofSpec>;
#[doc = "Register `FS_EOF` writer"]
pub type W = crate::W<FsEofSpec>;
#[doc = "Field `FS_EOF1_TIME` reader - "]
pub type FsEof1TimeR = crate::FieldReader<u16>;
#[doc = "Field `FS_EOF1_TIME` writer - "]
pub type FsEof1TimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FS_EOF1_TIME_125US` reader - "]
pub type FsEof1Time125usR = crate::FieldReader;
#[doc = "Field `FS_EOF1_TIME_125US` writer - "]
pub type FsEof1Time125usW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITDDMASM` reader - Isochronous Transfer Descriptor DMA State Machine Status"]
pub type ItddmasmR = crate::FieldReader;
#[doc = "Field `ITDTRAVSM` reader - Isochronous Transfer Descriptor Traversal State Machine Status"]
pub type ItdtravsmR = crate::FieldReader;
#[doc = "Field `DMADMSM` reader - DMA Administration State Machine Status"]
pub type DmadmsmR = crate::FieldReader;
#[doc = "Field `QHTTRSM` reader - Queue Head Traversal State Machine Status"]
pub type QhttrsmR = crate::FieldReader;
#[doc = "Field `QHTRAVSM` reader - Queue Head Traversal State Machine Status"]
pub type QhtravsmR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn fs_eof1_time(&self) -> FsEof1TimeR {
        FsEof1TimeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn fs_eof1_time_125us(&self) -> FsEof1Time125usR {
        FsEof1Time125usR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Isochronous Transfer Descriptor DMA State Machine Status"]
    #[inline(always)]
    pub fn itddmasm(&self) -> ItddmasmR {
        ItddmasmR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Isochronous Transfer Descriptor Traversal State Machine Status"]
    #[inline(always)]
    pub fn itdtravsm(&self) -> ItdtravsmR {
        ItdtravsmR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:25 - DMA Administration State Machine Status"]
    #[inline(always)]
    pub fn dmadmsm(&self) -> DmadmsmR {
        DmadmsmR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:28 - Queue Head Traversal State Machine Status"]
    #[inline(always)]
    pub fn qhttrsm(&self) -> QhttrsmR {
        QhttrsmR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Queue Head Traversal State Machine Status"]
    #[inline(always)]
    pub fn qhtravsm(&self) -> QhtravsmR {
        QhtravsmR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn fs_eof1_time(&mut self) -> FsEof1TimeW<'_, FsEofSpec> {
        FsEof1TimeW::new(self, 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn fs_eof1_time_125us(&mut self) -> FsEof1Time125usW<'_, FsEofSpec> {
        FsEof1Time125usW::new(self, 12)
    }
}
#[doc = "Full Speed EOF Control and Host SM Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_eof::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_eof::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsEofSpec;
impl crate::RegisterSpec for FsEofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_eof::R`](R) reader structure"]
impl crate::Readable for FsEofSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_eof::W`](W) writer structure"]
impl crate::Writable for FsEofSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_EOF to value 0"]
impl crate::Resettable for FsEofSpec {}
