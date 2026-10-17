#[doc = "Register `HS_EOF` reader"]
pub type R = crate::R<HsEofSpec>;
#[doc = "Register `HS_EOF` writer"]
pub type W = crate::W<HsEofSpec>;
#[doc = "Field `HS_EOF1_TIME` reader - "]
pub type HsEof1TimeR = crate::FieldReader<u16>;
#[doc = "Field `HS_EOF1_TIME` writer - "]
pub type HsEof1TimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SITDDMASM` reader - Split Isochronous Transfer Descriptor DMA State Machine Status"]
pub type SitddmasmR = crate::FieldReader;
#[doc = "Field `SITDTRAVSM` reader - Split Isochronous Transfer Descriptor Traversal State Machine Status"]
pub type SitdtravsmR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn hs_eof1_time(&self) -> HsEof1TimeR {
        HsEof1TimeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Split Isochronous Transfer Descriptor DMA State Machine Status"]
    #[inline(always)]
    pub fn sitddmasm(&self) -> SitddmasmR {
        SitddmasmR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Split Isochronous Transfer Descriptor Traversal State Machine Status"]
    #[inline(always)]
    pub fn sitdtravsm(&self) -> SitdtravsmR {
        SitdtravsmR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn hs_eof1_time(&mut self) -> HsEof1TimeW<'_, HsEofSpec> {
        HsEof1TimeW::new(self, 0)
    }
}
#[doc = "High Speed EOF Control and Host SM Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hs_eof::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hs_eof::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsEofSpec;
impl crate::RegisterSpec for HsEofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_eof::R`](R) reader structure"]
impl crate::Readable for HsEofSpec {}
#[doc = "`write(|w| ..)` method takes [`hs_eof::W`](W) writer structure"]
impl crate::Writable for HsEofSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HS_EOF to value 0"]
impl crate::Resettable for HsEofSpec {}
