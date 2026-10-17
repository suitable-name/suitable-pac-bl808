#[doc = "Register `SE_SHA_0_ENDIAN` reader"]
pub type R = crate::R<SeSha0EndianSpec>;
#[doc = "Register `SE_SHA_0_ENDIAN` writer"]
pub type W = crate::W<SeSha0EndianSpec>;
#[doc = "Field `se_sha_0_dout_endian` reader - "]
pub type SeSha0DoutEndianR = crate::BitReader;
#[doc = "Field `se_sha_0_dout_endian` writer - "]
pub type SeSha0DoutEndianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_dout_endian(&self) -> SeSha0DoutEndianR {
        SeSha0DoutEndianR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_dout_endian(&mut self) -> SeSha0DoutEndianW<'_, SeSha0EndianSpec> {
        SeSha0DoutEndianW::new(self, 0)
    }
}
#[doc = "SHA Output Endian Control\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_endian::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_endian::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0EndianSpec;
impl crate::RegisterSpec for SeSha0EndianSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_endian::R`](R) reader structure"]
impl crate::Readable for SeSha0EndianSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_endian::W`](W) writer structure"]
impl crate::Writable for SeSha0EndianSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_SHA_0_ENDIAN to value 0x01"]
impl crate::Resettable for SeSha0EndianSpec {
    const RESET_VALUE: u32 = 0x01;
}
