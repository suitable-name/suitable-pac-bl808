#[doc = "Register `MAC_ADDR0` reader"]
pub type R = crate::R<MacAddr0Spec>;
#[doc = "Register `MAC_ADDR0` writer"]
pub type W = crate::W<MacAddr0Spec>;
#[doc = "Field `MAC_B5` reader - "]
pub type MacB5R = crate::FieldReader;
#[doc = "Field `MAC_B5` writer - "]
pub type MacB5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAC_B4` reader - "]
pub type MacB4R = crate::FieldReader;
#[doc = "Field `MAC_B4` writer - "]
pub type MacB4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAC_B3` reader - "]
pub type MacB3R = crate::FieldReader;
#[doc = "Field `MAC_B3` writer - "]
pub type MacB3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAC_B2` reader - "]
pub type MacB2R = crate::FieldReader;
#[doc = "Field `MAC_B2` writer - "]
pub type MacB2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b5(&self) -> MacB5R {
        MacB5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b4(&self) -> MacB4R {
        MacB4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn mac_b3(&self) -> MacB3R {
        MacB3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mac_b2(&self) -> MacB2R {
        MacB2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b5(&mut self) -> MacB5W<'_, MacAddr0Spec> {
        MacB5W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b4(&mut self) -> MacB4W<'_, MacAddr0Spec> {
        MacB4W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn mac_b3(&mut self) -> MacB3W<'_, MacAddr0Spec> {
        MacB3W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mac_b2(&mut self) -> MacB2W<'_, MacAddr0Spec> {
        MacB2W::new(self, 24)
    }
}
#[doc = "MAC Individual Address Register 0 (Bytes 5-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddr0Spec;
impl crate::RegisterSpec for MacAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr0::R`](R) reader structure"]
impl crate::Readable for MacAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr0::W`](W) writer structure"]
impl crate::Writable for MacAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_ADDR0 to value 0"]
impl crate::Resettable for MacAddr0Spec {}
