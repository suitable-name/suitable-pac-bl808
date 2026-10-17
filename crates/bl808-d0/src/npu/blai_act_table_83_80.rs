#[doc = "Register `BLAI_ACT_TABLE_83_80` reader"]
pub type R = crate::R<BlaiActTable83_80Spec>;
#[doc = "Register `BLAI_ACT_TABLE_83_80` writer"]
pub type W = crate::W<BlaiActTable83_80Spec>;
#[doc = "Field `reg_act_table_80` reader - 8-bit activation value (Index 0x80)"]
pub type RegActTable80R = crate::FieldReader;
#[doc = "Field `reg_act_table_80` writer - 8-bit activation value (Index 0x80)"]
pub type RegActTable80W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_81` reader - 8-bit activation value (Index 0x81)"]
pub type RegActTable81R = crate::FieldReader;
#[doc = "Field `reg_act_table_81` writer - 8-bit activation value (Index 0x81)"]
pub type RegActTable81W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_82` reader - 8-bit activation value (Index 0x82)"]
pub type RegActTable82R = crate::FieldReader;
#[doc = "Field `reg_act_table_82` writer - 8-bit activation value (Index 0x82)"]
pub type RegActTable82W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_83` reader - 8-bit activation value (Index 0x83)"]
pub type RegActTable83R = crate::FieldReader;
#[doc = "Field `reg_act_table_83` writer - 8-bit activation value (Index 0x83)"]
pub type RegActTable83W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x80)"]
    #[inline(always)]
    pub fn reg_act_table_80(&self) -> RegActTable80R {
        RegActTable80R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x81)"]
    #[inline(always)]
    pub fn reg_act_table_81(&self) -> RegActTable81R {
        RegActTable81R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x82)"]
    #[inline(always)]
    pub fn reg_act_table_82(&self) -> RegActTable82R {
        RegActTable82R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x83)"]
    #[inline(always)]
    pub fn reg_act_table_83(&self) -> RegActTable83R {
        RegActTable83R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x80)"]
    #[inline(always)]
    pub fn reg_act_table_80(&mut self) -> RegActTable80W<'_, BlaiActTable83_80Spec> {
        RegActTable80W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x81)"]
    #[inline(always)]
    pub fn reg_act_table_81(&mut self) -> RegActTable81W<'_, BlaiActTable83_80Spec> {
        RegActTable81W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x82)"]
    #[inline(always)]
    pub fn reg_act_table_82(&mut self) -> RegActTable82W<'_, BlaiActTable83_80Spec> {
        RegActTable82W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x83)"]
    #[inline(always)]
    pub fn reg_act_table_83(&mut self) -> RegActTable83W<'_, BlaiActTable83_80Spec> {
        RegActTable83W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x80-0x83)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_83_80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_83_80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable83_80Spec;
impl crate::RegisterSpec for BlaiActTable83_80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_83_80::R`](R) reader structure"]
impl crate::Readable for BlaiActTable83_80Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_83_80::W`](W) writer structure"]
impl crate::Writable for BlaiActTable83_80Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_83_80 to value 0"]
impl crate::Resettable for BlaiActTable83_80Spec {}
