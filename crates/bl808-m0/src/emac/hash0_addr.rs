#[doc = "Register `HASH0_ADDR` reader"]
pub type R = crate::R<Hash0AddrSpec>;
#[doc = "Register `HASH0_ADDR` writer"]
pub type W = crate::W<Hash0AddrSpec>;
#[doc = "Field `HASH0` reader - "]
pub type Hash0R = crate::FieldReader<u32>;
#[doc = "Field `HASH0` writer - "]
pub type Hash0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash0(&self) -> Hash0R {
        Hash0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash0(&mut self) -> Hash0W<'_, Hash0AddrSpec> {
        Hash0W::new(self, 0)
    }
}
#[doc = "Hash Table Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hash0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hash0AddrSpec;
impl crate::RegisterSpec for Hash0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash0_addr::R`](R) reader structure"]
impl crate::Readable for Hash0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hash0_addr::W`](W) writer structure"]
impl crate::Writable for Hash0AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH0_ADDR to value 0"]
impl crate::Resettable for Hash0AddrSpec {}
