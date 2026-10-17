#[doc = "Register `BLAI_ACT_TABLE_AF_AC` reader"]
pub type R = crate::R<BlaiActTableAfAcSpec>;
#[doc = "Register `BLAI_ACT_TABLE_AF_AC` writer"]
pub type W = crate::W<BlaiActTableAfAcSpec>;
#[doc = "Field `reg_act_table_AC` reader - 8-bit activation value (Index 0xAC)"]
pub type RegActTableAcR = crate::FieldReader;
#[doc = "Field `reg_act_table_AC` writer - 8-bit activation value (Index 0xAC)"]
pub type RegActTableAcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_AD` reader - 8-bit activation value (Index 0xAD)"]
pub type RegActTableAdR = crate::FieldReader;
#[doc = "Field `reg_act_table_AD` writer - 8-bit activation value (Index 0xAD)"]
pub type RegActTableAdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_AE` reader - 8-bit activation value (Index 0xAE)"]
pub type RegActTableAeR = crate::FieldReader;
#[doc = "Field `reg_act_table_AE` writer - 8-bit activation value (Index 0xAE)"]
pub type RegActTableAeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_AF` reader - 8-bit activation value (Index 0xAF)"]
pub type RegActTableAfR = crate::FieldReader;
#[doc = "Field `reg_act_table_AF` writer - 8-bit activation value (Index 0xAF)"]
pub type RegActTableAfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xAC)"]
    #[inline(always)]
    pub fn reg_act_table_ac(&self) -> RegActTableAcR {
        RegActTableAcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xAD)"]
    #[inline(always)]
    pub fn reg_act_table_ad(&self) -> RegActTableAdR {
        RegActTableAdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xAE)"]
    #[inline(always)]
    pub fn reg_act_table_ae(&self) -> RegActTableAeR {
        RegActTableAeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xAF)"]
    #[inline(always)]
    pub fn reg_act_table_af(&self) -> RegActTableAfR {
        RegActTableAfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xAC)"]
    #[inline(always)]
    pub fn reg_act_table_ac(&mut self) -> RegActTableAcW<'_, BlaiActTableAfAcSpec> {
        RegActTableAcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xAD)"]
    #[inline(always)]
    pub fn reg_act_table_ad(&mut self) -> RegActTableAdW<'_, BlaiActTableAfAcSpec> {
        RegActTableAdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xAE)"]
    #[inline(always)]
    pub fn reg_act_table_ae(&mut self) -> RegActTableAeW<'_, BlaiActTableAfAcSpec> {
        RegActTableAeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xAF)"]
    #[inline(always)]
    pub fn reg_act_table_af(&mut self) -> RegActTableAfW<'_, BlaiActTableAfAcSpec> {
        RegActTableAfW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xAC-0xAF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_af_ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_af_ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableAfAcSpec;
impl crate::RegisterSpec for BlaiActTableAfAcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_af_ac::R`](R) reader structure"]
impl crate::Readable for BlaiActTableAfAcSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_af_ac::W`](W) writer structure"]
impl crate::Writable for BlaiActTableAfAcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_AF_AC to value 0"]
impl crate::Resettable for BlaiActTableAfAcSpec {}
