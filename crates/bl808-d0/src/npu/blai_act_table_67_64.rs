#[doc = "Register `BLAI_ACT_TABLE_67_64` reader"]
pub type R = crate::R<BlaiActTable67_64Spec>;
#[doc = "Register `BLAI_ACT_TABLE_67_64` writer"]
pub type W = crate::W<BlaiActTable67_64Spec>;
#[doc = "Field `reg_act_table_64` reader - 8-bit activation value (Index 0x64)"]
pub type RegActTable64R = crate::FieldReader;
#[doc = "Field `reg_act_table_64` writer - 8-bit activation value (Index 0x64)"]
pub type RegActTable64W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_65` reader - 8-bit activation value (Index 0x65)"]
pub type RegActTable65R = crate::FieldReader;
#[doc = "Field `reg_act_table_65` writer - 8-bit activation value (Index 0x65)"]
pub type RegActTable65W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_66` reader - 8-bit activation value (Index 0x66)"]
pub type RegActTable66R = crate::FieldReader;
#[doc = "Field `reg_act_table_66` writer - 8-bit activation value (Index 0x66)"]
pub type RegActTable66W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_67` reader - 8-bit activation value (Index 0x67)"]
pub type RegActTable67R = crate::FieldReader;
#[doc = "Field `reg_act_table_67` writer - 8-bit activation value (Index 0x67)"]
pub type RegActTable67W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x64)"]
    #[inline(always)]
    pub fn reg_act_table_64(&self) -> RegActTable64R {
        RegActTable64R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x65)"]
    #[inline(always)]
    pub fn reg_act_table_65(&self) -> RegActTable65R {
        RegActTable65R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x66)"]
    #[inline(always)]
    pub fn reg_act_table_66(&self) -> RegActTable66R {
        RegActTable66R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x67)"]
    #[inline(always)]
    pub fn reg_act_table_67(&self) -> RegActTable67R {
        RegActTable67R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x64)"]
    #[inline(always)]
    pub fn reg_act_table_64(&mut self) -> RegActTable64W<'_, BlaiActTable67_64Spec> {
        RegActTable64W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x65)"]
    #[inline(always)]
    pub fn reg_act_table_65(&mut self) -> RegActTable65W<'_, BlaiActTable67_64Spec> {
        RegActTable65W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x66)"]
    #[inline(always)]
    pub fn reg_act_table_66(&mut self) -> RegActTable66W<'_, BlaiActTable67_64Spec> {
        RegActTable66W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x67)"]
    #[inline(always)]
    pub fn reg_act_table_67(&mut self) -> RegActTable67W<'_, BlaiActTable67_64Spec> {
        RegActTable67W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x64-0x67)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_67_64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_67_64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable67_64Spec;
impl crate::RegisterSpec for BlaiActTable67_64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_67_64::R`](R) reader structure"]
impl crate::Readable for BlaiActTable67_64Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_67_64::W`](W) writer structure"]
impl crate::Writable for BlaiActTable67_64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_67_64 to value 0"]
impl crate::Resettable for BlaiActTable67_64Spec {}
