#[doc = "Register `BLAI_ACT_TABLE_C7_C4` reader"]
pub type R = crate::R<BlaiActTableC7C4Spec>;
#[doc = "Register `BLAI_ACT_TABLE_C7_C4` writer"]
pub type W = crate::W<BlaiActTableC7C4Spec>;
#[doc = "Field `reg_act_table_C4` reader - 8-bit activation value (Index 0xC4)"]
pub type RegActTableC4R = crate::FieldReader;
#[doc = "Field `reg_act_table_C4` writer - 8-bit activation value (Index 0xC4)"]
pub type RegActTableC4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C5` reader - 8-bit activation value (Index 0xC5)"]
pub type RegActTableC5R = crate::FieldReader;
#[doc = "Field `reg_act_table_C5` writer - 8-bit activation value (Index 0xC5)"]
pub type RegActTableC5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C6` reader - 8-bit activation value (Index 0xC6)"]
pub type RegActTableC6R = crate::FieldReader;
#[doc = "Field `reg_act_table_C6` writer - 8-bit activation value (Index 0xC6)"]
pub type RegActTableC6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C7` reader - 8-bit activation value (Index 0xC7)"]
pub type RegActTableC7R = crate::FieldReader;
#[doc = "Field `reg_act_table_C7` writer - 8-bit activation value (Index 0xC7)"]
pub type RegActTableC7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xC4)"]
    #[inline(always)]
    pub fn reg_act_table_c4(&self) -> RegActTableC4R {
        RegActTableC4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xC5)"]
    #[inline(always)]
    pub fn reg_act_table_c5(&self) -> RegActTableC5R {
        RegActTableC5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xC6)"]
    #[inline(always)]
    pub fn reg_act_table_c6(&self) -> RegActTableC6R {
        RegActTableC6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xC7)"]
    #[inline(always)]
    pub fn reg_act_table_c7(&self) -> RegActTableC7R {
        RegActTableC7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xC4)"]
    #[inline(always)]
    pub fn reg_act_table_c4(&mut self) -> RegActTableC4W<'_, BlaiActTableC7C4Spec> {
        RegActTableC4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xC5)"]
    #[inline(always)]
    pub fn reg_act_table_c5(&mut self) -> RegActTableC5W<'_, BlaiActTableC7C4Spec> {
        RegActTableC5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xC6)"]
    #[inline(always)]
    pub fn reg_act_table_c6(&mut self) -> RegActTableC6W<'_, BlaiActTableC7C4Spec> {
        RegActTableC6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xC7)"]
    #[inline(always)]
    pub fn reg_act_table_c7(&mut self) -> RegActTableC7W<'_, BlaiActTableC7C4Spec> {
        RegActTableC7W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xC4-0xC7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_c7_c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_c7_c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableC7C4Spec;
impl crate::RegisterSpec for BlaiActTableC7C4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_c7_c4::R`](R) reader structure"]
impl crate::Readable for BlaiActTableC7C4Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_c7_c4::W`](W) writer structure"]
impl crate::Writable for BlaiActTableC7C4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_C7_C4 to value 0"]
impl crate::Resettable for BlaiActTableC7C4Spec {}
