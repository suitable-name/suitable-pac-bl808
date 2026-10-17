#[doc = "Register `SF_ID0_OFFSET` reader"]
pub type R = crate::R<SfId0OffsetSpec>;
#[doc = "Register `SF_ID0_OFFSET` writer"]
pub type W = crate::W<SfId0OffsetSpec>;
#[doc = "Field `sf_id0_offset` reader - "]
pub type SfId0OffsetR = crate::FieldReader<u32>;
#[doc = "Field `sf_id0_offset` writer - "]
pub type SfId0OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sf_id0_offset(&self) -> SfId0OffsetR {
        SfId0OffsetR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sf_id0_offset(&mut self) -> SfId0OffsetW<'_, SfId0OffsetSpec> {
        SfId0OffsetW::new(self, 0)
    }
}
#[doc = "Remap Offset for Region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_id0_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_id0_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfId0OffsetSpec;
impl crate::RegisterSpec for SfId0OffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_id0_offset::R`](R) reader structure"]
impl crate::Readable for SfId0OffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_id0_offset::W`](W) writer structure"]
impl crate::Writable for SfId0OffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_ID0_OFFSET to value 0"]
impl crate::Resettable for SfId0OffsetSpec {}
