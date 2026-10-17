#[doc = "Register `EF_KEY_SLOT_9_W1` reader"]
pub type R = crate::R<EfKeySlot9W1Spec>;
#[doc = "Register `EF_KEY_SLOT_9_W1` writer"]
pub type W = crate::W<EfKeySlot9W1Spec>;
#[doc = "Field `ef_key_slot_9_w1` reader - "]
pub type EfKeySlot9W1R = crate::FieldReader<u32>;
#[doc = "Field `ef_key_slot_9_w1` writer - "]
pub type EfKeySlot9W1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_9_w1(&self) -> EfKeySlot9W1R {
        EfKeySlot9W1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_9_w1(&mut self) -> EfKeySlot9W1W<'_, EfKeySlot9W1Spec> {
        EfKeySlot9W1W::new(self, 0)
    }
}
#[doc = "Key Slot 9 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_9_w1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_9_w1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfKeySlot9W1Spec;
impl crate::RegisterSpec for EfKeySlot9W1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_key_slot_9_w1::R`](R) reader structure"]
impl crate::Readable for EfKeySlot9W1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_key_slot_9_w1::W`](W) writer structure"]
impl crate::Writable for EfKeySlot9W1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_KEY_SLOT_9_W1 to value 0"]
impl crate::Resettable for EfKeySlot9W1Spec {}
