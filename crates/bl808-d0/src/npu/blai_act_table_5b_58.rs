#[doc = "Register `BLAI_ACT_TABLE_5B_58` reader"]
pub type R = crate::R<BlaiActTable5b58Spec>;
#[doc = "Register `BLAI_ACT_TABLE_5B_58` writer"]
pub type W = crate::W<BlaiActTable5b58Spec>;
#[doc = "Field `reg_act_table_58` reader - 8-bit activation value (Index 0x58)"]
pub type RegActTable58R = crate::FieldReader;
#[doc = "Field `reg_act_table_58` writer - 8-bit activation value (Index 0x58)"]
pub type RegActTable58W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_59` reader - 8-bit activation value (Index 0x59)"]
pub type RegActTable59R = crate::FieldReader;
#[doc = "Field `reg_act_table_59` writer - 8-bit activation value (Index 0x59)"]
pub type RegActTable59W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_5A` reader - 8-bit activation value (Index 0x5A)"]
pub type RegActTable5aR = crate::FieldReader;
#[doc = "Field `reg_act_table_5A` writer - 8-bit activation value (Index 0x5A)"]
pub type RegActTable5aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_5B` reader - 8-bit activation value (Index 0x5B)"]
pub type RegActTable5bR = crate::FieldReader;
#[doc = "Field `reg_act_table_5B` writer - 8-bit activation value (Index 0x5B)"]
pub type RegActTable5bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x58)"]
    #[inline(always)]
    pub fn reg_act_table_58(&self) -> RegActTable58R {
        RegActTable58R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x59)"]
    #[inline(always)]
    pub fn reg_act_table_59(&self) -> RegActTable59R {
        RegActTable59R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x5A)"]
    #[inline(always)]
    pub fn reg_act_table_5a(&self) -> RegActTable5aR {
        RegActTable5aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x5B)"]
    #[inline(always)]
    pub fn reg_act_table_5b(&self) -> RegActTable5bR {
        RegActTable5bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x58)"]
    #[inline(always)]
    pub fn reg_act_table_58(&mut self) -> RegActTable58W<'_, BlaiActTable5b58Spec> {
        RegActTable58W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x59)"]
    #[inline(always)]
    pub fn reg_act_table_59(&mut self) -> RegActTable59W<'_, BlaiActTable5b58Spec> {
        RegActTable59W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x5A)"]
    #[inline(always)]
    pub fn reg_act_table_5a(&mut self) -> RegActTable5aW<'_, BlaiActTable5b58Spec> {
        RegActTable5aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x5B)"]
    #[inline(always)]
    pub fn reg_act_table_5b(&mut self) -> RegActTable5bW<'_, BlaiActTable5b58Spec> {
        RegActTable5bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x58-0x5B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_5b_58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_5b_58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable5b58Spec;
impl crate::RegisterSpec for BlaiActTable5b58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_5b_58::R`](R) reader structure"]
impl crate::Readable for BlaiActTable5b58Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_5b_58::W`](W) writer structure"]
impl crate::Writable for BlaiActTable5b58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_5B_58 to value 0"]
impl crate::Resettable for BlaiActTable5b58Spec {}
