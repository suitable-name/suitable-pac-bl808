#[doc = "Register `BLAI_ACT_TABLE_CF_CC` reader"]
pub type R = crate::R<BlaiActTableCfCcSpec>;
#[doc = "Register `BLAI_ACT_TABLE_CF_CC` writer"]
pub type W = crate::W<BlaiActTableCfCcSpec>;
#[doc = "Field `reg_act_table_CC` reader - 8-bit activation value (Index 0xCC)"]
pub type RegActTableCcR = crate::FieldReader;
#[doc = "Field `reg_act_table_CC` writer - 8-bit activation value (Index 0xCC)"]
pub type RegActTableCcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_CD` reader - 8-bit activation value (Index 0xCD)"]
pub type RegActTableCdR = crate::FieldReader;
#[doc = "Field `reg_act_table_CD` writer - 8-bit activation value (Index 0xCD)"]
pub type RegActTableCdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_CE` reader - 8-bit activation value (Index 0xCE)"]
pub type RegActTableCeR = crate::FieldReader;
#[doc = "Field `reg_act_table_CE` writer - 8-bit activation value (Index 0xCE)"]
pub type RegActTableCeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_CF` reader - 8-bit activation value (Index 0xCF)"]
pub type RegActTableCfR = crate::FieldReader;
#[doc = "Field `reg_act_table_CF` writer - 8-bit activation value (Index 0xCF)"]
pub type RegActTableCfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xCC)"]
    #[inline(always)]
    pub fn reg_act_table_cc(&self) -> RegActTableCcR {
        RegActTableCcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xCD)"]
    #[inline(always)]
    pub fn reg_act_table_cd(&self) -> RegActTableCdR {
        RegActTableCdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xCE)"]
    #[inline(always)]
    pub fn reg_act_table_ce(&self) -> RegActTableCeR {
        RegActTableCeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xCF)"]
    #[inline(always)]
    pub fn reg_act_table_cf(&self) -> RegActTableCfR {
        RegActTableCfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xCC)"]
    #[inline(always)]
    pub fn reg_act_table_cc(&mut self) -> RegActTableCcW<'_, BlaiActTableCfCcSpec> {
        RegActTableCcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xCD)"]
    #[inline(always)]
    pub fn reg_act_table_cd(&mut self) -> RegActTableCdW<'_, BlaiActTableCfCcSpec> {
        RegActTableCdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xCE)"]
    #[inline(always)]
    pub fn reg_act_table_ce(&mut self) -> RegActTableCeW<'_, BlaiActTableCfCcSpec> {
        RegActTableCeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xCF)"]
    #[inline(always)]
    pub fn reg_act_table_cf(&mut self) -> RegActTableCfW<'_, BlaiActTableCfCcSpec> {
        RegActTableCfW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xCC-0xCF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_cf_cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_cf_cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableCfCcSpec;
impl crate::RegisterSpec for BlaiActTableCfCcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_cf_cc::R`](R) reader structure"]
impl crate::Readable for BlaiActTableCfCcSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_cf_cc::W`](W) writer structure"]
impl crate::Writable for BlaiActTableCfCcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_CF_CC to value 0"]
impl crate::Resettable for BlaiActTableCfCcSpec {}
