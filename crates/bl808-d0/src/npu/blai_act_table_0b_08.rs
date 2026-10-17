#[doc = "Register `BLAI_ACT_TABLE_0B_08` reader"]
pub type R = crate::R<BlaiActTable0b08Spec>;
#[doc = "Register `BLAI_ACT_TABLE_0B_08` writer"]
pub type W = crate::W<BlaiActTable0b08Spec>;
#[doc = "Field `reg_act_table_08` reader - 8-bit activation value (Index 0x08)"]
pub type RegActTable08R = crate::FieldReader;
#[doc = "Field `reg_act_table_08` writer - 8-bit activation value (Index 0x08)"]
pub type RegActTable08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_09` reader - 8-bit activation value (Index 0x09)"]
pub type RegActTable09R = crate::FieldReader;
#[doc = "Field `reg_act_table_09` writer - 8-bit activation value (Index 0x09)"]
pub type RegActTable09W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_0A` reader - 8-bit activation value (Index 0x0A)"]
pub type RegActTable0aR = crate::FieldReader;
#[doc = "Field `reg_act_table_0A` writer - 8-bit activation value (Index 0x0A)"]
pub type RegActTable0aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_0B` reader - 8-bit activation value (Index 0x0B)"]
pub type RegActTable0bR = crate::FieldReader;
#[doc = "Field `reg_act_table_0B` writer - 8-bit activation value (Index 0x0B)"]
pub type RegActTable0bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x08)"]
    #[inline(always)]
    pub fn reg_act_table_08(&self) -> RegActTable08R {
        RegActTable08R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x09)"]
    #[inline(always)]
    pub fn reg_act_table_09(&self) -> RegActTable09R {
        RegActTable09R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x0A)"]
    #[inline(always)]
    pub fn reg_act_table_0a(&self) -> RegActTable0aR {
        RegActTable0aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x0B)"]
    #[inline(always)]
    pub fn reg_act_table_0b(&self) -> RegActTable0bR {
        RegActTable0bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x08)"]
    #[inline(always)]
    pub fn reg_act_table_08(&mut self) -> RegActTable08W<'_, BlaiActTable0b08Spec> {
        RegActTable08W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x09)"]
    #[inline(always)]
    pub fn reg_act_table_09(&mut self) -> RegActTable09W<'_, BlaiActTable0b08Spec> {
        RegActTable09W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x0A)"]
    #[inline(always)]
    pub fn reg_act_table_0a(&mut self) -> RegActTable0aW<'_, BlaiActTable0b08Spec> {
        RegActTable0aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x0B)"]
    #[inline(always)]
    pub fn reg_act_table_0b(&mut self) -> RegActTable0bW<'_, BlaiActTable0b08Spec> {
        RegActTable0bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x08-0x0B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_0b_08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_0b_08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable0b08Spec;
impl crate::RegisterSpec for BlaiActTable0b08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_0b_08::R`](R) reader structure"]
impl crate::Readable for BlaiActTable0b08Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_0b_08::W`](W) writer structure"]
impl crate::Writable for BlaiActTable0b08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_0B_08 to value 0"]
impl crate::Resettable for BlaiActTable0b08Spec {}
