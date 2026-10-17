#[doc = "Register `BLAI_ACT_TABLE_47_44` reader"]
pub type R = crate::R<BlaiActTable47_44Spec>;
#[doc = "Register `BLAI_ACT_TABLE_47_44` writer"]
pub type W = crate::W<BlaiActTable47_44Spec>;
#[doc = "Field `reg_act_table_44` reader - 8-bit activation value (Index 0x44)"]
pub type RegActTable44R = crate::FieldReader;
#[doc = "Field `reg_act_table_44` writer - 8-bit activation value (Index 0x44)"]
pub type RegActTable44W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_45` reader - 8-bit activation value (Index 0x45)"]
pub type RegActTable45R = crate::FieldReader;
#[doc = "Field `reg_act_table_45` writer - 8-bit activation value (Index 0x45)"]
pub type RegActTable45W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_46` reader - 8-bit activation value (Index 0x46)"]
pub type RegActTable46R = crate::FieldReader;
#[doc = "Field `reg_act_table_46` writer - 8-bit activation value (Index 0x46)"]
pub type RegActTable46W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_47` reader - 8-bit activation value (Index 0x47)"]
pub type RegActTable47R = crate::FieldReader;
#[doc = "Field `reg_act_table_47` writer - 8-bit activation value (Index 0x47)"]
pub type RegActTable47W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x44)"]
    #[inline(always)]
    pub fn reg_act_table_44(&self) -> RegActTable44R {
        RegActTable44R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x45)"]
    #[inline(always)]
    pub fn reg_act_table_45(&self) -> RegActTable45R {
        RegActTable45R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x46)"]
    #[inline(always)]
    pub fn reg_act_table_46(&self) -> RegActTable46R {
        RegActTable46R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x47)"]
    #[inline(always)]
    pub fn reg_act_table_47(&self) -> RegActTable47R {
        RegActTable47R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x44)"]
    #[inline(always)]
    pub fn reg_act_table_44(&mut self) -> RegActTable44W<'_, BlaiActTable47_44Spec> {
        RegActTable44W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x45)"]
    #[inline(always)]
    pub fn reg_act_table_45(&mut self) -> RegActTable45W<'_, BlaiActTable47_44Spec> {
        RegActTable45W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x46)"]
    #[inline(always)]
    pub fn reg_act_table_46(&mut self) -> RegActTable46W<'_, BlaiActTable47_44Spec> {
        RegActTable46W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x47)"]
    #[inline(always)]
    pub fn reg_act_table_47(&mut self) -> RegActTable47W<'_, BlaiActTable47_44Spec> {
        RegActTable47W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x44-0x47)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_47_44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_47_44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable47_44Spec;
impl crate::RegisterSpec for BlaiActTable47_44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_47_44::R`](R) reader structure"]
impl crate::Readable for BlaiActTable47_44Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_47_44::W`](W) writer structure"]
impl crate::Writable for BlaiActTable47_44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_47_44 to value 0"]
impl crate::Resettable for BlaiActTable47_44Spec {}
