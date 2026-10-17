#[doc = "Register `BLAI_ACT_TABLE_6B_68` reader"]
pub type R = crate::R<BlaiActTable6b68Spec>;
#[doc = "Register `BLAI_ACT_TABLE_6B_68` writer"]
pub type W = crate::W<BlaiActTable6b68Spec>;
#[doc = "Field `reg_act_table_68` reader - 8-bit activation value (Index 0x68)"]
pub type RegActTable68R = crate::FieldReader;
#[doc = "Field `reg_act_table_68` writer - 8-bit activation value (Index 0x68)"]
pub type RegActTable68W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_69` reader - 8-bit activation value (Index 0x69)"]
pub type RegActTable69R = crate::FieldReader;
#[doc = "Field `reg_act_table_69` writer - 8-bit activation value (Index 0x69)"]
pub type RegActTable69W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_6A` reader - 8-bit activation value (Index 0x6A)"]
pub type RegActTable6aR = crate::FieldReader;
#[doc = "Field `reg_act_table_6A` writer - 8-bit activation value (Index 0x6A)"]
pub type RegActTable6aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_6B` reader - 8-bit activation value (Index 0x6B)"]
pub type RegActTable6bR = crate::FieldReader;
#[doc = "Field `reg_act_table_6B` writer - 8-bit activation value (Index 0x6B)"]
pub type RegActTable6bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x68)"]
    #[inline(always)]
    pub fn reg_act_table_68(&self) -> RegActTable68R {
        RegActTable68R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x69)"]
    #[inline(always)]
    pub fn reg_act_table_69(&self) -> RegActTable69R {
        RegActTable69R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x6A)"]
    #[inline(always)]
    pub fn reg_act_table_6a(&self) -> RegActTable6aR {
        RegActTable6aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x6B)"]
    #[inline(always)]
    pub fn reg_act_table_6b(&self) -> RegActTable6bR {
        RegActTable6bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x68)"]
    #[inline(always)]
    pub fn reg_act_table_68(&mut self) -> RegActTable68W<'_, BlaiActTable6b68Spec> {
        RegActTable68W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x69)"]
    #[inline(always)]
    pub fn reg_act_table_69(&mut self) -> RegActTable69W<'_, BlaiActTable6b68Spec> {
        RegActTable69W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x6A)"]
    #[inline(always)]
    pub fn reg_act_table_6a(&mut self) -> RegActTable6aW<'_, BlaiActTable6b68Spec> {
        RegActTable6aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x6B)"]
    #[inline(always)]
    pub fn reg_act_table_6b(&mut self) -> RegActTable6bW<'_, BlaiActTable6b68Spec> {
        RegActTable6bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x68-0x6B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_6b_68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_6b_68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable6b68Spec;
impl crate::RegisterSpec for BlaiActTable6b68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_6b_68::R`](R) reader structure"]
impl crate::Readable for BlaiActTable6b68Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_6b_68::W`](W) writer structure"]
impl crate::Writable for BlaiActTable6b68Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_6B_68 to value 0"]
impl crate::Resettable for BlaiActTable6b68Spec {}
