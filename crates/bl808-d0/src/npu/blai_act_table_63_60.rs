#[doc = "Register `BLAI_ACT_TABLE_63_60` reader"]
pub type R = crate::R<BlaiActTable63_60Spec>;
#[doc = "Register `BLAI_ACT_TABLE_63_60` writer"]
pub type W = crate::W<BlaiActTable63_60Spec>;
#[doc = "Field `reg_act_table_60` reader - 8-bit activation value (Index 0x60)"]
pub type RegActTable60R = crate::FieldReader;
#[doc = "Field `reg_act_table_60` writer - 8-bit activation value (Index 0x60)"]
pub type RegActTable60W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_61` reader - 8-bit activation value (Index 0x61)"]
pub type RegActTable61R = crate::FieldReader;
#[doc = "Field `reg_act_table_61` writer - 8-bit activation value (Index 0x61)"]
pub type RegActTable61W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_62` reader - 8-bit activation value (Index 0x62)"]
pub type RegActTable62R = crate::FieldReader;
#[doc = "Field `reg_act_table_62` writer - 8-bit activation value (Index 0x62)"]
pub type RegActTable62W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_63` reader - 8-bit activation value (Index 0x63)"]
pub type RegActTable63R = crate::FieldReader;
#[doc = "Field `reg_act_table_63` writer - 8-bit activation value (Index 0x63)"]
pub type RegActTable63W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x60)"]
    #[inline(always)]
    pub fn reg_act_table_60(&self) -> RegActTable60R {
        RegActTable60R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x61)"]
    #[inline(always)]
    pub fn reg_act_table_61(&self) -> RegActTable61R {
        RegActTable61R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x62)"]
    #[inline(always)]
    pub fn reg_act_table_62(&self) -> RegActTable62R {
        RegActTable62R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x63)"]
    #[inline(always)]
    pub fn reg_act_table_63(&self) -> RegActTable63R {
        RegActTable63R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x60)"]
    #[inline(always)]
    pub fn reg_act_table_60(&mut self) -> RegActTable60W<'_, BlaiActTable63_60Spec> {
        RegActTable60W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x61)"]
    #[inline(always)]
    pub fn reg_act_table_61(&mut self) -> RegActTable61W<'_, BlaiActTable63_60Spec> {
        RegActTable61W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x62)"]
    #[inline(always)]
    pub fn reg_act_table_62(&mut self) -> RegActTable62W<'_, BlaiActTable63_60Spec> {
        RegActTable62W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x63)"]
    #[inline(always)]
    pub fn reg_act_table_63(&mut self) -> RegActTable63W<'_, BlaiActTable63_60Spec> {
        RegActTable63W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x60-0x63)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_63_60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_63_60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable63_60Spec;
impl crate::RegisterSpec for BlaiActTable63_60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_63_60::R`](R) reader structure"]
impl crate::Readable for BlaiActTable63_60Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_63_60::W`](W) writer structure"]
impl crate::Writable for BlaiActTable63_60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_63_60 to value 0"]
impl crate::Resettable for BlaiActTable63_60Spec {}
