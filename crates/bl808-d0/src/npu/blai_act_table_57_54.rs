#[doc = "Register `BLAI_ACT_TABLE_57_54` reader"]
pub type R = crate::R<BlaiActTable57_54Spec>;
#[doc = "Register `BLAI_ACT_TABLE_57_54` writer"]
pub type W = crate::W<BlaiActTable57_54Spec>;
#[doc = "Field `reg_act_table_54` reader - 8-bit activation value (Index 0x54)"]
pub type RegActTable54R = crate::FieldReader;
#[doc = "Field `reg_act_table_54` writer - 8-bit activation value (Index 0x54)"]
pub type RegActTable54W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_55` reader - 8-bit activation value (Index 0x55)"]
pub type RegActTable55R = crate::FieldReader;
#[doc = "Field `reg_act_table_55` writer - 8-bit activation value (Index 0x55)"]
pub type RegActTable55W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_56` reader - 8-bit activation value (Index 0x56)"]
pub type RegActTable56R = crate::FieldReader;
#[doc = "Field `reg_act_table_56` writer - 8-bit activation value (Index 0x56)"]
pub type RegActTable56W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_57` reader - 8-bit activation value (Index 0x57)"]
pub type RegActTable57R = crate::FieldReader;
#[doc = "Field `reg_act_table_57` writer - 8-bit activation value (Index 0x57)"]
pub type RegActTable57W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x54)"]
    #[inline(always)]
    pub fn reg_act_table_54(&self) -> RegActTable54R {
        RegActTable54R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x55)"]
    #[inline(always)]
    pub fn reg_act_table_55(&self) -> RegActTable55R {
        RegActTable55R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x56)"]
    #[inline(always)]
    pub fn reg_act_table_56(&self) -> RegActTable56R {
        RegActTable56R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x57)"]
    #[inline(always)]
    pub fn reg_act_table_57(&self) -> RegActTable57R {
        RegActTable57R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x54)"]
    #[inline(always)]
    pub fn reg_act_table_54(&mut self) -> RegActTable54W<'_, BlaiActTable57_54Spec> {
        RegActTable54W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x55)"]
    #[inline(always)]
    pub fn reg_act_table_55(&mut self) -> RegActTable55W<'_, BlaiActTable57_54Spec> {
        RegActTable55W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x56)"]
    #[inline(always)]
    pub fn reg_act_table_56(&mut self) -> RegActTable56W<'_, BlaiActTable57_54Spec> {
        RegActTable56W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x57)"]
    #[inline(always)]
    pub fn reg_act_table_57(&mut self) -> RegActTable57W<'_, BlaiActTable57_54Spec> {
        RegActTable57W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x54-0x57)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_57_54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_57_54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable57_54Spec;
impl crate::RegisterSpec for BlaiActTable57_54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_57_54::R`](R) reader structure"]
impl crate::Readable for BlaiActTable57_54Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_57_54::W`](W) writer structure"]
impl crate::Writable for BlaiActTable57_54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_57_54 to value 0"]
impl crate::Resettable for BlaiActTable57_54Spec {}
