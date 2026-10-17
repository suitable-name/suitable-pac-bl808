#[doc = "Register `EF_KEY_SLOT_11_W0` reader"]
pub type R = crate::R<EfKeySlot11W0Spec>;
#[doc = "Register `EF_KEY_SLOT_11_W0` writer"]
pub type W = crate::W<EfKeySlot11W0Spec>;
#[doc = "Field `ef_key_slot_11_w0` reader - "]
pub type EfKeySlot11W0R = crate::FieldReader<u32>;
#[doc = "Field `ef_key_slot_11_w0` writer - "]
pub type EfKeySlot11W0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_11_w0(&self) -> EfKeySlot11W0R {
        EfKeySlot11W0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_11_w0(&mut self) -> EfKeySlot11W0W<'_, EfKeySlot11W0Spec> {
        EfKeySlot11W0W::new(self, 0)
    }
}
#[doc = "Key Slot 11 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_11_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_11_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfKeySlot11W0Spec;
impl crate::RegisterSpec for EfKeySlot11W0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_key_slot_11_w0::R`](R) reader structure"]
impl crate::Readable for EfKeySlot11W0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_key_slot_11_w0::W`](W) writer structure"]
impl crate::Writable for EfKeySlot11W0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_KEY_SLOT_11_W0 to value 0"]
impl crate::Resettable for EfKeySlot11W0Spec {}
