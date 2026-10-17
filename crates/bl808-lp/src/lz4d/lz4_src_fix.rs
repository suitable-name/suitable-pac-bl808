#[doc = "Register `LZ4_SRC_FIX` reader"]
pub type R = crate::R<Lz4SrcFixSpec>;
#[doc = "Register `LZ4_SRC_FIX` writer"]
pub type W = crate::W<Lz4SrcFixSpec>;
#[doc = "Field `lz4_src_fix` reader - "]
pub type Lz4SrcFixR = crate::FieldReader<u16>;
#[doc = "Field `lz4_src_fix` writer - "]
pub type Lz4SrcFixW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 12:25"]
    #[inline(always)]
    pub fn lz4_src_fix(&self) -> Lz4SrcFixR {
        Lz4SrcFixR::new(((self.bits >> 12) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:25"]
    #[inline(always)]
    pub fn lz4_src_fix(&mut self) -> Lz4SrcFixW<'_, Lz4SrcFixSpec> {
        Lz4SrcFixW::new(self, 12)
    }
}
#[doc = "Source Address Fix Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_src_fix::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_src_fix::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4SrcFixSpec;
impl crate::RegisterSpec for Lz4SrcFixSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_src_fix::R`](R) reader structure"]
impl crate::Readable for Lz4SrcFixSpec {}
#[doc = "`write(|w| ..)` method takes [`lz4_src_fix::W`](W) writer structure"]
impl crate::Writable for Lz4SrcFixSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LZ4_SRC_FIX to value 0"]
impl crate::Resettable for Lz4SrcFixSpec {}
