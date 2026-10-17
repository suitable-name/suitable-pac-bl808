#[doc = "Register `SF_RESERVED` reader"]
pub type R = crate::R<SfReservedSpec>;
#[doc = "Register `SF_RESERVED` writer"]
pub type W = crate::W<SfReservedSpec>;
#[doc = "Field `sf_reserved` reader - "]
pub type SfReservedR = crate::FieldReader<u32>;
#[doc = "Field `sf_reserved` writer - "]
pub type SfReservedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_reserved(&self) -> SfReservedR {
        SfReservedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_reserved(&mut self) -> SfReservedW<'_, SfReservedSpec> {
        SfReservedW::new(self, 0)
    }
}
#[doc = "SF Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfReservedSpec;
impl crate::RegisterSpec for SfReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_reserved::R`](R) reader structure"]
impl crate::Readable for SfReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_reserved::W`](W) writer structure"]
impl crate::Writable for SfReservedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_RESERVED to value 0"]
impl crate::Resettable for SfReservedSpec {}
