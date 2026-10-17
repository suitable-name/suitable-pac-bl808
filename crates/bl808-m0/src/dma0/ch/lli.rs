#[doc = "Register `LLI` reader"]
pub type R = crate::R<LliSpec>;
#[doc = "Register `LLI` writer"]
pub type W = crate::W<LliSpec>;
#[doc = "Field `LLI` reader - "]
pub type LliR = crate::FieldReader<u32>;
#[doc = "Field `LLI` writer - "]
pub type LliW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&self) -> LliR {
        LliR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&mut self) -> LliW<'_, LliSpec> {
        LliW::new(self, 0)
    }
}
#[doc = "Next linked list item address\n\nYou can [`read`](crate::Reg::read) this register and get [`lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LliSpec;
impl crate::RegisterSpec for LliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lli::R`](R) reader structure"]
impl crate::Readable for LliSpec {}
#[doc = "`write(|w| ..)` method takes [`lli::W`](W) writer structure"]
impl crate::Writable for LliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLI to value 0"]
impl crate::Resettable for LliSpec {}
