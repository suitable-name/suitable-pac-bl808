#[doc = "Register `BLAI_ACT_TABLE_9B_98` reader"]
pub type R = crate::R<BlaiActTable9b98Spec>;
#[doc = "Register `BLAI_ACT_TABLE_9B_98` writer"]
pub type W = crate::W<BlaiActTable9b98Spec>;
#[doc = "Field `reg_act_table_98` reader - 8-bit activation value (Index 0x98)"]
pub type RegActTable98R = crate::FieldReader;
#[doc = "Field `reg_act_table_98` writer - 8-bit activation value (Index 0x98)"]
pub type RegActTable98W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_99` reader - 8-bit activation value (Index 0x99)"]
pub type RegActTable99R = crate::FieldReader;
#[doc = "Field `reg_act_table_99` writer - 8-bit activation value (Index 0x99)"]
pub type RegActTable99W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_9A` reader - 8-bit activation value (Index 0x9A)"]
pub type RegActTable9aR = crate::FieldReader;
#[doc = "Field `reg_act_table_9A` writer - 8-bit activation value (Index 0x9A)"]
pub type RegActTable9aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_9B` reader - 8-bit activation value (Index 0x9B)"]
pub type RegActTable9bR = crate::FieldReader;
#[doc = "Field `reg_act_table_9B` writer - 8-bit activation value (Index 0x9B)"]
pub type RegActTable9bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x98)"]
    #[inline(always)]
    pub fn reg_act_table_98(&self) -> RegActTable98R {
        RegActTable98R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x99)"]
    #[inline(always)]
    pub fn reg_act_table_99(&self) -> RegActTable99R {
        RegActTable99R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x9A)"]
    #[inline(always)]
    pub fn reg_act_table_9a(&self) -> RegActTable9aR {
        RegActTable9aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x9B)"]
    #[inline(always)]
    pub fn reg_act_table_9b(&self) -> RegActTable9bR {
        RegActTable9bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x98)"]
    #[inline(always)]
    pub fn reg_act_table_98(&mut self) -> RegActTable98W<'_, BlaiActTable9b98Spec> {
        RegActTable98W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x99)"]
    #[inline(always)]
    pub fn reg_act_table_99(&mut self) -> RegActTable99W<'_, BlaiActTable9b98Spec> {
        RegActTable99W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x9A)"]
    #[inline(always)]
    pub fn reg_act_table_9a(&mut self) -> RegActTable9aW<'_, BlaiActTable9b98Spec> {
        RegActTable9aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x9B)"]
    #[inline(always)]
    pub fn reg_act_table_9b(&mut self) -> RegActTable9bW<'_, BlaiActTable9b98Spec> {
        RegActTable9bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x98-0x9B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_9b_98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_9b_98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable9b98Spec;
impl crate::RegisterSpec for BlaiActTable9b98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_9b_98::R`](R) reader structure"]
impl crate::Readable for BlaiActTable9b98Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_9b_98::W`](W) writer structure"]
impl crate::Writable for BlaiActTable9b98Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_9B_98 to value 0"]
impl crate::Resettable for BlaiActTable9b98Spec {}
