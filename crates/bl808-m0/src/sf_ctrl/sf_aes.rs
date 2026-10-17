#[doc = "Register `SF_AES` reader"]
pub type R = crate::R<SfAesSpec>;
#[doc = "Register `SF_AES` writer"]
pub type W = crate::W<SfAesSpec>;
#[doc = "Field `sf_aes_en` reader - "]
pub type SfAesEnR = crate::BitReader;
#[doc = "Field `sf_aes_en` writer - "]
pub type SfAesEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_mode` reader - "]
pub type SfAesModeR = crate::FieldReader;
#[doc = "Field `sf_aes_mode` writer - "]
pub type SfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_aes_blk_mode` reader - "]
pub type SfAesBlkModeR = crate::BitReader;
#[doc = "Field `sf_aes_blk_mode` writer - "]
pub type SfAesBlkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_xts_key_opt` reader - "]
pub type SfAesXtsKeyOptR = crate::BitReader;
#[doc = "Field `sf_aes_xts_key_opt` writer - "]
pub type SfAesXtsKeyOptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_status` reader - "]
pub type SfAesStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&self) -> SfAesEnR {
        SfAesEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&self) -> SfAesModeR {
        SfAesModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_blk_mode(&self) -> SfAesBlkModeR {
        SfAesBlkModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_xts_key_opt(&self) -> SfAesXtsKeyOptR {
        SfAesXtsKeyOptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn sf_aes_status(&self) -> SfAesStatusR {
        SfAesStatusR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&mut self) -> SfAesEnW<'_, SfAesSpec> {
        SfAesEnW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&mut self) -> SfAesModeW<'_, SfAesSpec> {
        SfAesModeW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_blk_mode(&mut self) -> SfAesBlkModeW<'_, SfAesSpec> {
        SfAesBlkModeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_xts_key_opt(&mut self) -> SfAesXtsKeyOptW<'_, SfAesSpec> {
        SfAesXtsKeyOptW::new(self, 4)
    }
}
#[doc = "AES Global Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAesSpec;
impl crate::RegisterSpec for SfAesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_aes::R`](R) reader structure"]
impl crate::Readable for SfAesSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_aes::W`](W) writer structure"]
impl crate::Writable for SfAesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_AES to value 0"]
impl crate::Resettable for SfAesSpec {}
