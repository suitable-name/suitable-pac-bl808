#[doc = "Register `BLAI_ACT_TABLE_F7_F4` reader"]
pub type R = crate::R<BlaiActTableF7F4Spec>;
#[doc = "Register `BLAI_ACT_TABLE_F7_F4` writer"]
pub type W = crate::W<BlaiActTableF7F4Spec>;
#[doc = "Field `reg_act_table_F4` reader - 8-bit activation value (Index 0xF4)"]
pub type RegActTableF4R = crate::FieldReader;
#[doc = "Field `reg_act_table_F4` writer - 8-bit activation value (Index 0xF4)"]
pub type RegActTableF4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F5` reader - 8-bit activation value (Index 0xF5)"]
pub type RegActTableF5R = crate::FieldReader;
#[doc = "Field `reg_act_table_F5` writer - 8-bit activation value (Index 0xF5)"]
pub type RegActTableF5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F6` reader - 8-bit activation value (Index 0xF6)"]
pub type RegActTableF6R = crate::FieldReader;
#[doc = "Field `reg_act_table_F6` writer - 8-bit activation value (Index 0xF6)"]
pub type RegActTableF6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F7` reader - 8-bit activation value (Index 0xF7)"]
pub type RegActTableF7R = crate::FieldReader;
#[doc = "Field `reg_act_table_F7` writer - 8-bit activation value (Index 0xF7)"]
pub type RegActTableF7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xF4)"]
    #[inline(always)]
    pub fn reg_act_table_f4(&self) -> RegActTableF4R {
        RegActTableF4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xF5)"]
    #[inline(always)]
    pub fn reg_act_table_f5(&self) -> RegActTableF5R {
        RegActTableF5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xF6)"]
    #[inline(always)]
    pub fn reg_act_table_f6(&self) -> RegActTableF6R {
        RegActTableF6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xF7)"]
    #[inline(always)]
    pub fn reg_act_table_f7(&self) -> RegActTableF7R {
        RegActTableF7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xF4)"]
    #[inline(always)]
    pub fn reg_act_table_f4(&mut self) -> RegActTableF4W<'_, BlaiActTableF7F4Spec> {
        RegActTableF4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xF5)"]
    #[inline(always)]
    pub fn reg_act_table_f5(&mut self) -> RegActTableF5W<'_, BlaiActTableF7F4Spec> {
        RegActTableF5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xF6)"]
    #[inline(always)]
    pub fn reg_act_table_f6(&mut self) -> RegActTableF6W<'_, BlaiActTableF7F4Spec> {
        RegActTableF6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xF7)"]
    #[inline(always)]
    pub fn reg_act_table_f7(&mut self) -> RegActTableF7W<'_, BlaiActTableF7F4Spec> {
        RegActTableF7W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xF4-0xF7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_f7_f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_f7_f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableF7F4Spec;
impl crate::RegisterSpec for BlaiActTableF7F4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_f7_f4::R`](R) reader structure"]
impl crate::Readable for BlaiActTableF7F4Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_f7_f4::W`](W) writer structure"]
impl crate::Writable for BlaiActTableF7F4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_F7_F4 to value 0"]
impl crate::Resettable for BlaiActTableF7F4Spec {}
