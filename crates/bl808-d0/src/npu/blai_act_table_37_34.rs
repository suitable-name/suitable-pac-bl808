#[doc = "Register `BLAI_ACT_TABLE_37_34` reader"]
pub type R = crate::R<BlaiActTable37_34Spec>;
#[doc = "Register `BLAI_ACT_TABLE_37_34` writer"]
pub type W = crate::W<BlaiActTable37_34Spec>;
#[doc = "Field `reg_act_table_34` reader - 8-bit activation value (Index 0x34)"]
pub type RegActTable34R = crate::FieldReader;
#[doc = "Field `reg_act_table_34` writer - 8-bit activation value (Index 0x34)"]
pub type RegActTable34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_35` reader - 8-bit activation value (Index 0x35)"]
pub type RegActTable35R = crate::FieldReader;
#[doc = "Field `reg_act_table_35` writer - 8-bit activation value (Index 0x35)"]
pub type RegActTable35W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_36` reader - 8-bit activation value (Index 0x36)"]
pub type RegActTable36R = crate::FieldReader;
#[doc = "Field `reg_act_table_36` writer - 8-bit activation value (Index 0x36)"]
pub type RegActTable36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_37` reader - 8-bit activation value (Index 0x37)"]
pub type RegActTable37R = crate::FieldReader;
#[doc = "Field `reg_act_table_37` writer - 8-bit activation value (Index 0x37)"]
pub type RegActTable37W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x34)"]
    #[inline(always)]
    pub fn reg_act_table_34(&self) -> RegActTable34R {
        RegActTable34R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x35)"]
    #[inline(always)]
    pub fn reg_act_table_35(&self) -> RegActTable35R {
        RegActTable35R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x36)"]
    #[inline(always)]
    pub fn reg_act_table_36(&self) -> RegActTable36R {
        RegActTable36R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x37)"]
    #[inline(always)]
    pub fn reg_act_table_37(&self) -> RegActTable37R {
        RegActTable37R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x34)"]
    #[inline(always)]
    pub fn reg_act_table_34(&mut self) -> RegActTable34W<'_, BlaiActTable37_34Spec> {
        RegActTable34W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x35)"]
    #[inline(always)]
    pub fn reg_act_table_35(&mut self) -> RegActTable35W<'_, BlaiActTable37_34Spec> {
        RegActTable35W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x36)"]
    #[inline(always)]
    pub fn reg_act_table_36(&mut self) -> RegActTable36W<'_, BlaiActTable37_34Spec> {
        RegActTable36W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x37)"]
    #[inline(always)]
    pub fn reg_act_table_37(&mut self) -> RegActTable37W<'_, BlaiActTable37_34Spec> {
        RegActTable37W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x34-0x37)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_37_34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_37_34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable37_34Spec;
impl crate::RegisterSpec for BlaiActTable37_34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_37_34::R`](R) reader structure"]
impl crate::Readable for BlaiActTable37_34Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_37_34::W`](W) writer structure"]
impl crate::Writable for BlaiActTable37_34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_37_34 to value 0"]
impl crate::Resettable for BlaiActTable37_34Spec {}
