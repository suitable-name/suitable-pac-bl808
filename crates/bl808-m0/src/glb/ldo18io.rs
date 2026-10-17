#[doc = "Register `LDO18IO` reader"]
pub type R = crate::R<Ldo18ioSpec>;
#[doc = "Register `LDO18IO` writer"]
pub type W = crate::W<Ldo18ioSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LDO 1.8V IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo18io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo18io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo18ioSpec;
impl crate::RegisterSpec for Ldo18ioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo18io::R`](R) reader structure"]
impl crate::Readable for Ldo18ioSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo18io::W`](W) writer structure"]
impl crate::Writable for Ldo18ioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO18IO to value 0"]
impl crate::Resettable for Ldo18ioSpec {}
