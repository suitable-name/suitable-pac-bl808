#[doc = "Register `BLAI_ACT_TABLE_97_94` reader"]
pub type R = crate::R<BlaiActTable97_94Spec>;
#[doc = "Register `BLAI_ACT_TABLE_97_94` writer"]
pub type W = crate::W<BlaiActTable97_94Spec>;
#[doc = "Field `reg_act_table_94` reader - 8-bit activation value (Index 0x94)"]
pub type RegActTable94R = crate::FieldReader;
#[doc = "Field `reg_act_table_94` writer - 8-bit activation value (Index 0x94)"]
pub type RegActTable94W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_95` reader - 8-bit activation value (Index 0x95)"]
pub type RegActTable95R = crate::FieldReader;
#[doc = "Field `reg_act_table_95` writer - 8-bit activation value (Index 0x95)"]
pub type RegActTable95W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_96` reader - 8-bit activation value (Index 0x96)"]
pub type RegActTable96R = crate::FieldReader;
#[doc = "Field `reg_act_table_96` writer - 8-bit activation value (Index 0x96)"]
pub type RegActTable96W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_97` reader - 8-bit activation value (Index 0x97)"]
pub type RegActTable97R = crate::FieldReader;
#[doc = "Field `reg_act_table_97` writer - 8-bit activation value (Index 0x97)"]
pub type RegActTable97W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x94)"]
    #[inline(always)]
    pub fn reg_act_table_94(&self) -> RegActTable94R {
        RegActTable94R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x95)"]
    #[inline(always)]
    pub fn reg_act_table_95(&self) -> RegActTable95R {
        RegActTable95R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x96)"]
    #[inline(always)]
    pub fn reg_act_table_96(&self) -> RegActTable96R {
        RegActTable96R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x97)"]
    #[inline(always)]
    pub fn reg_act_table_97(&self) -> RegActTable97R {
        RegActTable97R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x94)"]
    #[inline(always)]
    pub fn reg_act_table_94(&mut self) -> RegActTable94W<'_, BlaiActTable97_94Spec> {
        RegActTable94W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x95)"]
    #[inline(always)]
    pub fn reg_act_table_95(&mut self) -> RegActTable95W<'_, BlaiActTable97_94Spec> {
        RegActTable95W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x96)"]
    #[inline(always)]
    pub fn reg_act_table_96(&mut self) -> RegActTable96W<'_, BlaiActTable97_94Spec> {
        RegActTable96W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x97)"]
    #[inline(always)]
    pub fn reg_act_table_97(&mut self) -> RegActTable97W<'_, BlaiActTable97_94Spec> {
        RegActTable97W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x94-0x97)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_97_94::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_97_94::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable97_94Spec;
impl crate::RegisterSpec for BlaiActTable97_94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_97_94::R`](R) reader structure"]
impl crate::Readable for BlaiActTable97_94Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_97_94::W`](W) writer structure"]
impl crate::Writable for BlaiActTable97_94Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_97_94 to value 0"]
impl crate::Resettable for BlaiActTable97_94Spec {}
