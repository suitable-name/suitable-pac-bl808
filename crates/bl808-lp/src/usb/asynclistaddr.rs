#[doc = "Register `ASYNCLISTADDR` reader"]
pub type R = crate::R<AsynclistaddrSpec>;
#[doc = "Register `ASYNCLISTADDR` writer"]
pub type W = crate::W<AsynclistaddrSpec>;
#[doc = "Field `ASYNC_LADR` reader - "]
pub type AsyncLadrR = crate::FieldReader<u32>;
#[doc = "Field `ASYNC_LADR` writer - "]
pub type AsyncLadrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn async_ladr(&self) -> AsyncLadrR {
        AsyncLadrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn async_ladr(&mut self) -> AsyncLadrW<'_, AsynclistaddrSpec> {
        AsyncLadrW::new(self, 5)
    }
}
#[doc = "Asynchronous List Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`asynclistaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynclistaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynclistaddrSpec;
impl crate::RegisterSpec for AsynclistaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asynclistaddr::R`](R) reader structure"]
impl crate::Readable for AsynclistaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`asynclistaddr::W`](W) writer structure"]
impl crate::Writable for AsynclistaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASYNCLISTADDR to value 0"]
impl crate::Resettable for AsynclistaddrSpec {}
