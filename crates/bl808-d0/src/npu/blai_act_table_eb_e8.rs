#[doc = "Register `BLAI_ACT_TABLE_EB_E8` reader"]
pub type R = crate::R<BlaiActTableEbE8Spec>;
#[doc = "Register `BLAI_ACT_TABLE_EB_E8` writer"]
pub type W = crate::W<BlaiActTableEbE8Spec>;
#[doc = "Field `reg_act_table_E8` reader - 8-bit activation value (Index 0xE8)"]
pub type RegActTableE8R = crate::FieldReader;
#[doc = "Field `reg_act_table_E8` writer - 8-bit activation value (Index 0xE8)"]
pub type RegActTableE8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_E9` reader - 8-bit activation value (Index 0xE9)"]
pub type RegActTableE9R = crate::FieldReader;
#[doc = "Field `reg_act_table_E9` writer - 8-bit activation value (Index 0xE9)"]
pub type RegActTableE9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_EA` reader - 8-bit activation value (Index 0xEA)"]
pub type RegActTableEaR = crate::FieldReader;
#[doc = "Field `reg_act_table_EA` writer - 8-bit activation value (Index 0xEA)"]
pub type RegActTableEaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_EB` reader - 8-bit activation value (Index 0xEB)"]
pub type RegActTableEbR = crate::FieldReader;
#[doc = "Field `reg_act_table_EB` writer - 8-bit activation value (Index 0xEB)"]
pub type RegActTableEbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xE8)"]
    #[inline(always)]
    pub fn reg_act_table_e8(&self) -> RegActTableE8R {
        RegActTableE8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xE9)"]
    #[inline(always)]
    pub fn reg_act_table_e9(&self) -> RegActTableE9R {
        RegActTableE9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xEA)"]
    #[inline(always)]
    pub fn reg_act_table_ea(&self) -> RegActTableEaR {
        RegActTableEaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xEB)"]
    #[inline(always)]
    pub fn reg_act_table_eb(&self) -> RegActTableEbR {
        RegActTableEbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xE8)"]
    #[inline(always)]
    pub fn reg_act_table_e8(&mut self) -> RegActTableE8W<'_, BlaiActTableEbE8Spec> {
        RegActTableE8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xE9)"]
    #[inline(always)]
    pub fn reg_act_table_e9(&mut self) -> RegActTableE9W<'_, BlaiActTableEbE8Spec> {
        RegActTableE9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xEA)"]
    #[inline(always)]
    pub fn reg_act_table_ea(&mut self) -> RegActTableEaW<'_, BlaiActTableEbE8Spec> {
        RegActTableEaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xEB)"]
    #[inline(always)]
    pub fn reg_act_table_eb(&mut self) -> RegActTableEbW<'_, BlaiActTableEbE8Spec> {
        RegActTableEbW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xE8-0xEB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_eb_e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_eb_e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableEbE8Spec;
impl crate::RegisterSpec for BlaiActTableEbE8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_eb_e8::R`](R) reader structure"]
impl crate::Readable for BlaiActTableEbE8Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_eb_e8::W`](W) writer structure"]
impl crate::Writable for BlaiActTableEbE8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_EB_E8 to value 0"]
impl crate::Resettable for BlaiActTableEbE8Spec {}
