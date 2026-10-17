#[doc = "Register `BLAI_ACT_TABLE_87_84` reader"]
pub type R = crate::R<BlaiActTable87_84Spec>;
#[doc = "Register `BLAI_ACT_TABLE_87_84` writer"]
pub type W = crate::W<BlaiActTable87_84Spec>;
#[doc = "Field `reg_act_table_84` reader - 8-bit activation value (Index 0x84)"]
pub type RegActTable84R = crate::FieldReader;
#[doc = "Field `reg_act_table_84` writer - 8-bit activation value (Index 0x84)"]
pub type RegActTable84W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_85` reader - 8-bit activation value (Index 0x85)"]
pub type RegActTable85R = crate::FieldReader;
#[doc = "Field `reg_act_table_85` writer - 8-bit activation value (Index 0x85)"]
pub type RegActTable85W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_86` reader - 8-bit activation value (Index 0x86)"]
pub type RegActTable86R = crate::FieldReader;
#[doc = "Field `reg_act_table_86` writer - 8-bit activation value (Index 0x86)"]
pub type RegActTable86W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_87` reader - 8-bit activation value (Index 0x87)"]
pub type RegActTable87R = crate::FieldReader;
#[doc = "Field `reg_act_table_87` writer - 8-bit activation value (Index 0x87)"]
pub type RegActTable87W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x84)"]
    #[inline(always)]
    pub fn reg_act_table_84(&self) -> RegActTable84R {
        RegActTable84R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x85)"]
    #[inline(always)]
    pub fn reg_act_table_85(&self) -> RegActTable85R {
        RegActTable85R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x86)"]
    #[inline(always)]
    pub fn reg_act_table_86(&self) -> RegActTable86R {
        RegActTable86R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x87)"]
    #[inline(always)]
    pub fn reg_act_table_87(&self) -> RegActTable87R {
        RegActTable87R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x84)"]
    #[inline(always)]
    pub fn reg_act_table_84(&mut self) -> RegActTable84W<'_, BlaiActTable87_84Spec> {
        RegActTable84W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x85)"]
    #[inline(always)]
    pub fn reg_act_table_85(&mut self) -> RegActTable85W<'_, BlaiActTable87_84Spec> {
        RegActTable85W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x86)"]
    #[inline(always)]
    pub fn reg_act_table_86(&mut self) -> RegActTable86W<'_, BlaiActTable87_84Spec> {
        RegActTable86W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x87)"]
    #[inline(always)]
    pub fn reg_act_table_87(&mut self) -> RegActTable87W<'_, BlaiActTable87_84Spec> {
        RegActTable87W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x84-0x87)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_87_84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_87_84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable87_84Spec;
impl crate::RegisterSpec for BlaiActTable87_84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_87_84::R`](R) reader structure"]
impl crate::Readable for BlaiActTable87_84Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_87_84::W`](W) writer structure"]
impl crate::Writable for BlaiActTable87_84Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_87_84 to value 0"]
impl crate::Resettable for BlaiActTable87_84Spec {}
