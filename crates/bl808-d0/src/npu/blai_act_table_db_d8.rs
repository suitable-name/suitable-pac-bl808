#[doc = "Register `BLAI_ACT_TABLE_DB_D8` reader"]
pub type R = crate::R<BlaiActTableDbD8Spec>;
#[doc = "Register `BLAI_ACT_TABLE_DB_D8` writer"]
pub type W = crate::W<BlaiActTableDbD8Spec>;
#[doc = "Field `reg_act_table_D8` reader - 8-bit activation value (Index 0xD8)"]
pub type RegActTableD8R = crate::FieldReader;
#[doc = "Field `reg_act_table_D8` writer - 8-bit activation value (Index 0xD8)"]
pub type RegActTableD8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_D9` reader - 8-bit activation value (Index 0xD9)"]
pub type RegActTableD9R = crate::FieldReader;
#[doc = "Field `reg_act_table_D9` writer - 8-bit activation value (Index 0xD9)"]
pub type RegActTableD9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_DA` reader - 8-bit activation value (Index 0xDA)"]
pub type RegActTableDaR = crate::FieldReader;
#[doc = "Field `reg_act_table_DA` writer - 8-bit activation value (Index 0xDA)"]
pub type RegActTableDaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_DB` reader - 8-bit activation value (Index 0xDB)"]
pub type RegActTableDbR = crate::FieldReader;
#[doc = "Field `reg_act_table_DB` writer - 8-bit activation value (Index 0xDB)"]
pub type RegActTableDbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xD8)"]
    #[inline(always)]
    pub fn reg_act_table_d8(&self) -> RegActTableD8R {
        RegActTableD8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xD9)"]
    #[inline(always)]
    pub fn reg_act_table_d9(&self) -> RegActTableD9R {
        RegActTableD9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xDA)"]
    #[inline(always)]
    pub fn reg_act_table_da(&self) -> RegActTableDaR {
        RegActTableDaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xDB)"]
    #[inline(always)]
    pub fn reg_act_table_db(&self) -> RegActTableDbR {
        RegActTableDbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xD8)"]
    #[inline(always)]
    pub fn reg_act_table_d8(&mut self) -> RegActTableD8W<'_, BlaiActTableDbD8Spec> {
        RegActTableD8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xD9)"]
    #[inline(always)]
    pub fn reg_act_table_d9(&mut self) -> RegActTableD9W<'_, BlaiActTableDbD8Spec> {
        RegActTableD9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xDA)"]
    #[inline(always)]
    pub fn reg_act_table_da(&mut self) -> RegActTableDaW<'_, BlaiActTableDbD8Spec> {
        RegActTableDaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xDB)"]
    #[inline(always)]
    pub fn reg_act_table_db(&mut self) -> RegActTableDbW<'_, BlaiActTableDbD8Spec> {
        RegActTableDbW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xD8-0xDB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_db_d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_db_d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableDbD8Spec;
impl crate::RegisterSpec for BlaiActTableDbD8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_db_d8::R`](R) reader structure"]
impl crate::Readable for BlaiActTableDbD8Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_db_d8::W`](W) writer structure"]
impl crate::Writable for BlaiActTableDbD8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_DB_D8 to value 0"]
impl crate::Resettable for BlaiActTableDbD8Spec {}
