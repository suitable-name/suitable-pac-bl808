#[doc = "Register `BLAI_ACT_TABLE_A3_A0` reader"]
pub type R = crate::R<BlaiActTableA3A0Spec>;
#[doc = "Register `BLAI_ACT_TABLE_A3_A0` writer"]
pub type W = crate::W<BlaiActTableA3A0Spec>;
#[doc = "Field `reg_act_table_A0` reader - 8-bit activation value (Index 0xA0)"]
pub type RegActTableA0R = crate::FieldReader;
#[doc = "Field `reg_act_table_A0` writer - 8-bit activation value (Index 0xA0)"]
pub type RegActTableA0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A1` reader - 8-bit activation value (Index 0xA1)"]
pub type RegActTableA1R = crate::FieldReader;
#[doc = "Field `reg_act_table_A1` writer - 8-bit activation value (Index 0xA1)"]
pub type RegActTableA1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A2` reader - 8-bit activation value (Index 0xA2)"]
pub type RegActTableA2R = crate::FieldReader;
#[doc = "Field `reg_act_table_A2` writer - 8-bit activation value (Index 0xA2)"]
pub type RegActTableA2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_A3` reader - 8-bit activation value (Index 0xA3)"]
pub type RegActTableA3R = crate::FieldReader;
#[doc = "Field `reg_act_table_A3` writer - 8-bit activation value (Index 0xA3)"]
pub type RegActTableA3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xA0)"]
    #[inline(always)]
    pub fn reg_act_table_a0(&self) -> RegActTableA0R {
        RegActTableA0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xA1)"]
    #[inline(always)]
    pub fn reg_act_table_a1(&self) -> RegActTableA1R {
        RegActTableA1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xA2)"]
    #[inline(always)]
    pub fn reg_act_table_a2(&self) -> RegActTableA2R {
        RegActTableA2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xA3)"]
    #[inline(always)]
    pub fn reg_act_table_a3(&self) -> RegActTableA3R {
        RegActTableA3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xA0)"]
    #[inline(always)]
    pub fn reg_act_table_a0(&mut self) -> RegActTableA0W<'_, BlaiActTableA3A0Spec> {
        RegActTableA0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xA1)"]
    #[inline(always)]
    pub fn reg_act_table_a1(&mut self) -> RegActTableA1W<'_, BlaiActTableA3A0Spec> {
        RegActTableA1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xA2)"]
    #[inline(always)]
    pub fn reg_act_table_a2(&mut self) -> RegActTableA2W<'_, BlaiActTableA3A0Spec> {
        RegActTableA2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xA3)"]
    #[inline(always)]
    pub fn reg_act_table_a3(&mut self) -> RegActTableA3W<'_, BlaiActTableA3A0Spec> {
        RegActTableA3W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xA0-0xA3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_a3_a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_a3_a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableA3A0Spec;
impl crate::RegisterSpec for BlaiActTableA3A0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_a3_a0::R`](R) reader structure"]
impl crate::Readable for BlaiActTableA3A0Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_a3_a0::W`](W) writer structure"]
impl crate::Writable for BlaiActTableA3A0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_A3_A0 to value 0"]
impl crate::Resettable for BlaiActTableA3A0Spec {}
