#[doc = "Register `BLAI_ACT_TABLE_13_10` reader"]
pub type R = crate::R<BlaiActTable13_10Spec>;
#[doc = "Register `BLAI_ACT_TABLE_13_10` writer"]
pub type W = crate::W<BlaiActTable13_10Spec>;
#[doc = "Field `reg_act_table_10` reader - 8-bit activation value (Index 0x10)"]
pub type RegActTable10R = crate::FieldReader;
#[doc = "Field `reg_act_table_10` writer - 8-bit activation value (Index 0x10)"]
pub type RegActTable10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_11` reader - 8-bit activation value (Index 0x11)"]
pub type RegActTable11R = crate::FieldReader;
#[doc = "Field `reg_act_table_11` writer - 8-bit activation value (Index 0x11)"]
pub type RegActTable11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_12` reader - 8-bit activation value (Index 0x12)"]
pub type RegActTable12R = crate::FieldReader;
#[doc = "Field `reg_act_table_12` writer - 8-bit activation value (Index 0x12)"]
pub type RegActTable12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_13` reader - 8-bit activation value (Index 0x13)"]
pub type RegActTable13R = crate::FieldReader;
#[doc = "Field `reg_act_table_13` writer - 8-bit activation value (Index 0x13)"]
pub type RegActTable13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x10)"]
    #[inline(always)]
    pub fn reg_act_table_10(&self) -> RegActTable10R {
        RegActTable10R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x11)"]
    #[inline(always)]
    pub fn reg_act_table_11(&self) -> RegActTable11R {
        RegActTable11R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x12)"]
    #[inline(always)]
    pub fn reg_act_table_12(&self) -> RegActTable12R {
        RegActTable12R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x13)"]
    #[inline(always)]
    pub fn reg_act_table_13(&self) -> RegActTable13R {
        RegActTable13R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x10)"]
    #[inline(always)]
    pub fn reg_act_table_10(&mut self) -> RegActTable10W<'_, BlaiActTable13_10Spec> {
        RegActTable10W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x11)"]
    #[inline(always)]
    pub fn reg_act_table_11(&mut self) -> RegActTable11W<'_, BlaiActTable13_10Spec> {
        RegActTable11W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x12)"]
    #[inline(always)]
    pub fn reg_act_table_12(&mut self) -> RegActTable12W<'_, BlaiActTable13_10Spec> {
        RegActTable12W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x13)"]
    #[inline(always)]
    pub fn reg_act_table_13(&mut self) -> RegActTable13W<'_, BlaiActTable13_10Spec> {
        RegActTable13W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x10-0x13)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_13_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_13_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable13_10Spec;
impl crate::RegisterSpec for BlaiActTable13_10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_13_10::R`](R) reader structure"]
impl crate::Readable for BlaiActTable13_10Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_13_10::W`](W) writer structure"]
impl crate::Writable for BlaiActTable13_10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_13_10 to value 0"]
impl crate::Resettable for BlaiActTable13_10Spec {}
