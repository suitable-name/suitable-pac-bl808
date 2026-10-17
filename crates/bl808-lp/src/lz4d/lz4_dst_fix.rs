#[doc = "Register `LZ4_DST_FIX` reader"]
pub type R = crate::R<Lz4DstFixSpec>;
#[doc = "Register `LZ4_DST_FIX` writer"]
pub type W = crate::W<Lz4DstFixSpec>;
#[doc = "Field `lz4_dst_fix` reader - "]
pub type Lz4DstFixR = crate::FieldReader<u16>;
#[doc = "Field `lz4_dst_fix` writer - "]
pub type Lz4DstFixW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 12:25"]
    #[inline(always)]
    pub fn lz4_dst_fix(&self) -> Lz4DstFixR {
        Lz4DstFixR::new(((self.bits >> 12) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:25"]
    #[inline(always)]
    pub fn lz4_dst_fix(&mut self) -> Lz4DstFixW<'_, Lz4DstFixSpec> {
        Lz4DstFixW::new(self, 12)
    }
}
#[doc = "Destination Address Fix Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_dst_fix::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_dst_fix::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4DstFixSpec;
impl crate::RegisterSpec for Lz4DstFixSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_dst_fix::R`](R) reader structure"]
impl crate::Readable for Lz4DstFixSpec {}
#[doc = "`write(|w| ..)` method takes [`lz4_dst_fix::W`](W) writer structure"]
impl crate::Writable for Lz4DstFixSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LZ4_DST_FIX to value 0"]
impl crate::Resettable for Lz4DstFixSpec {}
