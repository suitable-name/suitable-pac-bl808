#[doc = "Register `BLAI_ACT_TABLE_43_40` reader"]
pub type R = crate::R<BlaiActTable43_40Spec>;
#[doc = "Register `BLAI_ACT_TABLE_43_40` writer"]
pub type W = crate::W<BlaiActTable43_40Spec>;
#[doc = "Field `reg_act_table_40` reader - 8-bit activation value (Index 0x40)"]
pub type RegActTable40R = crate::FieldReader;
#[doc = "Field `reg_act_table_40` writer - 8-bit activation value (Index 0x40)"]
pub type RegActTable40W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_41` reader - 8-bit activation value (Index 0x41)"]
pub type RegActTable41R = crate::FieldReader;
#[doc = "Field `reg_act_table_41` writer - 8-bit activation value (Index 0x41)"]
pub type RegActTable41W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_42` reader - 8-bit activation value (Index 0x42)"]
pub type RegActTable42R = crate::FieldReader;
#[doc = "Field `reg_act_table_42` writer - 8-bit activation value (Index 0x42)"]
pub type RegActTable42W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_43` reader - 8-bit activation value (Index 0x43)"]
pub type RegActTable43R = crate::FieldReader;
#[doc = "Field `reg_act_table_43` writer - 8-bit activation value (Index 0x43)"]
pub type RegActTable43W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x40)"]
    #[inline(always)]
    pub fn reg_act_table_40(&self) -> RegActTable40R {
        RegActTable40R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x41)"]
    #[inline(always)]
    pub fn reg_act_table_41(&self) -> RegActTable41R {
        RegActTable41R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x42)"]
    #[inline(always)]
    pub fn reg_act_table_42(&self) -> RegActTable42R {
        RegActTable42R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x43)"]
    #[inline(always)]
    pub fn reg_act_table_43(&self) -> RegActTable43R {
        RegActTable43R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x40)"]
    #[inline(always)]
    pub fn reg_act_table_40(&mut self) -> RegActTable40W<'_, BlaiActTable43_40Spec> {
        RegActTable40W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x41)"]
    #[inline(always)]
    pub fn reg_act_table_41(&mut self) -> RegActTable41W<'_, BlaiActTable43_40Spec> {
        RegActTable41W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x42)"]
    #[inline(always)]
    pub fn reg_act_table_42(&mut self) -> RegActTable42W<'_, BlaiActTable43_40Spec> {
        RegActTable42W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x43)"]
    #[inline(always)]
    pub fn reg_act_table_43(&mut self) -> RegActTable43W<'_, BlaiActTable43_40Spec> {
        RegActTable43W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x40-0x43)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_43_40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_43_40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable43_40Spec;
impl crate::RegisterSpec for BlaiActTable43_40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_43_40::R`](R) reader structure"]
impl crate::Readable for BlaiActTable43_40Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_43_40::W`](W) writer structure"]
impl crate::Writable for BlaiActTable43_40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_43_40 to value 0"]
impl crate::Resettable for BlaiActTable43_40Spec {}
