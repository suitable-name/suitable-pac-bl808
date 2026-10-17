#[doc = "Register `BLAI_ACT_TABLE_17_14` reader"]
pub type R = crate::R<BlaiActTable17_14Spec>;
#[doc = "Register `BLAI_ACT_TABLE_17_14` writer"]
pub type W = crate::W<BlaiActTable17_14Spec>;
#[doc = "Field `reg_act_table_14` reader - 8-bit activation value (Index 0x14)"]
pub type RegActTable14R = crate::FieldReader;
#[doc = "Field `reg_act_table_14` writer - 8-bit activation value (Index 0x14)"]
pub type RegActTable14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_15` reader - 8-bit activation value (Index 0x15)"]
pub type RegActTable15R = crate::FieldReader;
#[doc = "Field `reg_act_table_15` writer - 8-bit activation value (Index 0x15)"]
pub type RegActTable15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_16` reader - 8-bit activation value (Index 0x16)"]
pub type RegActTable16R = crate::FieldReader;
#[doc = "Field `reg_act_table_16` writer - 8-bit activation value (Index 0x16)"]
pub type RegActTable16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_17` reader - 8-bit activation value (Index 0x17)"]
pub type RegActTable17R = crate::FieldReader;
#[doc = "Field `reg_act_table_17` writer - 8-bit activation value (Index 0x17)"]
pub type RegActTable17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x14)"]
    #[inline(always)]
    pub fn reg_act_table_14(&self) -> RegActTable14R {
        RegActTable14R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x15)"]
    #[inline(always)]
    pub fn reg_act_table_15(&self) -> RegActTable15R {
        RegActTable15R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x16)"]
    #[inline(always)]
    pub fn reg_act_table_16(&self) -> RegActTable16R {
        RegActTable16R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x17)"]
    #[inline(always)]
    pub fn reg_act_table_17(&self) -> RegActTable17R {
        RegActTable17R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x14)"]
    #[inline(always)]
    pub fn reg_act_table_14(&mut self) -> RegActTable14W<'_, BlaiActTable17_14Spec> {
        RegActTable14W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x15)"]
    #[inline(always)]
    pub fn reg_act_table_15(&mut self) -> RegActTable15W<'_, BlaiActTable17_14Spec> {
        RegActTable15W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x16)"]
    #[inline(always)]
    pub fn reg_act_table_16(&mut self) -> RegActTable16W<'_, BlaiActTable17_14Spec> {
        RegActTable16W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x17)"]
    #[inline(always)]
    pub fn reg_act_table_17(&mut self) -> RegActTable17W<'_, BlaiActTable17_14Spec> {
        RegActTable17W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x14-0x17)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_17_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_17_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable17_14Spec;
impl crate::RegisterSpec for BlaiActTable17_14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_17_14::R`](R) reader structure"]
impl crate::Readable for BlaiActTable17_14Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_17_14::W`](W) writer structure"]
impl crate::Writable for BlaiActTable17_14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_17_14 to value 0"]
impl crate::Resettable for BlaiActTable17_14Spec {}
