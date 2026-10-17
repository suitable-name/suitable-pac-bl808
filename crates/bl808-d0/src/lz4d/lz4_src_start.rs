#[doc = "Register `LZ4_SRC_START` reader"]
pub type R = crate::R<Lz4SrcStartSpec>;
#[doc = "Register `LZ4_SRC_START` writer"]
pub type W = crate::W<Lz4SrcStartSpec>;
#[doc = "Field `lz4_src_start` reader - Source start offset address"]
pub type Lz4SrcStartR = crate::FieldReader<u32>;
#[doc = "Field `lz4_src_start` writer - Source start offset address"]
pub type Lz4SrcStartW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `lz4_src_base` reader - Source base address pointer"]
pub type Lz4SrcBaseR = crate::FieldReader;
#[doc = "Field `lz4_src_base` writer - Source base address pointer"]
pub type Lz4SrcBaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:25 - Source start offset address"]
    #[inline(always)]
    pub fn lz4_src_start(&self) -> Lz4SrcStartR {
        Lz4SrcStartR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31 - Source base address pointer"]
    #[inline(always)]
    pub fn lz4_src_base(&self) -> Lz4SrcBaseR {
        Lz4SrcBaseR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:25 - Source start offset address"]
    #[inline(always)]
    pub fn lz4_src_start(&mut self) -> Lz4SrcStartW<'_, Lz4SrcStartSpec> {
        Lz4SrcStartW::new(self, 0)
    }
    #[doc = "Bits 26:31 - Source base address pointer"]
    #[inline(always)]
    pub fn lz4_src_base(&mut self) -> Lz4SrcBaseW<'_, Lz4SrcStartSpec> {
        Lz4SrcBaseW::new(self, 26)
    }
}
#[doc = "Source Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_src_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_src_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4SrcStartSpec;
impl crate::RegisterSpec for Lz4SrcStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_src_start::R`](R) reader structure"]
impl crate::Readable for Lz4SrcStartSpec {}
#[doc = "`write(|w| ..)` method takes [`lz4_src_start::W`](W) writer structure"]
impl crate::Writable for Lz4SrcStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LZ4_SRC_START to value 0"]
impl crate::Resettable for Lz4SrcStartSpec {}
