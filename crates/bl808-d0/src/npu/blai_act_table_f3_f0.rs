#[doc = "Register `BLAI_ACT_TABLE_F3_F0` reader"]
pub type R = crate::R<BlaiActTableF3F0Spec>;
#[doc = "Register `BLAI_ACT_TABLE_F3_F0` writer"]
pub type W = crate::W<BlaiActTableF3F0Spec>;
#[doc = "Field `reg_act_table_F0` reader - 8-bit activation value (Index 0xF0)"]
pub type RegActTableF0R = crate::FieldReader;
#[doc = "Field `reg_act_table_F0` writer - 8-bit activation value (Index 0xF0)"]
pub type RegActTableF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F1` reader - 8-bit activation value (Index 0xF1)"]
pub type RegActTableF1R = crate::FieldReader;
#[doc = "Field `reg_act_table_F1` writer - 8-bit activation value (Index 0xF1)"]
pub type RegActTableF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F2` reader - 8-bit activation value (Index 0xF2)"]
pub type RegActTableF2R = crate::FieldReader;
#[doc = "Field `reg_act_table_F2` writer - 8-bit activation value (Index 0xF2)"]
pub type RegActTableF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F3` reader - 8-bit activation value (Index 0xF3)"]
pub type RegActTableF3R = crate::FieldReader;
#[doc = "Field `reg_act_table_F3` writer - 8-bit activation value (Index 0xF3)"]
pub type RegActTableF3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xF0)"]
    #[inline(always)]
    pub fn reg_act_table_f0(&self) -> RegActTableF0R {
        RegActTableF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xF1)"]
    #[inline(always)]
    pub fn reg_act_table_f1(&self) -> RegActTableF1R {
        RegActTableF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xF2)"]
    #[inline(always)]
    pub fn reg_act_table_f2(&self) -> RegActTableF2R {
        RegActTableF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xF3)"]
    #[inline(always)]
    pub fn reg_act_table_f3(&self) -> RegActTableF3R {
        RegActTableF3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xF0)"]
    #[inline(always)]
    pub fn reg_act_table_f0(&mut self) -> RegActTableF0W<'_, BlaiActTableF3F0Spec> {
        RegActTableF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xF1)"]
    #[inline(always)]
    pub fn reg_act_table_f1(&mut self) -> RegActTableF1W<'_, BlaiActTableF3F0Spec> {
        RegActTableF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xF2)"]
    #[inline(always)]
    pub fn reg_act_table_f2(&mut self) -> RegActTableF2W<'_, BlaiActTableF3F0Spec> {
        RegActTableF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xF3)"]
    #[inline(always)]
    pub fn reg_act_table_f3(&mut self) -> RegActTableF3W<'_, BlaiActTableF3F0Spec> {
        RegActTableF3W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xF0-0xF3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_f3_f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_f3_f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableF3F0Spec;
impl crate::RegisterSpec for BlaiActTableF3F0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_f3_f0::R`](R) reader structure"]
impl crate::Readable for BlaiActTableF3F0Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_f3_f0::W`](W) writer structure"]
impl crate::Writable for BlaiActTableF3F0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_F3_F0 to value 0"]
impl crate::Resettable for BlaiActTableF3F0Spec {}
