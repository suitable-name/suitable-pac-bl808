#[doc = "Register `KEY` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Field `KEY0` reader - "]
pub type Key0R = crate::FieldReader;
#[doc = "Field `KEY0` writer - "]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEY1` reader - "]
pub type Key1R = crate::FieldReader;
#[doc = "Field `KEY1` writer - "]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEY2` reader - "]
pub type Key2R = crate::FieldReader;
#[doc = "Field `KEY2` writer - "]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEY3` reader - "]
pub type Key3R = crate::FieldReader;
#[doc = "Field `KEY3` writer - "]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<'_, KeySpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<'_, KeySpec> {
        Key1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<'_, KeySpec> {
        Key2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<'_, KeySpec> {
        Key3W::new(self, 24)
    }
}
#[doc = "Color Key Value\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KeySpec {}
