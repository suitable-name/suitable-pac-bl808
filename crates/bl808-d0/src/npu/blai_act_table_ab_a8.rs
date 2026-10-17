#[doc = "Register `BLAI_ACT_TABLE_AB_A8` reader"]
pub type R = crate::R<BlaiActTableAbA8Spec>;
#[doc = "Register `BLAI_ACT_TABLE_AB_A8` writer"]
pub type W = crate::W<BlaiActTableAbA8Spec>;
#[doc = "Field `reg_act_table_A8` reader - 8-bit activation value (Index 0xA8)"]
pub type RegActTableA8R = crate::FieldReader;
#[doc = "Field `reg_act_table_A8` writer - 8-bit activation value (Index 0xA8)"]
pub type RegActTableA8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A9` reader - 8-bit activation value (Index 0xA9)"]
pub type RegActTableA9R = crate::FieldReader;
#[doc = "Field `reg_act_table_A9` writer - 8-bit activation value (Index 0xA9)"]
pub type RegActTableA9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_AA` reader - 8-bit activation value (Index 0xAA)"]
pub type RegActTableAaR = crate::FieldReader;
#[doc = "Field `reg_act_table_AA` writer - 8-bit activation value (Index 0xAA)"]
pub type RegActTableAaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_AB` reader - 8-bit activation value (Index 0xAB)"]
pub type RegActTableAbR = crate::FieldReader;
#[doc = "Field `reg_act_table_AB` writer - 8-bit activation value (Index 0xAB)"]
pub type RegActTableAbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xA8)"]
    #[inline(always)]
    pub fn reg_act_table_a8(&self) -> RegActTableA8R {
        RegActTableA8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xA9)"]
    #[inline(always)]
    pub fn reg_act_table_a9(&self) -> RegActTableA9R {
        RegActTableA9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xAA)"]
    #[inline(always)]
    pub fn reg_act_table_aa(&self) -> RegActTableAaR {
        RegActTableAaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xAB)"]
    #[inline(always)]
    pub fn reg_act_table_ab(&self) -> RegActTableAbR {
        RegActTableAbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xA8)"]
    #[inline(always)]
    pub fn reg_act_table_a8(&mut self) -> RegActTableA8W<'_, BlaiActTableAbA8Spec> {
        RegActTableA8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xA9)"]
    #[inline(always)]
    pub fn reg_act_table_a9(&mut self) -> RegActTableA9W<'_, BlaiActTableAbA8Spec> {
        RegActTableA9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xAA)"]
    #[inline(always)]
    pub fn reg_act_table_aa(&mut self) -> RegActTableAaW<'_, BlaiActTableAbA8Spec> {
        RegActTableAaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xAB)"]
    #[inline(always)]
    pub fn reg_act_table_ab(&mut self) -> RegActTableAbW<'_, BlaiActTableAbA8Spec> {
        RegActTableAbW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xA8-0xAB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_ab_a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_ab_a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableAbA8Spec;
impl crate::RegisterSpec for BlaiActTableAbA8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_ab_a8::R`](R) reader structure"]
impl crate::Readable for BlaiActTableAbA8Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_ab_a8::W`](W) writer structure"]
impl crate::Writable for BlaiActTableAbA8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_AB_A8 to value 0"]
impl crate::Resettable for BlaiActTableAbA8Spec {}
