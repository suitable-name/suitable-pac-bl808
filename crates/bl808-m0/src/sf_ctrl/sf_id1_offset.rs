#[doc = "Register `SF_ID1_OFFSET` reader"]
pub type R = crate::R<SfId1OffsetSpec>;
#[doc = "Register `SF_ID1_OFFSET` writer"]
pub type W = crate::W<SfId1OffsetSpec>;
#[doc = "Field `sf_id1_offset` reader - "]
pub type SfId1OffsetR = crate::FieldReader<u32>;
#[doc = "Field `sf_id1_offset` writer - "]
pub type SfId1OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sf_id1_offset(&self) -> SfId1OffsetR {
        SfId1OffsetR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sf_id1_offset(&mut self) -> SfId1OffsetW<'_, SfId1OffsetSpec> {
        SfId1OffsetW::new(self, 0)
    }
}
#[doc = "Remap Offset for Region 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_id1_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_id1_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfId1OffsetSpec;
impl crate::RegisterSpec for SfId1OffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_id1_offset::R`](R) reader structure"]
impl crate::Readable for SfId1OffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_id1_offset::W`](W) writer structure"]
impl crate::Writable for SfId1OffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_ID1_OFFSET to value 0"]
impl crate::Resettable for SfId1OffsetSpec {}
