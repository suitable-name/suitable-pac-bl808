#[doc = "Register `HASH1_ADDR` reader"]
pub type R = crate::R<Hash1AddrSpec>;
#[doc = "Register `HASH1_ADDR` writer"]
pub type W = crate::W<Hash1AddrSpec>;
#[doc = "Field `HASH1` reader - "]
pub type Hash1R = crate::FieldReader<u32>;
#[doc = "Field `HASH1` writer - "]
pub type Hash1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash1(&self) -> Hash1R {
        Hash1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash1(&mut self) -> Hash1W<'_, Hash1AddrSpec> {
        Hash1W::new(self, 0)
    }
}
#[doc = "Hash Table Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hash1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hash1AddrSpec;
impl crate::RegisterSpec for Hash1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash1_addr::R`](R) reader structure"]
impl crate::Readable for Hash1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hash1_addr::W`](W) writer structure"]
impl crate::Writable for Hash1AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH1_ADDR to value 0"]
impl crate::Resettable for Hash1AddrSpec {}
