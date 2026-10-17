#[doc = "Register `LZ4_DST_START` reader"]
pub type R = crate::R<Lz4DstStartSpec>;
#[doc = "Register `LZ4_DST_START` writer"]
pub type W = crate::W<Lz4DstStartSpec>;
#[doc = "Field `lz4_dst_start` reader - Destination start offset address"]
pub type Lz4DstStartR = crate::FieldReader<u32>;
#[doc = "Field `lz4_dst_start` writer - Destination start offset address"]
pub type Lz4DstStartW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `lz4_dst_base` reader - Destination base address pointer"]
pub type Lz4DstBaseR = crate::FieldReader;
#[doc = "Field `lz4_dst_base` writer - Destination base address pointer"]
pub type Lz4DstBaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:25 - Destination start offset address"]
    #[inline(always)]
    pub fn lz4_dst_start(&self) -> Lz4DstStartR {
        Lz4DstStartR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31 - Destination base address pointer"]
    #[inline(always)]
    pub fn lz4_dst_base(&self) -> Lz4DstBaseR {
        Lz4DstBaseR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:25 - Destination start offset address"]
    #[inline(always)]
    pub fn lz4_dst_start(&mut self) -> Lz4DstStartW<'_, Lz4DstStartSpec> {
        Lz4DstStartW::new(self, 0)
    }
    #[doc = "Bits 26:31 - Destination base address pointer"]
    #[inline(always)]
    pub fn lz4_dst_base(&mut self) -> Lz4DstBaseW<'_, Lz4DstStartSpec> {
        Lz4DstBaseW::new(self, 26)
    }
}
#[doc = "Destination Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_dst_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_dst_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4DstStartSpec;
impl crate::RegisterSpec for Lz4DstStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_dst_start::R`](R) reader structure"]
impl crate::Readable for Lz4DstStartSpec {}
#[doc = "`write(|w| ..)` method takes [`lz4_dst_start::W`](W) writer structure"]
impl crate::Writable for Lz4DstStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LZ4_DST_START to value 0"]
impl crate::Resettable for Lz4DstStartSpec {}
