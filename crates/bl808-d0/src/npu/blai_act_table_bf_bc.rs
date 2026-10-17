#[doc = "Register `BLAI_ACT_TABLE_BF_BC` reader"]
pub type R = crate::R<BlaiActTableBfBcSpec>;
#[doc = "Register `BLAI_ACT_TABLE_BF_BC` writer"]
pub type W = crate::W<BlaiActTableBfBcSpec>;
#[doc = "Field `reg_act_table_BC` reader - 8-bit activation value (Index 0xBC)"]
pub type RegActTableBcR = crate::FieldReader;
#[doc = "Field `reg_act_table_BC` writer - 8-bit activation value (Index 0xBC)"]
pub type RegActTableBcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_BD` reader - 8-bit activation value (Index 0xBD)"]
pub type RegActTableBdR = crate::FieldReader;
#[doc = "Field `reg_act_table_BD` writer - 8-bit activation value (Index 0xBD)"]
pub type RegActTableBdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_BE` reader - 8-bit activation value (Index 0xBE)"]
pub type RegActTableBeR = crate::FieldReader;
#[doc = "Field `reg_act_table_BE` writer - 8-bit activation value (Index 0xBE)"]
pub type RegActTableBeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_BF` reader - 8-bit activation value (Index 0xBF)"]
pub type RegActTableBfR = crate::FieldReader;
#[doc = "Field `reg_act_table_BF` writer - 8-bit activation value (Index 0xBF)"]
pub type RegActTableBfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xBC)"]
    #[inline(always)]
    pub fn reg_act_table_bc(&self) -> RegActTableBcR {
        RegActTableBcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xBD)"]
    #[inline(always)]
    pub fn reg_act_table_bd(&self) -> RegActTableBdR {
        RegActTableBdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xBE)"]
    #[inline(always)]
    pub fn reg_act_table_be(&self) -> RegActTableBeR {
        RegActTableBeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xBF)"]
    #[inline(always)]
    pub fn reg_act_table_bf(&self) -> RegActTableBfR {
        RegActTableBfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xBC)"]
    #[inline(always)]
    pub fn reg_act_table_bc(&mut self) -> RegActTableBcW<'_, BlaiActTableBfBcSpec> {
        RegActTableBcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xBD)"]
    #[inline(always)]
    pub fn reg_act_table_bd(&mut self) -> RegActTableBdW<'_, BlaiActTableBfBcSpec> {
        RegActTableBdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xBE)"]
    #[inline(always)]
    pub fn reg_act_table_be(&mut self) -> RegActTableBeW<'_, BlaiActTableBfBcSpec> {
        RegActTableBeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xBF)"]
    #[inline(always)]
    pub fn reg_act_table_bf(&mut self) -> RegActTableBfW<'_, BlaiActTableBfBcSpec> {
        RegActTableBfW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xBC-0xBF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_bf_bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_bf_bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableBfBcSpec;
impl crate::RegisterSpec for BlaiActTableBfBcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_bf_bc::R`](R) reader structure"]
impl crate::Readable for BlaiActTableBfBcSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_bf_bc::W`](W) writer structure"]
impl crate::Writable for BlaiActTableBfBcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_BF_BC to value 0"]
impl crate::Resettable for BlaiActTableBfBcSpec {}
