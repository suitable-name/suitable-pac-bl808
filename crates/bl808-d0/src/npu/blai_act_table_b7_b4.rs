#[doc = "Register `BLAI_ACT_TABLE_B7_B4` reader"]
pub type R = crate::R<BlaiActTableB7B4Spec>;
#[doc = "Register `BLAI_ACT_TABLE_B7_B4` writer"]
pub type W = crate::W<BlaiActTableB7B4Spec>;
#[doc = "Field `reg_act_table_B4` reader - 8-bit activation value (Index 0xB4)"]
pub type RegActTableB4R = crate::FieldReader;
#[doc = "Field `reg_act_table_B4` writer - 8-bit activation value (Index 0xB4)"]
pub type RegActTableB4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B5` reader - 8-bit activation value (Index 0xB5)"]
pub type RegActTableB5R = crate::FieldReader;
#[doc = "Field `reg_act_table_B5` writer - 8-bit activation value (Index 0xB5)"]
pub type RegActTableB5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B6` reader - 8-bit activation value (Index 0xB6)"]
pub type RegActTableB6R = crate::FieldReader;
#[doc = "Field `reg_act_table_B6` writer - 8-bit activation value (Index 0xB6)"]
pub type RegActTableB6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B7` reader - 8-bit activation value (Index 0xB7)"]
pub type RegActTableB7R = crate::FieldReader;
#[doc = "Field `reg_act_table_B7` writer - 8-bit activation value (Index 0xB7)"]
pub type RegActTableB7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xB4)"]
    #[inline(always)]
    pub fn reg_act_table_b4(&self) -> RegActTableB4R {
        RegActTableB4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xB5)"]
    #[inline(always)]
    pub fn reg_act_table_b5(&self) -> RegActTableB5R {
        RegActTableB5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xB6)"]
    #[inline(always)]
    pub fn reg_act_table_b6(&self) -> RegActTableB6R {
        RegActTableB6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xB7)"]
    #[inline(always)]
    pub fn reg_act_table_b7(&self) -> RegActTableB7R {
        RegActTableB7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xB4)"]
    #[inline(always)]
    pub fn reg_act_table_b4(&mut self) -> RegActTableB4W<'_, BlaiActTableB7B4Spec> {
        RegActTableB4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xB5)"]
    #[inline(always)]
    pub fn reg_act_table_b5(&mut self) -> RegActTableB5W<'_, BlaiActTableB7B4Spec> {
        RegActTableB5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xB6)"]
    #[inline(always)]
    pub fn reg_act_table_b6(&mut self) -> RegActTableB6W<'_, BlaiActTableB7B4Spec> {
        RegActTableB6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xB7)"]
    #[inline(always)]
    pub fn reg_act_table_b7(&mut self) -> RegActTableB7W<'_, BlaiActTableB7B4Spec> {
        RegActTableB7W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xB4-0xB7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_b7_b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_b7_b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableB7B4Spec;
impl crate::RegisterSpec for BlaiActTableB7B4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_b7_b4::R`](R) reader structure"]
impl crate::Readable for BlaiActTableB7B4Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_b7_b4::W`](W) writer structure"]
impl crate::Writable for BlaiActTableB7B4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_B7_B4 to value 0"]
impl crate::Resettable for BlaiActTableB7B4Spec {}
