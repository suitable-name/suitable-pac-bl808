#[doc = "Register `SE_AES_0_MDA` reader"]
pub type R = crate::R<SeAes0MdaSpec>;
#[doc = "Register `SE_AES_0_MDA` writer"]
pub type W = crate::W<SeAes0MdaSpec>;
#[doc = "Field `se_aes_0_mda` reader - "]
pub type SeAes0MdaR = crate::FieldReader<u32>;
#[doc = "Field `se_aes_0_mda` writer - "]
pub type SeAes0MdaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_mda(&self) -> SeAes0MdaR {
        SeAes0MdaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_mda(&mut self) -> SeAes0MdaW<'_, SeAes0MdaSpec> {
        SeAes0MdaW::new(self, 0)
    }
}
#[doc = "AES Message Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_mda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_mda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0MdaSpec;
impl crate::RegisterSpec for SeAes0MdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_mda::R`](R) reader structure"]
impl crate::Readable for SeAes0MdaSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_mda::W`](W) writer structure"]
impl crate::Writable for SeAes0MdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_MDA to value 0"]
impl crate::Resettable for SeAes0MdaSpec {}
