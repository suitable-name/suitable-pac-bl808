#[doc = "Register `BLAI_ACT_TABLE_E7_E4` reader"]
pub type R = crate::R<BlaiActTableE7E4Spec>;
#[doc = "Register `BLAI_ACT_TABLE_E7_E4` writer"]
pub type W = crate::W<BlaiActTableE7E4Spec>;
#[doc = "Field `reg_act_table_E4` reader - 8-bit activation value (Index 0xE4)"]
pub type RegActTableE4R = crate::FieldReader;
#[doc = "Field `reg_act_table_E4` writer - 8-bit activation value (Index 0xE4)"]
pub type RegActTableE4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E5` reader - 8-bit activation value (Index 0xE5)"]
pub type RegActTableE5R = crate::FieldReader;
#[doc = "Field `reg_act_table_E5` writer - 8-bit activation value (Index 0xE5)"]
pub type RegActTableE5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E6` reader - 8-bit activation value (Index 0xE6)"]
pub type RegActTableE6R = crate::FieldReader;
#[doc = "Field `reg_act_table_E6` writer - 8-bit activation value (Index 0xE6)"]
pub type RegActTableE6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E7` reader - 8-bit activation value (Index 0xE7)"]
pub type RegActTableE7R = crate::FieldReader;
#[doc = "Field `reg_act_table_E7` writer - 8-bit activation value (Index 0xE7)"]
pub type RegActTableE7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xE4)"]
    #[inline(always)]
    pub fn reg_act_table_e4(&self) -> RegActTableE4R {
        RegActTableE4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xE5)"]
    #[inline(always)]
    pub fn reg_act_table_e5(&self) -> RegActTableE5R {
        RegActTableE5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xE6)"]
    #[inline(always)]
    pub fn reg_act_table_e6(&self) -> RegActTableE6R {
        RegActTableE6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xE7)"]
    #[inline(always)]
    pub fn reg_act_table_e7(&self) -> RegActTableE7R {
        RegActTableE7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xE4)"]
    #[inline(always)]
    pub fn reg_act_table_e4(&mut self) -> RegActTableE4W<'_, BlaiActTableE7E4Spec> {
        RegActTableE4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xE5)"]
    #[inline(always)]
    pub fn reg_act_table_e5(&mut self) -> RegActTableE5W<'_, BlaiActTableE7E4Spec> {
        RegActTableE5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xE6)"]
    #[inline(always)]
    pub fn reg_act_table_e6(&mut self) -> RegActTableE6W<'_, BlaiActTableE7E4Spec> {
        RegActTableE6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xE7)"]
    #[inline(always)]
    pub fn reg_act_table_e7(&mut self) -> RegActTableE7W<'_, BlaiActTableE7E4Spec> {
        RegActTableE7W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xE4-0xE7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_e7_e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_e7_e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableE7E4Spec;
impl crate::RegisterSpec for BlaiActTableE7E4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_e7_e4::R`](R) reader structure"]
impl crate::Readable for BlaiActTableE7E4Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_e7_e4::W`](W) writer structure"]
impl crate::Writable for BlaiActTableE7E4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_E7_E4 to value 0"]
impl crate::Resettable for BlaiActTableE7E4Spec {}
