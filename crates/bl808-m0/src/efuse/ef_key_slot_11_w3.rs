#[doc = "Register `EF_KEY_SLOT_11_W3` reader"]
pub type R = crate::R<EfKeySlot11W3Spec>;
#[doc = "Register `EF_KEY_SLOT_11_W3` writer"]
pub type W = crate::W<EfKeySlot11W3Spec>;
#[doc = "Field `ef_key_slot_11_w3` reader - "]
pub type EfKeySlot11W3R = crate::FieldReader<u32>;
#[doc = "Field `ef_key_slot_11_w3` writer - "]
pub type EfKeySlot11W3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_11_w3(&self) -> EfKeySlot11W3R {
        EfKeySlot11W3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_11_w3(&mut self) -> EfKeySlot11W3W<'_, EfKeySlot11W3Spec> {
        EfKeySlot11W3W::new(self, 0)
    }
}
#[doc = "Key Slot 11 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_11_w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_11_w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfKeySlot11W3Spec;
impl crate::RegisterSpec for EfKeySlot11W3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_key_slot_11_w3::R`](R) reader structure"]
impl crate::Readable for EfKeySlot11W3Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_key_slot_11_w3::W`](W) writer structure"]
impl crate::Writable for EfKeySlot11W3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_KEY_SLOT_11_W3 to value 0"]
impl crate::Resettable for EfKeySlot11W3Spec {}
