#[doc = "Register `BLAI_ACT_TABLE_77_74` reader"]
pub type R = crate::R<BlaiActTable77_74Spec>;
#[doc = "Register `BLAI_ACT_TABLE_77_74` writer"]
pub type W = crate::W<BlaiActTable77_74Spec>;
#[doc = "Field `reg_act_table_74` reader - 8-bit activation value (Index 0x74)"]
pub type RegActTable74R = crate::FieldReader;
#[doc = "Field `reg_act_table_74` writer - 8-bit activation value (Index 0x74)"]
pub type RegActTable74W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_75` reader - 8-bit activation value (Index 0x75)"]
pub type RegActTable75R = crate::FieldReader;
#[doc = "Field `reg_act_table_75` writer - 8-bit activation value (Index 0x75)"]
pub type RegActTable75W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_76` reader - 8-bit activation value (Index 0x76)"]
pub type RegActTable76R = crate::FieldReader;
#[doc = "Field `reg_act_table_76` writer - 8-bit activation value (Index 0x76)"]
pub type RegActTable76W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_77` reader - 8-bit activation value (Index 0x77)"]
pub type RegActTable77R = crate::FieldReader;
#[doc = "Field `reg_act_table_77` writer - 8-bit activation value (Index 0x77)"]
pub type RegActTable77W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x74)"]
    #[inline(always)]
    pub fn reg_act_table_74(&self) -> RegActTable74R {
        RegActTable74R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x75)"]
    #[inline(always)]
    pub fn reg_act_table_75(&self) -> RegActTable75R {
        RegActTable75R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x76)"]
    #[inline(always)]
    pub fn reg_act_table_76(&self) -> RegActTable76R {
        RegActTable76R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x77)"]
    #[inline(always)]
    pub fn reg_act_table_77(&self) -> RegActTable77R {
        RegActTable77R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x74)"]
    #[inline(always)]
    pub fn reg_act_table_74(&mut self) -> RegActTable74W<'_, BlaiActTable77_74Spec> {
        RegActTable74W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x75)"]
    #[inline(always)]
    pub fn reg_act_table_75(&mut self) -> RegActTable75W<'_, BlaiActTable77_74Spec> {
        RegActTable75W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x76)"]
    #[inline(always)]
    pub fn reg_act_table_76(&mut self) -> RegActTable76W<'_, BlaiActTable77_74Spec> {
        RegActTable76W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x77)"]
    #[inline(always)]
    pub fn reg_act_table_77(&mut self) -> RegActTable77W<'_, BlaiActTable77_74Spec> {
        RegActTable77W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x74-0x77)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_77_74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_77_74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable77_74Spec;
impl crate::RegisterSpec for BlaiActTable77_74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_77_74::R`](R) reader structure"]
impl crate::Readable for BlaiActTable77_74Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_77_74::W`](W) writer structure"]
impl crate::Writable for BlaiActTable77_74Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_77_74 to value 0"]
impl crate::Resettable for BlaiActTable77_74Spec {}
