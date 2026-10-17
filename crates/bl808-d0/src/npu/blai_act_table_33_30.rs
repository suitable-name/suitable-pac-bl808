#[doc = "Register `BLAI_ACT_TABLE_33_30` reader"]
pub type R = crate::R<BlaiActTable33_30Spec>;
#[doc = "Register `BLAI_ACT_TABLE_33_30` writer"]
pub type W = crate::W<BlaiActTable33_30Spec>;
#[doc = "Field `reg_act_table_30` reader - 8-bit activation value (Index 0x30)"]
pub type RegActTable30R = crate::FieldReader;
#[doc = "Field `reg_act_table_30` writer - 8-bit activation value (Index 0x30)"]
pub type RegActTable30W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_31` reader - 8-bit activation value (Index 0x31)"]
pub type RegActTable31R = crate::FieldReader;
#[doc = "Field `reg_act_table_31` writer - 8-bit activation value (Index 0x31)"]
pub type RegActTable31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_32` reader - 8-bit activation value (Index 0x32)"]
pub type RegActTable32R = crate::FieldReader;
#[doc = "Field `reg_act_table_32` writer - 8-bit activation value (Index 0x32)"]
pub type RegActTable32W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_33` reader - 8-bit activation value (Index 0x33)"]
pub type RegActTable33R = crate::FieldReader;
#[doc = "Field `reg_act_table_33` writer - 8-bit activation value (Index 0x33)"]
pub type RegActTable33W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x30)"]
    #[inline(always)]
    pub fn reg_act_table_30(&self) -> RegActTable30R {
        RegActTable30R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x31)"]
    #[inline(always)]
    pub fn reg_act_table_31(&self) -> RegActTable31R {
        RegActTable31R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x32)"]
    #[inline(always)]
    pub fn reg_act_table_32(&self) -> RegActTable32R {
        RegActTable32R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x33)"]
    #[inline(always)]
    pub fn reg_act_table_33(&self) -> RegActTable33R {
        RegActTable33R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x30)"]
    #[inline(always)]
    pub fn reg_act_table_30(&mut self) -> RegActTable30W<'_, BlaiActTable33_30Spec> {
        RegActTable30W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x31)"]
    #[inline(always)]
    pub fn reg_act_table_31(&mut self) -> RegActTable31W<'_, BlaiActTable33_30Spec> {
        RegActTable31W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x32)"]
    #[inline(always)]
    pub fn reg_act_table_32(&mut self) -> RegActTable32W<'_, BlaiActTable33_30Spec> {
        RegActTable32W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x33)"]
    #[inline(always)]
    pub fn reg_act_table_33(&mut self) -> RegActTable33W<'_, BlaiActTable33_30Spec> {
        RegActTable33W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x30-0x33)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_33_30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_33_30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable33_30Spec;
impl crate::RegisterSpec for BlaiActTable33_30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_33_30::R`](R) reader structure"]
impl crate::Readable for BlaiActTable33_30Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_33_30::W`](W) writer structure"]
impl crate::Writable for BlaiActTable33_30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_33_30 to value 0"]
impl crate::Resettable for BlaiActTable33_30Spec {}
