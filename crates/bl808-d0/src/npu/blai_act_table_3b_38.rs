#[doc = "Register `BLAI_ACT_TABLE_3B_38` reader"]
pub type R = crate::R<BlaiActTable3b38Spec>;
#[doc = "Register `BLAI_ACT_TABLE_3B_38` writer"]
pub type W = crate::W<BlaiActTable3b38Spec>;
#[doc = "Field `reg_act_table_38` reader - 8-bit activation value (Index 0x38)"]
pub type RegActTable38R = crate::FieldReader;
#[doc = "Field `reg_act_table_38` writer - 8-bit activation value (Index 0x38)"]
pub type RegActTable38W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_39` reader - 8-bit activation value (Index 0x39)"]
pub type RegActTable39R = crate::FieldReader;
#[doc = "Field `reg_act_table_39` writer - 8-bit activation value (Index 0x39)"]
pub type RegActTable39W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_3A` reader - 8-bit activation value (Index 0x3A)"]
pub type RegActTable3aR = crate::FieldReader;
#[doc = "Field `reg_act_table_3A` writer - 8-bit activation value (Index 0x3A)"]
pub type RegActTable3aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_3B` reader - 8-bit activation value (Index 0x3B)"]
pub type RegActTable3bR = crate::FieldReader;
#[doc = "Field `reg_act_table_3B` writer - 8-bit activation value (Index 0x3B)"]
pub type RegActTable3bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x38)"]
    #[inline(always)]
    pub fn reg_act_table_38(&self) -> RegActTable38R {
        RegActTable38R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x39)"]
    #[inline(always)]
    pub fn reg_act_table_39(&self) -> RegActTable39R {
        RegActTable39R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x3A)"]
    #[inline(always)]
    pub fn reg_act_table_3a(&self) -> RegActTable3aR {
        RegActTable3aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x3B)"]
    #[inline(always)]
    pub fn reg_act_table_3b(&self) -> RegActTable3bR {
        RegActTable3bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x38)"]
    #[inline(always)]
    pub fn reg_act_table_38(&mut self) -> RegActTable38W<'_, BlaiActTable3b38Spec> {
        RegActTable38W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x39)"]
    #[inline(always)]
    pub fn reg_act_table_39(&mut self) -> RegActTable39W<'_, BlaiActTable3b38Spec> {
        RegActTable39W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x3A)"]
    #[inline(always)]
    pub fn reg_act_table_3a(&mut self) -> RegActTable3aW<'_, BlaiActTable3b38Spec> {
        RegActTable3aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x3B)"]
    #[inline(always)]
    pub fn reg_act_table_3b(&mut self) -> RegActTable3bW<'_, BlaiActTable3b38Spec> {
        RegActTable3bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x38-0x3B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_3b_38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_3b_38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable3b38Spec;
impl crate::RegisterSpec for BlaiActTable3b38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_3b_38::R`](R) reader structure"]
impl crate::Readable for BlaiActTable3b38Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_3b_38::W`](W) writer structure"]
impl crate::Writable for BlaiActTable3b38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_3B_38 to value 0"]
impl crate::Resettable for BlaiActTable3b38Spec {}
