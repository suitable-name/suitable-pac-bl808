#[doc = "Register `BLAI_ACT_TABLE_B3_B0` reader"]
pub type R = crate::R<BlaiActTableB3B0Spec>;
#[doc = "Register `BLAI_ACT_TABLE_B3_B0` writer"]
pub type W = crate::W<BlaiActTableB3B0Spec>;
#[doc = "Field `reg_act_table_B0` reader - 8-bit activation value (Index 0xB0)"]
pub type RegActTableB0R = crate::FieldReader;
#[doc = "Field `reg_act_table_B0` writer - 8-bit activation value (Index 0xB0)"]
pub type RegActTableB0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B1` reader - 8-bit activation value (Index 0xB1)"]
pub type RegActTableB1R = crate::FieldReader;
#[doc = "Field `reg_act_table_B1` writer - 8-bit activation value (Index 0xB1)"]
pub type RegActTableB1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B2` reader - 8-bit activation value (Index 0xB2)"]
pub type RegActTableB2R = crate::FieldReader;
#[doc = "Field `reg_act_table_B2` writer - 8-bit activation value (Index 0xB2)"]
pub type RegActTableB2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B3` reader - 8-bit activation value (Index 0xB3)"]
pub type RegActTableB3R = crate::FieldReader;
#[doc = "Field `reg_act_table_B3` writer - 8-bit activation value (Index 0xB3)"]
pub type RegActTableB3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xB0)"]
    #[inline(always)]
    pub fn reg_act_table_b0(&self) -> RegActTableB0R {
        RegActTableB0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xB1)"]
    #[inline(always)]
    pub fn reg_act_table_b1(&self) -> RegActTableB1R {
        RegActTableB1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xB2)"]
    #[inline(always)]
    pub fn reg_act_table_b2(&self) -> RegActTableB2R {
        RegActTableB2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xB3)"]
    #[inline(always)]
    pub fn reg_act_table_b3(&self) -> RegActTableB3R {
        RegActTableB3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xB0)"]
    #[inline(always)]
    pub fn reg_act_table_b0(&mut self) -> RegActTableB0W<'_, BlaiActTableB3B0Spec> {
        RegActTableB0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xB1)"]
    #[inline(always)]
    pub fn reg_act_table_b1(&mut self) -> RegActTableB1W<'_, BlaiActTableB3B0Spec> {
        RegActTableB1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xB2)"]
    #[inline(always)]
    pub fn reg_act_table_b2(&mut self) -> RegActTableB2W<'_, BlaiActTableB3B0Spec> {
        RegActTableB2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xB3)"]
    #[inline(always)]
    pub fn reg_act_table_b3(&mut self) -> RegActTableB3W<'_, BlaiActTableB3B0Spec> {
        RegActTableB3W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xB0-0xB3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_b3_b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_b3_b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableB3B0Spec;
impl crate::RegisterSpec for BlaiActTableB3B0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_b3_b0::R`](R) reader structure"]
impl crate::Readable for BlaiActTableB3B0Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_b3_b0::W`](W) writer structure"]
impl crate::Writable for BlaiActTableB3B0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_B3_B0 to value 0"]
impl crate::Resettable for BlaiActTableB3B0Spec {}
