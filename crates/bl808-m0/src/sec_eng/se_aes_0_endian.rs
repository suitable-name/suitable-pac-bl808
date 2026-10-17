#[doc = "Register `SE_AES_0_ENDIAN` reader"]
pub type R = crate::R<SeAes0EndianSpec>;
#[doc = "Register `SE_AES_0_ENDIAN` writer"]
pub type W = crate::W<SeAes0EndianSpec>;
#[doc = "Field `se_aes_0_dout_endian` reader - "]
pub type SeAes0DoutEndianR = crate::BitReader;
#[doc = "Field `se_aes_0_dout_endian` writer - "]
pub type SeAes0DoutEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_din_endian` reader - "]
pub type SeAes0DinEndianR = crate::BitReader;
#[doc = "Field `se_aes_0_din_endian` writer - "]
pub type SeAes0DinEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_key_endian` reader - "]
pub type SeAes0KeyEndianR = crate::BitReader;
#[doc = "Field `se_aes_0_key_endian` writer - "]
pub type SeAes0KeyEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_iv_endian` reader - "]
pub type SeAes0IvEndianR = crate::BitReader;
#[doc = "Field `se_aes_0_iv_endian` writer - "]
pub type SeAes0IvEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_twk_endian` reader - "]
pub type SeAes0TwkEndianR = crate::BitReader;
#[doc = "Field `se_aes_0_twk_endian` writer - "]
pub type SeAes0TwkEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_ctr_len` reader - "]
pub type SeAes0CtrLenR = crate::FieldReader;
#[doc = "Field `se_aes_0_ctr_len` writer - "]
pub type SeAes0CtrLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&self) -> SeAes0DoutEndianR {
        SeAes0DoutEndianR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&self) -> SeAes0DinEndianR {
        SeAes0DinEndianR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&self) -> SeAes0KeyEndianR {
        SeAes0KeyEndianR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&self) -> SeAes0IvEndianR {
        SeAes0IvEndianR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_0_twk_endian(&self) -> SeAes0TwkEndianR {
        SeAes0TwkEndianR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&self) -> SeAes0CtrLenR {
        SeAes0CtrLenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&mut self) -> SeAes0DoutEndianW<'_, SeAes0EndianSpec> {
        SeAes0DoutEndianW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&mut self) -> SeAes0DinEndianW<'_, SeAes0EndianSpec> {
        SeAes0DinEndianW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&mut self) -> SeAes0KeyEndianW<'_, SeAes0EndianSpec> {
        SeAes0KeyEndianW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&mut self) -> SeAes0IvEndianW<'_, SeAes0EndianSpec> {
        SeAes0IvEndianW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_0_twk_endian(&mut self) -> SeAes0TwkEndianW<'_, SeAes0EndianSpec> {
        SeAes0TwkEndianW::new(self, 4)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&mut self) -> SeAes0CtrLenW<'_, SeAes0EndianSpec> {
        SeAes0CtrLenW::new(self, 30)
    }
}
#[doc = "AES Endian and CTR Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_endian::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_endian::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0EndianSpec;
impl crate::RegisterSpec for SeAes0EndianSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_endian::R`](R) reader structure"]
impl crate::Readable for SeAes0EndianSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_endian::W`](W) writer structure"]
impl crate::Writable for SeAes0EndianSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_ENDIAN to value 0x1f"]
impl crate::Resettable for SeAes0EndianSpec {
    const RESET_VALUE: u32 = 0x1f;
}
