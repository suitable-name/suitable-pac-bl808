#[doc = "Register `BLAI_ACT_TABLE_FF_FC` reader"]
pub type R = crate::R<BlaiActTableFfFcSpec>;
#[doc = "Register `BLAI_ACT_TABLE_FF_FC` writer"]
pub type W = crate::W<BlaiActTableFfFcSpec>;
#[doc = "Field `reg_act_table_FC` reader - 8-bit activation value (Index 0xFC)"]
pub type RegActTableFcR = crate::FieldReader;
#[doc = "Field `reg_act_table_FC` writer - 8-bit activation value (Index 0xFC)"]
pub type RegActTableFcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_FD` reader - 8-bit activation value (Index 0xFD)"]
pub type RegActTableFdR = crate::FieldReader;
#[doc = "Field `reg_act_table_FD` writer - 8-bit activation value (Index 0xFD)"]
pub type RegActTableFdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_FE` reader - 8-bit activation value (Index 0xFE)"]
pub type RegActTableFeR = crate::FieldReader;
#[doc = "Field `reg_act_table_FE` writer - 8-bit activation value (Index 0xFE)"]
pub type RegActTableFeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_FF` reader - 8-bit activation value (Index 0xFF)"]
pub type RegActTableFfR = crate::FieldReader;
#[doc = "Field `reg_act_table_FF` writer - 8-bit activation value (Index 0xFF)"]
pub type RegActTableFfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xFC)"]
    #[inline(always)]
    pub fn reg_act_table_fc(&self) -> RegActTableFcR {
        RegActTableFcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xFD)"]
    #[inline(always)]
    pub fn reg_act_table_fd(&self) -> RegActTableFdR {
        RegActTableFdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xFE)"]
    #[inline(always)]
    pub fn reg_act_table_fe(&self) -> RegActTableFeR {
        RegActTableFeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xFF)"]
    #[inline(always)]
    pub fn reg_act_table_ff(&self) -> RegActTableFfR {
        RegActTableFfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xFC)"]
    #[inline(always)]
    pub fn reg_act_table_fc(&mut self) -> RegActTableFcW<'_, BlaiActTableFfFcSpec> {
        RegActTableFcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xFD)"]
    #[inline(always)]
    pub fn reg_act_table_fd(&mut self) -> RegActTableFdW<'_, BlaiActTableFfFcSpec> {
        RegActTableFdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xFE)"]
    #[inline(always)]
    pub fn reg_act_table_fe(&mut self) -> RegActTableFeW<'_, BlaiActTableFfFcSpec> {
        RegActTableFeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xFF)"]
    #[inline(always)]
    pub fn reg_act_table_ff(&mut self) -> RegActTableFfW<'_, BlaiActTableFfFcSpec> {
        RegActTableFfW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xFC-0xFF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_ff_fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_ff_fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableFfFcSpec;
impl crate::RegisterSpec for BlaiActTableFfFcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_ff_fc::R`](R) reader structure"]
impl crate::Readable for BlaiActTableFfFcSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_ff_fc::W`](W) writer structure"]
impl crate::Writable for BlaiActTableFfFcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_FF_FC to value 0"]
impl crate::Resettable for BlaiActTableFfFcSpec {}
