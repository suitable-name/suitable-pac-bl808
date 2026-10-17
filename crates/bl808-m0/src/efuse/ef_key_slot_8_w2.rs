#[doc = "Register `EF_KEY_SLOT_8_W2` reader"]
pub type R = crate::R<EfKeySlot8W2Spec>;
#[doc = "Register `EF_KEY_SLOT_8_W2` writer"]
pub type W = crate::W<EfKeySlot8W2Spec>;
#[doc = "Field `ef_key_slot_8_w2` reader - "]
pub type EfKeySlot8W2R = crate::FieldReader<u32>;
#[doc = "Field `ef_key_slot_8_w2` writer - "]
pub type EfKeySlot8W2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_8_w2(&self) -> EfKeySlot8W2R {
        EfKeySlot8W2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_8_w2(&mut self) -> EfKeySlot8W2W<'_, EfKeySlot8W2Spec> {
        EfKeySlot8W2W::new(self, 0)
    }
}
#[doc = "Key Slot 8 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_8_w2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_8_w2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfKeySlot8W2Spec;
impl crate::RegisterSpec for EfKeySlot8W2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_key_slot_8_w2::R`](R) reader structure"]
impl crate::Readable for EfKeySlot8W2Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_key_slot_8_w2::W`](W) writer structure"]
impl crate::Writable for EfKeySlot8W2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_KEY_SLOT_8_W2 to value 0"]
impl crate::Resettable for EfKeySlot8W2Spec {}
