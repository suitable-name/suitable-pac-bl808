#[doc = "Register `KEY_EN` reader"]
pub type R = crate::R<KeyEnSpec>;
#[doc = "Register `KEY_EN` writer"]
pub type W = crate::W<KeyEnSpec>;
#[doc = "Field `KEY_EN` reader - Enable color key function"]
pub type KeyEnR = crate::BitReader;
#[doc = "Field `KEY_EN` writer - Enable color key function"]
pub type KeyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_MODE` reader - Key mode selection (0:8b, 1:16b, 2:24b, 3:32b)"]
pub type KeyModeR = crate::FieldReader;
#[doc = "Field `KEY_MODE` writer - Key mode selection (0:8b, 1:16b, 2:24b, 3:32b)"]
pub type KeyModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY_STRB` reader - "]
pub type KeyStrbR = crate::FieldReader;
#[doc = "Field `KEY_STRB` writer - "]
pub type KeyStrbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable color key function"]
    #[inline(always)]
    pub fn key_en(&self) -> KeyEnR {
        KeyEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Key mode selection (0:8b, 1:16b, 2:24b, 3:32b)"]
    #[inline(always)]
    pub fn key_mode(&self) -> KeyModeR {
        KeyModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_strb(&self) -> KeyStrbR {
        KeyStrbR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable color key function"]
    #[inline(always)]
    pub fn key_en(&mut self) -> KeyEnW<'_, KeyEnSpec> {
        KeyEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Key mode selection (0:8b, 1:16b, 2:24b, 3:32b)"]
    #[inline(always)]
    pub fn key_mode(&mut self) -> KeyModeW<'_, KeyEnSpec> {
        KeyModeW::new(self, 1)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_strb(&mut self) -> KeyStrbW<'_, KeyEnSpec> {
        KeyStrbW::new(self, 4)
    }
}
#[doc = "Color Key Control\n\nYou can [`read`](crate::Reg::read) this register and get [`key_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyEnSpec;
impl crate::RegisterSpec for KeyEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_en::R`](R) reader structure"]
impl crate::Readable for KeyEnSpec {}
#[doc = "`write(|w| ..)` method takes [`key_en::W`](W) writer structure"]
impl crate::Writable for KeyEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_EN to value 0"]
impl crate::Resettable for KeyEnSpec {}
