#[doc = "Register `MJPEG_HEADER_BYTE` reader"]
pub type R = crate::R<MjpegHeaderByteSpec>;
#[doc = "Register `MJPEG_HEADER_BYTE` writer"]
pub type W = crate::W<MjpegHeaderByteSpec>;
#[doc = "Field `reg_head_byte` reader - Header byte count for JPEG start."]
pub type RegHeadByteR = crate::FieldReader<u16>;
#[doc = "Field `reg_head_byte` writer - Header byte count for JPEG start."]
pub type RegHeadByteW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_tail_exp` reader - Tail expansion enable."]
pub type RegTailExpR = crate::BitReader;
#[doc = "Field `reg_tail_exp` writer - Tail expansion enable."]
pub type RegTailExpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_y0_order` reader - Y0 data fetch order."]
pub type RegY0OrderR = crate::FieldReader;
#[doc = "Field `reg_y0_order` writer - Y0 data fetch order."]
pub type RegY0OrderW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_u0_order` reader - U0 data fetch order."]
pub type RegU0OrderR = crate::FieldReader;
#[doc = "Field `reg_u0_order` writer - U0 data fetch order."]
pub type RegU0OrderW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_y1_order` reader - Y1 data fetch order."]
pub type RegY1OrderR = crate::FieldReader;
#[doc = "Field `reg_y1_order` writer - Y1 data fetch order."]
pub type RegY1OrderW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_v0_order` reader - V0 data fetch order."]
pub type RegV0OrderR = crate::FieldReader;
#[doc = "Field `reg_v0_order` writer - V0 data fetch order."]
pub type RegV0OrderW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Header byte count for JPEG start."]
    #[inline(always)]
    pub fn reg_head_byte(&self) -> RegHeadByteR {
        RegHeadByteR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Tail expansion enable."]
    #[inline(always)]
    pub fn reg_tail_exp(&self) -> RegTailExpR {
        RegTailExpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Y0 data fetch order."]
    #[inline(always)]
    pub fn reg_y0_order(&self) -> RegY0OrderR {
        RegY0OrderR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - U0 data fetch order."]
    #[inline(always)]
    pub fn reg_u0_order(&self) -> RegU0OrderR {
        RegU0OrderR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Y1 data fetch order."]
    #[inline(always)]
    pub fn reg_y1_order(&self) -> RegY1OrderR {
        RegY1OrderR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - V0 data fetch order."]
    #[inline(always)]
    pub fn reg_v0_order(&self) -> RegV0OrderR {
        RegV0OrderR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Header byte count for JPEG start."]
    #[inline(always)]
    pub fn reg_head_byte(&mut self) -> RegHeadByteW<'_, MjpegHeaderByteSpec> {
        RegHeadByteW::new(self, 0)
    }
    #[doc = "Bit 16 - Tail expansion enable."]
    #[inline(always)]
    pub fn reg_tail_exp(&mut self) -> RegTailExpW<'_, MjpegHeaderByteSpec> {
        RegTailExpW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Y0 data fetch order."]
    #[inline(always)]
    pub fn reg_y0_order(&mut self) -> RegY0OrderW<'_, MjpegHeaderByteSpec> {
        RegY0OrderW::new(self, 24)
    }
    #[doc = "Bits 26:27 - U0 data fetch order."]
    #[inline(always)]
    pub fn reg_u0_order(&mut self) -> RegU0OrderW<'_, MjpegHeaderByteSpec> {
        RegU0OrderW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Y1 data fetch order."]
    #[inline(always)]
    pub fn reg_y1_order(&mut self) -> RegY1OrderW<'_, MjpegHeaderByteSpec> {
        RegY1OrderW::new(self, 28)
    }
    #[doc = "Bits 30:31 - V0 data fetch order."]
    #[inline(always)]
    pub fn reg_v0_order(&mut self) -> RegV0OrderW<'_, MjpegHeaderByteSpec> {
        RegV0OrderW::new(self, 30)
    }
}
#[doc = "MJPEG Header Byte Count and Component Order\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_header_byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_header_byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegHeaderByteSpec;
impl crate::RegisterSpec for MjpegHeaderByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_header_byte::R`](R) reader structure"]
impl crate::Readable for MjpegHeaderByteSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_header_byte::W`](W) writer structure"]
impl crate::Writable for MjpegHeaderByteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_HEADER_BYTE to value 0xe400_0000"]
impl crate::Resettable for MjpegHeaderByteSpec {
    const RESET_VALUE: u32 = 0xe400_0000;
}
