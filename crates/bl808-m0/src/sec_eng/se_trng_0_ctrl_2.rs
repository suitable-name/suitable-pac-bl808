#[doc = "Register `SE_TRNG_0_CTRL_2` reader"]
pub type R = crate::R<SeTrng0Ctrl2Spec>;
#[doc = "Register `SE_TRNG_0_CTRL_2` writer"]
pub type W = crate::W<SeTrng0Ctrl2Spec>;
#[doc = "Field `se_trng_0_reseed_n_msb` reader - "]
pub type SeTrng0ReseedNMsbR = crate::FieldReader<u16>;
#[doc = "Field `se_trng_0_reseed_n_msb` writer - "]
pub type SeTrng0ReseedNMsbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_msb(&self) -> SeTrng0ReseedNMsbR {
        SeTrng0ReseedNMsbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_msb(&mut self) -> SeTrng0ReseedNMsbW<'_, SeTrng0Ctrl2Spec> {
        SeTrng0ReseedNMsbW::new(self, 0)
    }
}
#[doc = "TRNG Reseed Parameter MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Ctrl2Spec;
impl crate::RegisterSpec for SeTrng0Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_ctrl_2::R`](R) reader structure"]
impl crate::Readable for SeTrng0Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_ctrl_2::W`](W) writer structure"]
impl crate::Writable for SeTrng0Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_TRNG_0_CTRL_2 to value 0xff"]
impl crate::Resettable for SeTrng0Ctrl2Spec {
    const RESET_VALUE: u32 = 0xff;
}
