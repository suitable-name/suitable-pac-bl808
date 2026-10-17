#[doc = "Register `BLAI_ACT_TABLE_1B_18` reader"]
pub type R = crate::R<BlaiActTable1b18Spec>;
#[doc = "Register `BLAI_ACT_TABLE_1B_18` writer"]
pub type W = crate::W<BlaiActTable1b18Spec>;
#[doc = "Field `reg_act_table_18` reader - 8-bit activation value (Index 0x18)"]
pub type RegActTable18R = crate::FieldReader;
#[doc = "Field `reg_act_table_18` writer - 8-bit activation value (Index 0x18)"]
pub type RegActTable18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_19` reader - 8-bit activation value (Index 0x19)"]
pub type RegActTable19R = crate::FieldReader;
#[doc = "Field `reg_act_table_19` writer - 8-bit activation value (Index 0x19)"]
pub type RegActTable19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_1A` reader - 8-bit activation value (Index 0x1A)"]
pub type RegActTable1aR = crate::FieldReader;
#[doc = "Field `reg_act_table_1A` writer - 8-bit activation value (Index 0x1A)"]
pub type RegActTable1aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_1B` reader - 8-bit activation value (Index 0x1B)"]
pub type RegActTable1bR = crate::FieldReader;
#[doc = "Field `reg_act_table_1B` writer - 8-bit activation value (Index 0x1B)"]
pub type RegActTable1bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x18)"]
    #[inline(always)]
    pub fn reg_act_table_18(&self) -> RegActTable18R {
        RegActTable18R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x19)"]
    #[inline(always)]
    pub fn reg_act_table_19(&self) -> RegActTable19R {
        RegActTable19R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x1A)"]
    #[inline(always)]
    pub fn reg_act_table_1a(&self) -> RegActTable1aR {
        RegActTable1aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x1B)"]
    #[inline(always)]
    pub fn reg_act_table_1b(&self) -> RegActTable1bR {
        RegActTable1bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x18)"]
    #[inline(always)]
    pub fn reg_act_table_18(&mut self) -> RegActTable18W<'_, BlaiActTable1b18Spec> {
        RegActTable18W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x19)"]
    #[inline(always)]
    pub fn reg_act_table_19(&mut self) -> RegActTable19W<'_, BlaiActTable1b18Spec> {
        RegActTable19W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x1A)"]
    #[inline(always)]
    pub fn reg_act_table_1a(&mut self) -> RegActTable1aW<'_, BlaiActTable1b18Spec> {
        RegActTable1aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x1B)"]
    #[inline(always)]
    pub fn reg_act_table_1b(&mut self) -> RegActTable1bW<'_, BlaiActTable1b18Spec> {
        RegActTable1bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x18-0x1B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_1b_18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_1b_18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable1b18Spec;
impl crate::RegisterSpec for BlaiActTable1b18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_1b_18::R`](R) reader structure"]
impl crate::Readable for BlaiActTable1b18Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_1b_18::W`](W) writer structure"]
impl crate::Writable for BlaiActTable1b18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_1B_18 to value 0"]
impl crate::Resettable for BlaiActTable1b18Spec {}
