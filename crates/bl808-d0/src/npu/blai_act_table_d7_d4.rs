#[doc = "Register `BLAI_ACT_TABLE_D7_D4` reader"]
pub type R = crate::R<BlaiActTableD7D4Spec>;
#[doc = "Register `BLAI_ACT_TABLE_D7_D4` writer"]
pub type W = crate::W<BlaiActTableD7D4Spec>;
#[doc = "Field `reg_act_table_D4` reader - 8-bit activation value (Index 0xD4)"]
pub type RegActTableD4R = crate::FieldReader;
#[doc = "Field `reg_act_table_D4` writer - 8-bit activation value (Index 0xD4)"]
pub type RegActTableD4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_D5` reader - 8-bit activation value (Index 0xD5)"]
pub type RegActTableD5R = crate::FieldReader;
#[doc = "Field `reg_act_table_D5` writer - 8-bit activation value (Index 0xD5)"]
pub type RegActTableD5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_D6` reader - 8-bit activation value (Index 0xD6)"]
pub type RegActTableD6R = crate::FieldReader;
#[doc = "Field `reg_act_table_D6` writer - 8-bit activation value (Index 0xD6)"]
pub type RegActTableD6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_D7` reader - 8-bit activation value (Index 0xD7)"]
pub type RegActTableD7R = crate::FieldReader;
#[doc = "Field `reg_act_table_D7` writer - 8-bit activation value (Index 0xD7)"]
pub type RegActTableD7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xD4)"]
    #[inline(always)]
    pub fn reg_act_table_d4(&self) -> RegActTableD4R {
        RegActTableD4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xD5)"]
    #[inline(always)]
    pub fn reg_act_table_d5(&self) -> RegActTableD5R {
        RegActTableD5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xD6)"]
    #[inline(always)]
    pub fn reg_act_table_d6(&self) -> RegActTableD6R {
        RegActTableD6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xD7)"]
    #[inline(always)]
    pub fn reg_act_table_d7(&self) -> RegActTableD7R {
        RegActTableD7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xD4)"]
    #[inline(always)]
    pub fn reg_act_table_d4(&mut self) -> RegActTableD4W<'_, BlaiActTableD7D4Spec> {
        RegActTableD4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xD5)"]
    #[inline(always)]
    pub fn reg_act_table_d5(&mut self) -> RegActTableD5W<'_, BlaiActTableD7D4Spec> {
        RegActTableD5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xD6)"]
    #[inline(always)]
    pub fn reg_act_table_d6(&mut self) -> RegActTableD6W<'_, BlaiActTableD7D4Spec> {
        RegActTableD6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xD7)"]
    #[inline(always)]
    pub fn reg_act_table_d7(&mut self) -> RegActTableD7W<'_, BlaiActTableD7D4Spec> {
        RegActTableD7W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xD4-0xD7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_d7_d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_d7_d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableD7D4Spec;
impl crate::RegisterSpec for BlaiActTableD7D4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_d7_d4::R`](R) reader structure"]
impl crate::Readable for BlaiActTableD7D4Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_d7_d4::W`](W) writer structure"]
impl crate::Writable for BlaiActTableD7D4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_D7_D4 to value 0"]
impl crate::Resettable for BlaiActTableD7D4Spec {}
