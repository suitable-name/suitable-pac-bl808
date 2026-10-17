#[doc = "Register `BLAI_ACT_TABLE_8B_88` reader"]
pub type R = crate::R<BlaiActTable8b88Spec>;
#[doc = "Register `BLAI_ACT_TABLE_8B_88` writer"]
pub type W = crate::W<BlaiActTable8b88Spec>;
#[doc = "Field `reg_act_table_88` reader - 8-bit activation value (Index 0x88)"]
pub type RegActTable88R = crate::FieldReader;
#[doc = "Field `reg_act_table_88` writer - 8-bit activation value (Index 0x88)"]
pub type RegActTable88W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_89` reader - 8-bit activation value (Index 0x89)"]
pub type RegActTable89R = crate::FieldReader;
#[doc = "Field `reg_act_table_89` writer - 8-bit activation value (Index 0x89)"]
pub type RegActTable89W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_8A` reader - 8-bit activation value (Index 0x8A)"]
pub type RegActTable8aR = crate::FieldReader;
#[doc = "Field `reg_act_table_8A` writer - 8-bit activation value (Index 0x8A)"]
pub type RegActTable8aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_8B` reader - 8-bit activation value (Index 0x8B)"]
pub type RegActTable8bR = crate::FieldReader;
#[doc = "Field `reg_act_table_8B` writer - 8-bit activation value (Index 0x8B)"]
pub type RegActTable8bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x88)"]
    #[inline(always)]
    pub fn reg_act_table_88(&self) -> RegActTable88R {
        RegActTable88R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x89)"]
    #[inline(always)]
    pub fn reg_act_table_89(&self) -> RegActTable89R {
        RegActTable89R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x8A)"]
    #[inline(always)]
    pub fn reg_act_table_8a(&self) -> RegActTable8aR {
        RegActTable8aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x8B)"]
    #[inline(always)]
    pub fn reg_act_table_8b(&self) -> RegActTable8bR {
        RegActTable8bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x88)"]
    #[inline(always)]
    pub fn reg_act_table_88(&mut self) -> RegActTable88W<'_, BlaiActTable8b88Spec> {
        RegActTable88W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x89)"]
    #[inline(always)]
    pub fn reg_act_table_89(&mut self) -> RegActTable89W<'_, BlaiActTable8b88Spec> {
        RegActTable89W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x8A)"]
    #[inline(always)]
    pub fn reg_act_table_8a(&mut self) -> RegActTable8aW<'_, BlaiActTable8b88Spec> {
        RegActTable8aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x8B)"]
    #[inline(always)]
    pub fn reg_act_table_8b(&mut self) -> RegActTable8bW<'_, BlaiActTable8b88Spec> {
        RegActTable8bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x88-0x8B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_8b_88::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_8b_88::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable8b88Spec;
impl crate::RegisterSpec for BlaiActTable8b88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_8b_88::R`](R) reader structure"]
impl crate::Readable for BlaiActTable8b88Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_8b_88::W`](W) writer structure"]
impl crate::Writable for BlaiActTable8b88Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_8B_88 to value 0"]
impl crate::Resettable for BlaiActTable8b88Spec {}
