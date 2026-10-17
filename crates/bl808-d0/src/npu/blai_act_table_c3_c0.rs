#[doc = "Register `BLAI_ACT_TABLE_C3_C0` reader"]
pub type R = crate::R<BlaiActTableC3C0Spec>;
#[doc = "Register `BLAI_ACT_TABLE_C3_C0` writer"]
pub type W = crate::W<BlaiActTableC3C0Spec>;
#[doc = "Field `reg_act_table_C0` reader - 8-bit activation value (Index 0xC0)"]
pub type RegActTableC0R = crate::FieldReader;
#[doc = "Field `reg_act_table_C0` writer - 8-bit activation value (Index 0xC0)"]
pub type RegActTableC0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C1` reader - 8-bit activation value (Index 0xC1)"]
pub type RegActTableC1R = crate::FieldReader;
#[doc = "Field `reg_act_table_C1` writer - 8-bit activation value (Index 0xC1)"]
pub type RegActTableC1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C2` reader - 8-bit activation value (Index 0xC2)"]
pub type RegActTableC2R = crate::FieldReader;
#[doc = "Field `reg_act_table_C2` writer - 8-bit activation value (Index 0xC2)"]
pub type RegActTableC2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C3` reader - 8-bit activation value (Index 0xC3)"]
pub type RegActTableC3R = crate::FieldReader;
#[doc = "Field `reg_act_table_C3` writer - 8-bit activation value (Index 0xC3)"]
pub type RegActTableC3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xC0)"]
    #[inline(always)]
    pub fn reg_act_table_c0(&self) -> RegActTableC0R {
        RegActTableC0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xC1)"]
    #[inline(always)]
    pub fn reg_act_table_c1(&self) -> RegActTableC1R {
        RegActTableC1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xC2)"]
    #[inline(always)]
    pub fn reg_act_table_c2(&self) -> RegActTableC2R {
        RegActTableC2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xC3)"]
    #[inline(always)]
    pub fn reg_act_table_c3(&self) -> RegActTableC3R {
        RegActTableC3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xC0)"]
    #[inline(always)]
    pub fn reg_act_table_c0(&mut self) -> RegActTableC0W<'_, BlaiActTableC3C0Spec> {
        RegActTableC0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xC1)"]
    #[inline(always)]
    pub fn reg_act_table_c1(&mut self) -> RegActTableC1W<'_, BlaiActTableC3C0Spec> {
        RegActTableC1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xC2)"]
    #[inline(always)]
    pub fn reg_act_table_c2(&mut self) -> RegActTableC2W<'_, BlaiActTableC3C0Spec> {
        RegActTableC2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xC3)"]
    #[inline(always)]
    pub fn reg_act_table_c3(&mut self) -> RegActTableC3W<'_, BlaiActTableC3C0Spec> {
        RegActTableC3W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xC0-0xC3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_c3_c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_c3_c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableC3C0Spec;
impl crate::RegisterSpec for BlaiActTableC3C0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_c3_c0::R`](R) reader structure"]
impl crate::Readable for BlaiActTableC3C0Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_c3_c0::W`](W) writer structure"]
impl crate::Writable for BlaiActTableC3C0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_C3_C0 to value 0"]
impl crate::Resettable for BlaiActTableC3C0Spec {}
