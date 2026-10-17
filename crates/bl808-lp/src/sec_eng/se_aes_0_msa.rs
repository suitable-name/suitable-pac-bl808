#[doc = "Register `SE_AES_0_MSA` reader"]
pub type R = crate::R<SeAes0MsaSpec>;
#[doc = "Register `SE_AES_0_MSA` writer"]
pub type W = crate::W<SeAes0MsaSpec>;
#[doc = "Field `se_aes_0_msa` reader - "]
pub type SeAes0MsaR = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_msa` writer - "]
pub type SeAes0MsaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_msa(&self) -> SeAes0MsaR {
        SeAes0MsaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_msa(&mut self) -> SeAes0MsaW<'_, SeAes0MsaSpec> {
        SeAes0MsaW::new(self, 0)
    }
}
#[doc = "AES Message Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_msa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_msa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0MsaSpec;
impl crate::RegisterSpec for SeAes0MsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_msa::R`](R) reader structure"]
impl crate::Readable for SeAes0MsaSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_msa::W`](W) writer structure"]
impl crate::Writable for SeAes0MsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_MSA to value 0"]
impl crate::Resettable for SeAes0MsaSpec {}
