#[doc = "Register `BLAI_ACT_TABLE_2B_28` reader"]
pub type R = crate::R<BlaiActTable2b28Spec>;
#[doc = "Register `BLAI_ACT_TABLE_2B_28` writer"]
pub type W = crate::W<BlaiActTable2b28Spec>;
#[doc = "Field `reg_act_table_28` reader - 8-bit activation value (Index 0x28)"]
pub type RegActTable28R = crate::FieldReader;
#[doc = "Field `reg_act_table_28` writer - 8-bit activation value (Index 0x28)"]
pub type RegActTable28W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_29` reader - 8-bit activation value (Index 0x29)"]
pub type RegActTable29R = crate::FieldReader;
#[doc = "Field `reg_act_table_29` writer - 8-bit activation value (Index 0x29)"]
pub type RegActTable29W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_2A` reader - 8-bit activation value (Index 0x2A)"]
pub type RegActTable2aR = crate::FieldReader;
#[doc = "Field `reg_act_table_2A` writer - 8-bit activation value (Index 0x2A)"]
pub type RegActTable2aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_2B` reader - 8-bit activation value (Index 0x2B)"]
pub type RegActTable2bR = crate::FieldReader;
#[doc = "Field `reg_act_table_2B` writer - 8-bit activation value (Index 0x2B)"]
pub type RegActTable2bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x28)"]
    #[inline(always)]
    pub fn reg_act_table_28(&self) -> RegActTable28R {
        RegActTable28R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x29)"]
    #[inline(always)]
    pub fn reg_act_table_29(&self) -> RegActTable29R {
        RegActTable29R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x2A)"]
    #[inline(always)]
    pub fn reg_act_table_2a(&self) -> RegActTable2aR {
        RegActTable2aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x2B)"]
    #[inline(always)]
    pub fn reg_act_table_2b(&self) -> RegActTable2bR {
        RegActTable2bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x28)"]
    #[inline(always)]
    pub fn reg_act_table_28(&mut self) -> RegActTable28W<'_, BlaiActTable2b28Spec> {
        RegActTable28W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x29)"]
    #[inline(always)]
    pub fn reg_act_table_29(&mut self) -> RegActTable29W<'_, BlaiActTable2b28Spec> {
        RegActTable29W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x2A)"]
    #[inline(always)]
    pub fn reg_act_table_2a(&mut self) -> RegActTable2aW<'_, BlaiActTable2b28Spec> {
        RegActTable2aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x2B)"]
    #[inline(always)]
    pub fn reg_act_table_2b(&mut self) -> RegActTable2bW<'_, BlaiActTable2b28Spec> {
        RegActTable2bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x28-0x2B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_2b_28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_2b_28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable2b28Spec;
impl crate::RegisterSpec for BlaiActTable2b28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_2b_28::R`](R) reader structure"]
impl crate::Readable for BlaiActTable2b28Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_2b_28::W`](W) writer structure"]
impl crate::Writable for BlaiActTable2b28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_2B_28 to value 0"]
impl crate::Resettable for BlaiActTable2b28Spec {}
