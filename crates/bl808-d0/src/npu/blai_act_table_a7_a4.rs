#[doc = "Register `BLAI_ACT_TABLE_A7_A4` reader"]
pub type R = crate::R<BlaiActTableA7A4Spec>;
#[doc = "Register `BLAI_ACT_TABLE_A7_A4` writer"]
pub type W = crate::W<BlaiActTableA7A4Spec>;
#[doc = "Field `reg_act_table_A4` reader - 8-bit activation value (Index 0xA4)"]
pub type RegActTableA4R = crate::FieldReader;
#[doc = "Field `reg_act_table_A4` writer - 8-bit activation value (Index 0xA4)"]
pub type RegActTableA4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A5` reader - 8-bit activation value (Index 0xA5)"]
pub type RegActTableA5R = crate::FieldReader;
#[doc = "Field `reg_act_table_A5` writer - 8-bit activation value (Index 0xA5)"]
pub type RegActTableA5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A6` reader - 8-bit activation value (Index 0xA6)"]
pub type RegActTableA6R = crate::FieldReader;
#[doc = "Field `reg_act_table_A6` writer - 8-bit activation value (Index 0xA6)"]
pub type RegActTableA6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A7` reader - 8-bit activation value (Index 0xA7)"]
pub type RegActTableA7R = crate::FieldReader;
#[doc = "Field `reg_act_table_A7` writer - 8-bit activation value (Index 0xA7)"]
pub type RegActTableA7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xA4)"]
    #[inline(always)]
    pub fn reg_act_table_a4(&self) -> RegActTableA4R {
        RegActTableA4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xA5)"]
    #[inline(always)]
    pub fn reg_act_table_a5(&self) -> RegActTableA5R {
        RegActTableA5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xA6)"]
    #[inline(always)]
    pub fn reg_act_table_a6(&self) -> RegActTableA6R {
        RegActTableA6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xA7)"]
    #[inline(always)]
    pub fn reg_act_table_a7(&self) -> RegActTableA7R {
        RegActTableA7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xA4)"]
    #[inline(always)]
    pub fn reg_act_table_a4(&mut self) -> RegActTableA4W<'_, BlaiActTableA7A4Spec> {
        RegActTableA4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xA5)"]
    #[inline(always)]
    pub fn reg_act_table_a5(&mut self) -> RegActTableA5W<'_, BlaiActTableA7A4Spec> {
        RegActTableA5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xA6)"]
    #[inline(always)]
    pub fn reg_act_table_a6(&mut self) -> RegActTableA6W<'_, BlaiActTableA7A4Spec> {
        RegActTableA6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xA7)"]
    #[inline(always)]
    pub fn reg_act_table_a7(&mut self) -> RegActTableA7W<'_, BlaiActTableA7A4Spec> {
        RegActTableA7W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xA4-0xA7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_a7_a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_a7_a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableA7A4Spec;
impl crate::RegisterSpec for BlaiActTableA7A4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_a7_a4::R`](R) reader structure"]
impl crate::Readable for BlaiActTableA7A4Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_a7_a4::W`](W) writer structure"]
impl crate::Writable for BlaiActTableA7A4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_A7_A4 to value 0"]
impl crate::Resettable for BlaiActTableA7A4Spec {}
