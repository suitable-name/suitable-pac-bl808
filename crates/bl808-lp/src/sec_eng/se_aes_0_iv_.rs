#[doc = "Register `SE_AES_0_IV_%s` reader"]
pub type R = crate::R<SeAes0Iv_Spec>;
#[doc = "Register `SE_AES_0_IV_%s` writer"]
pub type W = crate::W<SeAes0Iv_Spec>;
#[doc = "Field `se_aes_0_iv` reader - "]
pub type SeAes0IvR = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_iv` writer - "]
pub type SeAes0IvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_iv(&self) -> SeAes0IvR {
        SeAes0IvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_iv(&mut self) -> SeAes0IvW<'_, SeAes0Iv_Spec> {
        SeAes0IvW::new(self, 0)
    }
}
#[doc = "AES IV Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_iv_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_iv_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0Iv_Spec;
impl crate::RegisterSpec for SeAes0Iv_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_iv_::R`](R) reader structure"]
impl crate::Readable for SeAes0Iv_Spec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_iv_::W`](W) writer structure"]
impl crate::Writable for SeAes0Iv_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_IV_%s to value 0"]
impl crate::Resettable for SeAes0Iv_Spec {}
