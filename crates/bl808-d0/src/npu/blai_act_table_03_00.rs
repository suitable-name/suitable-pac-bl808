#[doc = "Register `BLAI_ACT_TABLE_03_00` reader"]
pub type R = crate::R<BlaiActTable03_00Spec>;
#[doc = "Register `BLAI_ACT_TABLE_03_00` writer"]
pub type W = crate::W<BlaiActTable03_00Spec>;
#[doc = "Field `reg_act_table_00` reader - 8-bit activation value (Index 0x00)"]
pub type RegActTable00R = crate::FieldReader;
#[doc = "Field `reg_act_table_00` writer - 8-bit activation value (Index 0x00)"]
pub type RegActTable00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_01` reader - 8-bit activation value (Index 0x01)"]
pub type RegActTable01R = crate::FieldReader;
#[doc = "Field `reg_act_table_01` writer - 8-bit activation value (Index 0x01)"]
pub type RegActTable01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_02` reader - 8-bit activation value (Index 0x02)"]
pub type RegActTable02R = crate::FieldReader;
#[doc = "Field `reg_act_table_02` writer - 8-bit activation value (Index 0x02)"]
pub type RegActTable02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_03` reader - 8-bit activation value (Index 0x03)"]
pub type RegActTable03R = crate::FieldReader;
#[doc = "Field `reg_act_table_03` writer - 8-bit activation value (Index 0x03)"]
pub type RegActTable03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x00)"]
    #[inline(always)]
    pub fn reg_act_table_00(&self) -> RegActTable00R {
        RegActTable00R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x01)"]
    #[inline(always)]
    pub fn reg_act_table_01(&self) -> RegActTable01R {
        RegActTable01R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x02)"]
    #[inline(always)]
    pub fn reg_act_table_02(&self) -> RegActTable02R {
        RegActTable02R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x03)"]
    #[inline(always)]
    pub fn reg_act_table_03(&self) -> RegActTable03R {
        RegActTable03R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x00)"]
    #[inline(always)]
    pub fn reg_act_table_00(&mut self) -> RegActTable00W<'_, BlaiActTable03_00Spec> {
        RegActTable00W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x01)"]
    #[inline(always)]
    pub fn reg_act_table_01(&mut self) -> RegActTable01W<'_, BlaiActTable03_00Spec> {
        RegActTable01W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x02)"]
    #[inline(always)]
    pub fn reg_act_table_02(&mut self) -> RegActTable02W<'_, BlaiActTable03_00Spec> {
        RegActTable02W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x03)"]
    #[inline(always)]
    pub fn reg_act_table_03(&mut self) -> RegActTable03W<'_, BlaiActTable03_00Spec> {
        RegActTable03W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x00-0x03)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_03_00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_03_00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable03_00Spec;
impl crate::RegisterSpec for BlaiActTable03_00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_03_00::R`](R) reader structure"]
impl crate::Readable for BlaiActTable03_00Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_03_00::W`](W) writer structure"]
impl crate::Writable for BlaiActTable03_00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_03_00 to value 0"]
impl crate::Resettable for BlaiActTable03_00Spec {}
