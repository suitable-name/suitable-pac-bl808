#[doc = "Register `SE_TRNG_0_CTRL_1` reader"]
pub type R = crate::R<SeTrng0Ctrl1Spec>;
#[doc = "Register `SE_TRNG_0_CTRL_1` writer"]
pub type W = crate::W<SeTrng0Ctrl1Spec>;
#[doc = "Field `se_trng_0_reseed_n_lsb` reader - "]
pub type SeTrng0ReseedNLsbR = crate::FieldReader<u32>;
#[doc = "Field `se_trng_0_reseed_n_lsb` writer - "]
pub type SeTrng0ReseedNLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_lsb(&self) -> SeTrng0ReseedNLsbR {
        SeTrng0ReseedNLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_lsb(&mut self) -> SeTrng0ReseedNLsbW<'_, SeTrng0Ctrl1Spec> {
        SeTrng0ReseedNLsbW::new(self, 0)
    }
}
#[doc = "TRNG Reseed Parameter LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Ctrl1Spec;
impl crate::RegisterSpec for SeTrng0Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SeTrng0Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SeTrng0Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_TRNG_0_CTRL_1 to value 0xffff"]
impl crate::Resettable for SeTrng0Ctrl1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
