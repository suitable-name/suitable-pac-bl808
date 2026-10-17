#[doc = "Register `BLAI_ACT_TABLE_27_24` reader"]
pub type R = crate::R<BlaiActTable27_24Spec>;
#[doc = "Register `BLAI_ACT_TABLE_27_24` writer"]
pub type W = crate::W<BlaiActTable27_24Spec>;
#[doc = "Field `reg_act_table_24` reader - 8-bit activation value (Index 0x24)"]
pub type RegActTable24R = crate::FieldReader;
#[doc = "Field `reg_act_table_24` writer - 8-bit activation value (Index 0x24)"]
pub type RegActTable24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_25` reader - 8-bit activation value (Index 0x25)"]
pub type RegActTable25R = crate::FieldReader;
#[doc = "Field `reg_act_table_25` writer - 8-bit activation value (Index 0x25)"]
pub type RegActTable25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_26` reader - 8-bit activation value (Index 0x26)"]
pub type RegActTable26R = crate::FieldReader;
#[doc = "Field `reg_act_table_26` writer - 8-bit activation value (Index 0x26)"]
pub type RegActTable26W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_27` reader - 8-bit activation value (Index 0x27)"]
pub type RegActTable27R = crate::FieldReader;
#[doc = "Field `reg_act_table_27` writer - 8-bit activation value (Index 0x27)"]
pub type RegActTable27W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x24)"]
    #[inline(always)]
    pub fn reg_act_table_24(&self) -> RegActTable24R {
        RegActTable24R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x25)"]
    #[inline(always)]
    pub fn reg_act_table_25(&self) -> RegActTable25R {
        RegActTable25R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x26)"]
    #[inline(always)]
    pub fn reg_act_table_26(&self) -> RegActTable26R {
        RegActTable26R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x27)"]
    #[inline(always)]
    pub fn reg_act_table_27(&self) -> RegActTable27R {
        RegActTable27R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x24)"]
    #[inline(always)]
    pub fn reg_act_table_24(&mut self) -> RegActTable24W<'_, BlaiActTable27_24Spec> {
        RegActTable24W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x25)"]
    #[inline(always)]
    pub fn reg_act_table_25(&mut self) -> RegActTable25W<'_, BlaiActTable27_24Spec> {
        RegActTable25W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x26)"]
    #[inline(always)]
    pub fn reg_act_table_26(&mut self) -> RegActTable26W<'_, BlaiActTable27_24Spec> {
        RegActTable26W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x27)"]
    #[inline(always)]
    pub fn reg_act_table_27(&mut self) -> RegActTable27W<'_, BlaiActTable27_24Spec> {
        RegActTable27W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x24-0x27)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_27_24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_27_24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable27_24Spec;
impl crate::RegisterSpec for BlaiActTable27_24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_27_24::R`](R) reader structure"]
impl crate::Readable for BlaiActTable27_24Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_27_24::W`](W) writer structure"]
impl crate::Writable for BlaiActTable27_24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_27_24 to value 0"]
impl crate::Resettable for BlaiActTable27_24Spec {}
