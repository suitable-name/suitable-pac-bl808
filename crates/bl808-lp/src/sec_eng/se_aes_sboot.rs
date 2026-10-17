#[doc = "Register `SE_AES_SBOOT` reader"]
pub type R = crate::R<SeAesSbootSpec>;
#[doc = "Register `SE_AES_SBOOT` writer"]
pub type W = crate::W<SeAesSbootSpec>;
#[doc = "Field `se_aes_sboot_key_sel` reader - "]
pub type SeAesSbootKeySelR = crate::BitReader;
#[doc = "Field `se_aes_sboot_key_sel` writer - "]
pub type SeAesSbootKeySelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_xts_mode` reader - "]
pub type SeAes0XtsModeR = crate::BitReader;
#[doc = "Field `se_aes_0_xts_mode` writer - "]
pub type SeAes0XtsModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_uni_len` reader - "]
pub type SeAes0UniLenR = crate::FieldReader<u16>;
#[doc = "Field `se_aes_0_uni_len` writer - "]
pub type SeAes0UniLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_sboot_key_sel(&self) -> SeAesSbootKeySelR {
        SeAesSbootKeySelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_xts_mode(&self) -> SeAes0XtsModeR {
        SeAes0XtsModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_uni_len(&self) -> SeAes0UniLenR {
        SeAes0UniLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_sboot_key_sel(&mut self) -> SeAesSbootKeySelW<'_, SeAesSbootSpec> {
        SeAesSbootKeySelW::new(self, 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_xts_mode(&mut self) -> SeAes0XtsModeW<'_, SeAesSbootSpec> {
        SeAes0XtsModeW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_uni_len(&mut self) -> SeAes0UniLenW<'_, SeAesSbootSpec> {
        SeAes0UniLenW::new(self, 16)
    }
}
#[doc = "AES Secure Boot Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_sboot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_sboot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAesSbootSpec;
impl crate::RegisterSpec for SeAesSbootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_sboot::R`](R) reader structure"]
impl crate::Readable for SeAesSbootSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_sboot::W`](W) writer structure"]
impl crate::Writable for SeAesSbootSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_SBOOT to value 0x0002_0000"]
impl crate::Resettable for SeAesSbootSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
