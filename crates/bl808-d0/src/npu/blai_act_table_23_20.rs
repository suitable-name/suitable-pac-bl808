#[doc = "Register `BLAI_ACT_TABLE_23_20` reader"]
pub type R = crate::R<BlaiActTable23_20Spec>;
#[doc = "Register `BLAI_ACT_TABLE_23_20` writer"]
pub type W = crate::W<BlaiActTable23_20Spec>;
#[doc = "Field `reg_act_table_20` reader - 8-bit activation value (Index 0x20)"]
pub type RegActTable20R = crate::FieldReader;
#[doc = "Field `reg_act_table_20` writer - 8-bit activation value (Index 0x20)"]
pub type RegActTable20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_21` reader - 8-bit activation value (Index 0x21)"]
pub type RegActTable21R = crate::FieldReader;
#[doc = "Field `reg_act_table_21` writer - 8-bit activation value (Index 0x21)"]
pub type RegActTable21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_22` reader - 8-bit activation value (Index 0x22)"]
pub type RegActTable22R = crate::FieldReader;
#[doc = "Field `reg_act_table_22` writer - 8-bit activation value (Index 0x22)"]
pub type RegActTable22W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_23` reader - 8-bit activation value (Index 0x23)"]
pub type RegActTable23R = crate::FieldReader;
#[doc = "Field `reg_act_table_23` writer - 8-bit activation value (Index 0x23)"]
pub type RegActTable23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x20)"]
    #[inline(always)]
    pub fn reg_act_table_20(&self) -> RegActTable20R {
        RegActTable20R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x21)"]
    #[inline(always)]
    pub fn reg_act_table_21(&self) -> RegActTable21R {
        RegActTable21R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x22)"]
    #[inline(always)]
    pub fn reg_act_table_22(&self) -> RegActTable22R {
        RegActTable22R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x23)"]
    #[inline(always)]
    pub fn reg_act_table_23(&self) -> RegActTable23R {
        RegActTable23R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x20)"]
    #[inline(always)]
    pub fn reg_act_table_20(&mut self) -> RegActTable20W<'_, BlaiActTable23_20Spec> {
        RegActTable20W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x21)"]
    #[inline(always)]
    pub fn reg_act_table_21(&mut self) -> RegActTable21W<'_, BlaiActTable23_20Spec> {
        RegActTable21W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x22)"]
    #[inline(always)]
    pub fn reg_act_table_22(&mut self) -> RegActTable22W<'_, BlaiActTable23_20Spec> {
        RegActTable22W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x23)"]
    #[inline(always)]
    pub fn reg_act_table_23(&mut self) -> RegActTable23W<'_, BlaiActTable23_20Spec> {
        RegActTable23W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x20-0x23)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_23_20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_23_20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable23_20Spec;
impl crate::RegisterSpec for BlaiActTable23_20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_23_20::R`](R) reader structure"]
impl crate::Readable for BlaiActTable23_20Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_23_20::W`](W) writer structure"]
impl crate::Writable for BlaiActTable23_20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_23_20 to value 0"]
impl crate::Resettable for BlaiActTable23_20Spec {}
