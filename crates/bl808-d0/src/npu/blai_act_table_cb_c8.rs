#[doc = "Register `BLAI_ACT_TABLE_CB_C8` reader"]
pub type R = crate::R<BlaiActTableCbC8Spec>;
#[doc = "Register `BLAI_ACT_TABLE_CB_C8` writer"]
pub type W = crate::W<BlaiActTableCbC8Spec>;
#[doc = "Field `reg_act_table_C8` reader - 8-bit activation value (Index 0xC8)"]
pub type RegActTableC8R = crate::FieldReader;
#[doc = "Field `reg_act_table_C8` writer - 8-bit activation value (Index 0xC8)"]
pub type RegActTableC8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_C9` reader - 8-bit activation value (Index 0xC9)"]
pub type RegActTableC9R = crate::FieldReader;
#[doc = "Field `reg_act_table_C9` writer - 8-bit activation value (Index 0xC9)"]
pub type RegActTableC9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_CA` reader - 8-bit activation value (Index 0xCA)"]
pub type RegActTableCaR = crate::FieldReader;
#[doc = "Field `reg_act_table_CA` writer - 8-bit activation value (Index 0xCA)"]
pub type RegActTableCaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_CB` reader - 8-bit activation value (Index 0xCB)"]
pub type RegActTableCbR = crate::FieldReader;
#[doc = "Field `reg_act_table_CB` writer - 8-bit activation value (Index 0xCB)"]
pub type RegActTableCbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xC8)"]
    #[inline(always)]
    pub fn reg_act_table_c8(&self) -> RegActTableC8R {
        RegActTableC8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xC9)"]
    #[inline(always)]
    pub fn reg_act_table_c9(&self) -> RegActTableC9R {
        RegActTableC9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xCA)"]
    #[inline(always)]
    pub fn reg_act_table_ca(&self) -> RegActTableCaR {
        RegActTableCaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xCB)"]
    #[inline(always)]
    pub fn reg_act_table_cb(&self) -> RegActTableCbR {
        RegActTableCbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xC8)"]
    #[inline(always)]
    pub fn reg_act_table_c8(&mut self) -> RegActTableC8W<'_, BlaiActTableCbC8Spec> {
        RegActTableC8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xC9)"]
    #[inline(always)]
    pub fn reg_act_table_c9(&mut self) -> RegActTableC9W<'_, BlaiActTableCbC8Spec> {
        RegActTableC9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xCA)"]
    #[inline(always)]
    pub fn reg_act_table_ca(&mut self) -> RegActTableCaW<'_, BlaiActTableCbC8Spec> {
        RegActTableCaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xCB)"]
    #[inline(always)]
    pub fn reg_act_table_cb(&mut self) -> RegActTableCbW<'_, BlaiActTableCbC8Spec> {
        RegActTableCbW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xC8-0xCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_cb_c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_cb_c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableCbC8Spec;
impl crate::RegisterSpec for BlaiActTableCbC8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_cb_c8::R`](R) reader structure"]
impl crate::Readable for BlaiActTableCbC8Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_cb_c8::W`](W) writer structure"]
impl crate::Writable for BlaiActTableCbC8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_CB_C8 to value 0"]
impl crate::Resettable for BlaiActTableCbC8Spec {}
