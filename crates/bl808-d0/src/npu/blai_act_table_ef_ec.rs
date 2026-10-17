#[doc = "Register `BLAI_ACT_TABLE_EF_EC` reader"]
pub type R = crate::R<BlaiActTableEfEcSpec>;
#[doc = "Register `BLAI_ACT_TABLE_EF_EC` writer"]
pub type W = crate::W<BlaiActTableEfEcSpec>;
#[doc = "Field `reg_act_table_EC` reader - 8-bit activation value (Index 0xEC)"]
pub type RegActTableEcR = crate::FieldReader;
#[doc = "Field `reg_act_table_EC` writer - 8-bit activation value (Index 0xEC)"]
pub type RegActTableEcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_ED` reader - 8-bit activation value (Index 0xED)"]
pub type RegActTableEdR = crate::FieldReader;
#[doc = "Field `reg_act_table_ED` writer - 8-bit activation value (Index 0xED)"]
pub type RegActTableEdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_EE` reader - 8-bit activation value (Index 0xEE)"]
pub type RegActTableEeR = crate::FieldReader;
#[doc = "Field `reg_act_table_EE` writer - 8-bit activation value (Index 0xEE)"]
pub type RegActTableEeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_EF` reader - 8-bit activation value (Index 0xEF)"]
pub type RegActTableEfR = crate::FieldReader;
#[doc = "Field `reg_act_table_EF` writer - 8-bit activation value (Index 0xEF)"]
pub type RegActTableEfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xEC)"]
    #[inline(always)]
    pub fn reg_act_table_ec(&self) -> RegActTableEcR {
        RegActTableEcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xED)"]
    #[inline(always)]
    pub fn reg_act_table_ed(&self) -> RegActTableEdR {
        RegActTableEdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xEE)"]
    #[inline(always)]
    pub fn reg_act_table_ee(&self) -> RegActTableEeR {
        RegActTableEeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xEF)"]
    #[inline(always)]
    pub fn reg_act_table_ef(&self) -> RegActTableEfR {
        RegActTableEfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xEC)"]
    #[inline(always)]
    pub fn reg_act_table_ec(&mut self) -> RegActTableEcW<'_, BlaiActTableEfEcSpec> {
        RegActTableEcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xED)"]
    #[inline(always)]
    pub fn reg_act_table_ed(&mut self) -> RegActTableEdW<'_, BlaiActTableEfEcSpec> {
        RegActTableEdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xEE)"]
    #[inline(always)]
    pub fn reg_act_table_ee(&mut self) -> RegActTableEeW<'_, BlaiActTableEfEcSpec> {
        RegActTableEeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xEF)"]
    #[inline(always)]
    pub fn reg_act_table_ef(&mut self) -> RegActTableEfW<'_, BlaiActTableEfEcSpec> {
        RegActTableEfW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xEC-0xEF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_ef_ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_ef_ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableEfEcSpec;
impl crate::RegisterSpec for BlaiActTableEfEcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_ef_ec::R`](R) reader structure"]
impl crate::Readable for BlaiActTableEfEcSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_ef_ec::W`](W) writer structure"]
impl crate::Writable for BlaiActTableEfEcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_EF_EC to value 0"]
impl crate::Resettable for BlaiActTableEfEcSpec {}
