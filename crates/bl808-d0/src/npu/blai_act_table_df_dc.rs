#[doc = "Register `BLAI_ACT_TABLE_DF_DC` reader"]
pub type R = crate::R<BlaiActTableDfDcSpec>;
#[doc = "Register `BLAI_ACT_TABLE_DF_DC` writer"]
pub type W = crate::W<BlaiActTableDfDcSpec>;
#[doc = "Field `reg_act_table_DC` reader - 8-bit activation value (Index 0xDC)"]
pub type RegActTableDcR = crate::FieldReader;
#[doc = "Field `reg_act_table_DC` writer - 8-bit activation value (Index 0xDC)"]
pub type RegActTableDcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_DD` reader - 8-bit activation value (Index 0xDD)"]
pub type RegActTableDdR = crate::FieldReader;
#[doc = "Field `reg_act_table_DD` writer - 8-bit activation value (Index 0xDD)"]
pub type RegActTableDdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_DE` reader - 8-bit activation value (Index 0xDE)"]
pub type RegActTableDeR = crate::FieldReader;
#[doc = "Field `reg_act_table_DE` writer - 8-bit activation value (Index 0xDE)"]
pub type RegActTableDeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_DF` reader - 8-bit activation value (Index 0xDF)"]
pub type RegActTableDfR = crate::FieldReader;
#[doc = "Field `reg_act_table_DF` writer - 8-bit activation value (Index 0xDF)"]
pub type RegActTableDfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xDC)"]
    #[inline(always)]
    pub fn reg_act_table_dc(&self) -> RegActTableDcR {
        RegActTableDcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xDD)"]
    #[inline(always)]
    pub fn reg_act_table_dd(&self) -> RegActTableDdR {
        RegActTableDdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xDE)"]
    #[inline(always)]
    pub fn reg_act_table_de(&self) -> RegActTableDeR {
        RegActTableDeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xDF)"]
    #[inline(always)]
    pub fn reg_act_table_df(&self) -> RegActTableDfR {
        RegActTableDfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xDC)"]
    #[inline(always)]
    pub fn reg_act_table_dc(&mut self) -> RegActTableDcW<'_, BlaiActTableDfDcSpec> {
        RegActTableDcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xDD)"]
    #[inline(always)]
    pub fn reg_act_table_dd(&mut self) -> RegActTableDdW<'_, BlaiActTableDfDcSpec> {
        RegActTableDdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xDE)"]
    #[inline(always)]
    pub fn reg_act_table_de(&mut self) -> RegActTableDeW<'_, BlaiActTableDfDcSpec> {
        RegActTableDeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xDF)"]
    #[inline(always)]
    pub fn reg_act_table_df(&mut self) -> RegActTableDfW<'_, BlaiActTableDfDcSpec> {
        RegActTableDfW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xDC-0xDF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_df_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_df_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableDfDcSpec;
impl crate::RegisterSpec for BlaiActTableDfDcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_df_dc::R`](R) reader structure"]
impl crate::Readable for BlaiActTableDfDcSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_df_dc::W`](W) writer structure"]
impl crate::Writable for BlaiActTableDfDcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_DF_DC to value 0"]
impl crate::Resettable for BlaiActTableDfDcSpec {}
