#[doc = "Register `SE_AES_0_LINK` reader"]
pub type R = crate::R<SeAes0LinkSpec>;
#[doc = "Register `SE_AES_0_LINK` writer"]
pub type W = crate::W<SeAes0LinkSpec>;
#[doc = "Field `se_aes_0_lca` reader - "]
pub type SeAes0LcaR = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_lca` writer - "]
pub type SeAes0LcaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_lca(&self) -> SeAes0LcaR {
        SeAes0LcaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_lca(&mut self) -> SeAes0LcaW<'_, SeAes0LinkSpec> {
        SeAes0LcaW::new(self, 0)
    }
}
#[doc = "AES Link List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0LinkSpec;
impl crate::RegisterSpec for SeAes0LinkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_link::R`](R) reader structure"]
impl crate::Readable for SeAes0LinkSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_link::W`](W) writer structure"]
impl crate::Writable for SeAes0LinkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_LINK to value 0"]
impl crate::Resettable for SeAes0LinkSpec {}
