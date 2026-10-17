#[doc = "Register `LLI` reader"]
pub type R = crate::R<LliSpec>;
#[doc = "Register `LLI` writer"]
pub type W = crate::W<LliSpec>;
#[doc = "Field `LLI_EN` reader - Linked List Item enable"]
pub type LliEnR = crate::BitReader;
#[doc = "Field `LLI_EN` writer - Linked List Item enable"]
pub type LliEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLI_ADDR` reader - Next LLI descriptor address (word-aligned)"]
pub type LliAddrR = crate::FieldReader<u32>;
#[doc = "Field `LLI_ADDR` writer - Next LLI descriptor address (word-aligned)"]
pub type LliAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Linked List Item enable"]
    #[inline(always)]
    pub fn lli_en(&self) -> LliEnR {
        LliEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - Next LLI descriptor address (word-aligned)"]
    #[inline(always)]
    pub fn lli_addr(&self) -> LliAddrR {
        LliAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Linked List Item enable"]
    #[inline(always)]
    pub fn lli_en(&mut self) -> LliEnW<'_, LliSpec> {
        LliEnW::new(self, 0)
    }
    #[doc = "Bits 2:31 - Next LLI descriptor address (word-aligned)"]
    #[inline(always)]
    pub fn lli_addr(&mut self) -> LliAddrW<'_, LliSpec> {
        LliAddrW::new(self, 2)
    }
}
#[doc = "Next LLI Address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
