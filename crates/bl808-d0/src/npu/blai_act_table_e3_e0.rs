#[doc = "Register `BLAI_ACT_TABLE_E3_E0` reader"]
pub type R = crate::R<BlaiActTableE3E0Spec>;
#[doc = "Register `BLAI_ACT_TABLE_E3_E0` writer"]
pub type W = crate::W<BlaiActTableE3E0Spec>;
#[doc = "Field `reg_act_table_E0` reader - 8-bit activation value (Index 0xE0)"]
pub type RegActTableE0R = crate::FieldReader;
#[doc = "Field `reg_act_table_E0` writer - 8-bit activation value (Index 0xE0)"]
pub type RegActTableE0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E1` reader - 8-bit activation value (Index 0xE1)"]
pub type RegActTableE1R = crate::FieldReader;
#[doc = "Field `reg_act_table_E1` writer - 8-bit activation value (Index 0xE1)"]
pub type RegActTableE1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E2` reader - 8-bit activation value (Index 0xE2)"]
pub type RegActTableE2R = crate::FieldReader;
#[doc = "Field `reg_act_table_E2` writer - 8-bit activation value (Index 0xE2)"]
pub type RegActTableE2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E3` reader - 8-bit activation value (Index 0xE3)"]
pub type RegActTableE3R = crate::FieldReader;
#[doc = "Field `reg_act_table_E3` writer - 8-bit activation value (Index 0xE3)"]
pub type RegActTableE3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xE0)"]
    #[inline(always)]
    pub fn reg_act_table_e0(&self) -> RegActTableE0R {
        RegActTableE0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xE1)"]
    #[inline(always)]
    pub fn reg_act_table_e1(&self) -> RegActTableE1R {
        RegActTableE1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xE2)"]
    #[inline(always)]
    pub fn reg_act_table_e2(&self) -> RegActTableE2R {
        RegActTableE2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xE3)"]
    #[inline(always)]
    pub fn reg_act_table_e3(&self) -> RegActTableE3R {
        RegActTableE3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xE0)"]
    #[inline(always)]
    pub fn reg_act_table_e0(&mut self) -> RegActTableE0W<'_, BlaiActTableE3E0Spec> {
        RegActTableE0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xE1)"]
    #[inline(always)]
    pub fn reg_act_table_e1(&mut self) -> RegActTableE1W<'_, BlaiActTableE3E0Spec> {
        RegActTableE1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xE2)"]
    #[inline(always)]
    pub fn reg_act_table_e2(&mut self) -> RegActTableE2W<'_, BlaiActTableE3E0Spec> {
        RegActTableE2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xE3)"]
    #[inline(always)]
    pub fn reg_act_table_e3(&mut self) -> RegActTableE3W<'_, BlaiActTableE3E0Spec> {
        RegActTableE3W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xE0-0xE3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_e3_e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_e3_e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableE3E0Spec;
impl crate::RegisterSpec for BlaiActTableE3E0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_e3_e0::R`](R) reader structure"]
impl crate::Readable for BlaiActTableE3E0Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_e3_e0::W`](W) writer structure"]
impl crate::Writable for BlaiActTableE3E0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_E3_E0 to value 0"]
impl crate::Resettable for BlaiActTableE3E0Spec {}
