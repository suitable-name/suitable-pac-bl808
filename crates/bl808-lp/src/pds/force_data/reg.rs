#[doc = "Register `REG` reader"]
pub type R = crate::R<RegSpec>;
#[doc = "Register `REG` writer"]
pub type W = crate::W<RegSpec>;
#[doc = "Field `even` reader - "]
pub type EvenR = crate::FieldReader<u16>;
#[doc = "Field `even` writer - "]
pub type EvenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `odd` reader - "]
pub type OddR = crate::FieldReader<u16>;
#[doc = "Field `odd` writer - "]
pub type OddW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn even(&self) -> EvenR {
        EvenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn even(&mut self) -> EvenW<'_, RegSpec> {
        EvenW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn odd(&mut self) -> OddW<'_, RegSpec> {
        OddW::new(self, 16)
    }
}
#[doc = "Touch Force Data for two adjacent channels\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpec;
impl crate::RegisterSpec for RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg::R`](R) reader structure"]
impl crate::Readable for RegSpec {}
#[doc = "`write(|w| ..)` method takes [`reg::W`](W) writer structure"]
impl crate::Writable for RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG to value 0x0400_0400"]
impl crate::Resettable for RegSpec {
    const RESET_VALUE: u32 = 0x0400_0400;
}
