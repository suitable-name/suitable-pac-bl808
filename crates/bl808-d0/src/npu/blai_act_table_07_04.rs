#[doc = "Register `BLAI_ACT_TABLE_07_04` reader"]
pub type R = crate::R<BlaiActTable07_04Spec>;
#[doc = "Register `BLAI_ACT_TABLE_07_04` writer"]
pub type W = crate::W<BlaiActTable07_04Spec>;
#[doc = "Field `reg_act_table_04` reader - 8-bit activation value (Index 0x04)"]
pub type RegActTable04R = crate::FieldReader;
#[doc = "Field `reg_act_table_04` writer - 8-bit activation value (Index 0x04)"]
pub type RegActTable04W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_05` reader - 8-bit activation value (Index 0x05)"]
pub type RegActTable05R = crate::FieldReader;
#[doc = "Field `reg_act_table_05` writer - 8-bit activation value (Index 0x05)"]
pub type RegActTable05W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_06` reader - 8-bit activation value (Index 0x06)"]
pub type RegActTable06R = crate::FieldReader;
#[doc = "Field `reg_act_table_06` writer - 8-bit activation value (Index 0x06)"]
pub type RegActTable06W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_07` reader - 8-bit activation value (Index 0x07)"]
pub type RegActTable07R = crate::FieldReader;
#[doc = "Field `reg_act_table_07` writer - 8-bit activation value (Index 0x07)"]
pub type RegActTable07W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x04)"]
    #[inline(always)]
    pub fn reg_act_table_04(&self) -> RegActTable04R {
        RegActTable04R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x05)"]
    #[inline(always)]
    pub fn reg_act_table_05(&self) -> RegActTable05R {
        RegActTable05R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x06)"]
    #[inline(always)]
    pub fn reg_act_table_06(&self) -> RegActTable06R {
        RegActTable06R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x07)"]
    #[inline(always)]
    pub fn reg_act_table_07(&self) -> RegActTable07R {
        RegActTable07R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x04)"]
    #[inline(always)]
    pub fn reg_act_table_04(&mut self) -> RegActTable04W<'_, BlaiActTable07_04Spec> {
        RegActTable04W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x05)"]
    #[inline(always)]
    pub fn reg_act_table_05(&mut self) -> RegActTable05W<'_, BlaiActTable07_04Spec> {
        RegActTable05W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x06)"]
    #[inline(always)]
    pub fn reg_act_table_06(&mut self) -> RegActTable06W<'_, BlaiActTable07_04Spec> {
        RegActTable06W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x07)"]
    #[inline(always)]
    pub fn reg_act_table_07(&mut self) -> RegActTable07W<'_, BlaiActTable07_04Spec> {
        RegActTable07W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x04-0x07)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_07_04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_07_04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable07_04Spec;
impl crate::RegisterSpec for BlaiActTable07_04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_07_04::R`](R) reader structure"]
impl crate::Readable for BlaiActTable07_04Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_07_04::W`](W) writer structure"]
impl crate::Writable for BlaiActTable07_04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_07_04 to value 0"]
impl crate::Resettable for BlaiActTable07_04Spec {}
