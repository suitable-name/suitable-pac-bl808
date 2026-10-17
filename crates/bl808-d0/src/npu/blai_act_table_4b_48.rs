#[doc = "Register `BLAI_ACT_TABLE_4B_48` reader"]
pub type R = crate::R<BlaiActTable4b48Spec>;
#[doc = "Register `BLAI_ACT_TABLE_4B_48` writer"]
pub type W = crate::W<BlaiActTable4b48Spec>;
#[doc = "Field `reg_act_table_48` reader - 8-bit activation value (Index 0x48)"]
pub type RegActTable48R = crate::FieldReader;
#[doc = "Field `reg_act_table_48` writer - 8-bit activation value (Index 0x48)"]
pub type RegActTable48W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_49` reader - 8-bit activation value (Index 0x49)"]
pub type RegActTable49R = crate::FieldReader;
#[doc = "Field `reg_act_table_49` writer - 8-bit activation value (Index 0x49)"]
pub type RegActTable49W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_4A` reader - 8-bit activation value (Index 0x4A)"]
pub type RegActTable4aR = crate::FieldReader;
#[doc = "Field `reg_act_table_4A` writer - 8-bit activation value (Index 0x4A)"]
pub type RegActTable4aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_4B` reader - 8-bit activation value (Index 0x4B)"]
pub type RegActTable4bR = crate::FieldReader;
#[doc = "Field `reg_act_table_4B` writer - 8-bit activation value (Index 0x4B)"]
pub type RegActTable4bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x48)"]
    #[inline(always)]
    pub fn reg_act_table_48(&self) -> RegActTable48R {
        RegActTable48R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x49)"]
    #[inline(always)]
    pub fn reg_act_table_49(&self) -> RegActTable49R {
        RegActTable49R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x4A)"]
    #[inline(always)]
    pub fn reg_act_table_4a(&self) -> RegActTable4aR {
        RegActTable4aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x4B)"]
    #[inline(always)]
    pub fn reg_act_table_4b(&self) -> RegActTable4bR {
        RegActTable4bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x48)"]
    #[inline(always)]
    pub fn reg_act_table_48(&mut self) -> RegActTable48W<'_, BlaiActTable4b48Spec> {
        RegActTable48W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x49)"]
    #[inline(always)]
    pub fn reg_act_table_49(&mut self) -> RegActTable49W<'_, BlaiActTable4b48Spec> {
        RegActTable49W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x4A)"]
    #[inline(always)]
    pub fn reg_act_table_4a(&mut self) -> RegActTable4aW<'_, BlaiActTable4b48Spec> {
        RegActTable4aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x4B)"]
    #[inline(always)]
    pub fn reg_act_table_4b(&mut self) -> RegActTable4bW<'_, BlaiActTable4b48Spec> {
        RegActTable4bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x48-0x4B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_4b_48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_4b_48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable4b48Spec;
impl crate::RegisterSpec for BlaiActTable4b48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_4b_48::R`](R) reader structure"]
impl crate::Readable for BlaiActTable4b48Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_4b_48::W`](W) writer structure"]
impl crate::Writable for BlaiActTable4b48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_4B_48 to value 0"]
impl crate::Resettable for BlaiActTable4b48Spec {}
