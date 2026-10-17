#[doc = "Register `BLAI_ACT_TABLE_73_70` reader"]
pub type R = crate::R<BlaiActTable73_70Spec>;
#[doc = "Register `BLAI_ACT_TABLE_73_70` writer"]
pub type W = crate::W<BlaiActTable73_70Spec>;
#[doc = "Field `reg_act_table_70` reader - 8-bit activation value (Index 0x70)"]
pub type RegActTable70R = crate::FieldReader;
#[doc = "Field `reg_act_table_70` writer - 8-bit activation value (Index 0x70)"]
pub type RegActTable70W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_71` reader - 8-bit activation value (Index 0x71)"]
pub type RegActTable71R = crate::FieldReader;
#[doc = "Field `reg_act_table_71` writer - 8-bit activation value (Index 0x71)"]
pub type RegActTable71W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_72` reader - 8-bit activation value (Index 0x72)"]
pub type RegActTable72R = crate::FieldReader;
#[doc = "Field `reg_act_table_72` writer - 8-bit activation value (Index 0x72)"]
pub type RegActTable72W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_73` reader - 8-bit activation value (Index 0x73)"]
pub type RegActTable73R = crate::FieldReader;
#[doc = "Field `reg_act_table_73` writer - 8-bit activation value (Index 0x73)"]
pub type RegActTable73W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x70)"]
    #[inline(always)]
    pub fn reg_act_table_70(&self) -> RegActTable70R {
        RegActTable70R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x71)"]
    #[inline(always)]
    pub fn reg_act_table_71(&self) -> RegActTable71R {
        RegActTable71R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x72)"]
    #[inline(always)]
    pub fn reg_act_table_72(&self) -> RegActTable72R {
        RegActTable72R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x73)"]
    #[inline(always)]
    pub fn reg_act_table_73(&self) -> RegActTable73R {
        RegActTable73R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x70)"]
    #[inline(always)]
    pub fn reg_act_table_70(&mut self) -> RegActTable70W<'_, BlaiActTable73_70Spec> {
        RegActTable70W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x71)"]
    #[inline(always)]
    pub fn reg_act_table_71(&mut self) -> RegActTable71W<'_, BlaiActTable73_70Spec> {
        RegActTable71W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x72)"]
    #[inline(always)]
    pub fn reg_act_table_72(&mut self) -> RegActTable72W<'_, BlaiActTable73_70Spec> {
        RegActTable72W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x73)"]
    #[inline(always)]
    pub fn reg_act_table_73(&mut self) -> RegActTable73W<'_, BlaiActTable73_70Spec> {
        RegActTable73W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x70-0x73)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_73_70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_73_70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable73_70Spec;
impl crate::RegisterSpec for BlaiActTable73_70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_73_70::R`](R) reader structure"]
impl crate::Readable for BlaiActTable73_70Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_73_70::W`](W) writer structure"]
impl crate::Writable for BlaiActTable73_70Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_73_70 to value 0"]
impl crate::Resettable for BlaiActTable73_70Spec {}
