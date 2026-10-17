#[doc = "Register `LZ4_INT_EN` reader"]
pub type R = crate::R<Lz4IntEnSpec>;
#[doc = "Register `LZ4_INT_EN` writer"]
pub type W = crate::W<Lz4IntEnSpec>;
#[doc = "Field `lz4_done_en` reader - Enable decompression done interrupt"]
pub type Lz4DoneEnR = crate::BitReader;
#[doc = "Field `lz4_done_en` writer - Enable decompression done interrupt"]
pub type Lz4DoneEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lz4_err_en` reader - Enable decompression error interrupt"]
pub type Lz4ErrEnR = crate::BitReader;
#[doc = "Field `lz4_err_en` writer - Enable decompression error interrupt"]
pub type Lz4ErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lz4_dst_int_en` reader - Destination-specific interrupt enables"]
pub type Lz4DstIntEnR = crate::FieldReader;
#[doc = "Field `lz4_dst_int_en` writer - Destination-specific interrupt enables"]
pub type Lz4DstIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `lz4_dst_int_inv` reader - Destination-specific interrupt inversion"]
pub type Lz4DstIntInvR = crate::FieldReader;
#[doc = "Field `lz4_dst_int_inv` writer - Destination-specific interrupt inversion"]
pub type Lz4DstIntInvW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Enable decompression done interrupt"]
    #[inline(always)]
    pub fn lz4_done_en(&self) -> Lz4DoneEnR {
        Lz4DoneEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable decompression error interrupt"]
    #[inline(always)]
    pub fn lz4_err_en(&self) -> Lz4ErrEnR {
        Lz4ErrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Destination-specific interrupt enables"]
    #[inline(always)]
    pub fn lz4_dst_int_en(&self) -> Lz4DstIntEnR {
        Lz4DstIntEnR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - Destination-specific interrupt inversion"]
    #[inline(always)]
    pub fn lz4_dst_int_inv(&self) -> Lz4DstIntInvR {
        Lz4DstIntInvR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable decompression done interrupt"]
    #[inline(always)]
    pub fn lz4_done_en(&mut self) -> Lz4DoneEnW<'_, Lz4IntEnSpec> {
        Lz4DoneEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable decompression error interrupt"]
    #[inline(always)]
    pub fn lz4_err_en(&mut self) -> Lz4ErrEnW<'_, Lz4IntEnSpec> {
        Lz4ErrEnW::new(self, 1)
    }
    #[doc = "Bits 10:15 - Destination-specific interrupt enables"]
    #[inline(always)]
    pub fn lz4_dst_int_en(&mut self) -> Lz4DstIntEnW<'_, Lz4IntEnSpec> {
        Lz4DstIntEnW::new(self, 10)
    }
    #[doc = "Bits 26:31 - Destination-specific interrupt inversion"]
    #[inline(always)]
    pub fn lz4_dst_int_inv(&mut self) -> Lz4DstIntInvW<'_, Lz4IntEnSpec> {
        Lz4DstIntInvW::new(self, 26)
    }
}
#[doc = "LZ4 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4IntEnSpec;
impl crate::RegisterSpec for Lz4IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_int_en::R`](R) reader structure"]
impl crate::Readable for Lz4IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`lz4_int_en::W`](W) writer structure"]
impl crate::Writable for Lz4IntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LZ4_INT_EN to value 0x03"]
impl crate::Resettable for Lz4IntEnSpec {
    const RESET_VALUE: u32 = 0x03;
}
