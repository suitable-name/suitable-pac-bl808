#[doc = "Register `SE_AES_1_KEY_SEL` reader"]
pub type R = crate::R<SeAes1KeySelSpec>;
#[doc = "Register `SE_AES_1_KEY_SEL` writer"]
pub type W = crate::W<SeAes1KeySelSpec>;
#[doc = "Field `se_aes_1_key_sel` reader - "]
pub type SeAes1KeySelR = crate::FieldReader;
#[doc = "Field `se_aes_1_key_sel` writer - "]
pub type SeAes1KeySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_1_key_sel(&self) -> SeAes1KeySelR {
        SeAes1KeySelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_1_key_sel(&mut self) -> SeAes1KeySelW<'_, SeAes1KeySelSpec> {
        SeAes1KeySelW::new(self, 0)
    }
}
#[doc = "AES-1 HW Key Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_1_key_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_1_key_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes1KeySelSpec;
impl crate::RegisterSpec for SeAes1KeySelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_1_key_sel::R`](R) reader structure"]
impl crate::Readable for SeAes1KeySelSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_1_key_sel::W`](W) writer structure"]
impl crate::Writable for SeAes1KeySelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_1_KEY_SEL to value 0"]
impl crate::Resettable for SeAes1KeySelSpec {}
