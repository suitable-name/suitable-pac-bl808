#[doc = "Register `SF_BK2_ID0_OFFSET` reader"]
pub type R = crate::R<SfBk2Id0OffsetSpec>;
#[doc = "Register `SF_BK2_ID0_OFFSET` writer"]
pub type W = crate::W<SfBk2Id0OffsetSpec>;
#[doc = "Field `sf_bk2_id0_offset` reader - "]
pub type SfBk2Id0OffsetR = crate::FieldReader<u32>;
#[doc = "Field `sf_bk2_id0_offset` writer - "]
pub type SfBk2Id0OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sf_bk2_id0_offset(&self) -> SfBk2Id0OffsetR {
        SfBk2Id0OffsetR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sf_bk2_id0_offset(&mut self) -> SfBk2Id0OffsetW<'_, SfBk2Id0OffsetSpec> {
        SfBk2Id0OffsetW::new(self, 0)
    }
}
#[doc = "Remap Offset for Bank 2 Region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_bk2_id0_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_bk2_id0_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfBk2Id0OffsetSpec;
impl crate::RegisterSpec for SfBk2Id0OffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_bk2_id0_offset::R`](R) reader structure"]
impl crate::Readable for SfBk2Id0OffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_bk2_id0_offset::W`](W) writer structure"]
impl crate::Writable for SfBk2Id0OffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_BK2_ID0_OFFSET to value 0"]
impl crate::Resettable for SfBk2Id0OffsetSpec {}
