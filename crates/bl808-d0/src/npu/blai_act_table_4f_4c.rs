#[doc = "Register `BLAI_ACT_TABLE_4F_4C` reader"]
pub type R = crate::R<BlaiActTable4f4cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_4F_4C` writer"]
pub type W = crate::W<BlaiActTable4f4cSpec>;
#[doc = "Field `reg_act_table_4C` reader - 8-bit activation value (Index 0x4C)"]
pub type RegActTable4cR = crate::FieldReader;
#[doc = "Field `reg_act_table_4C` writer - 8-bit activation value (Index 0x4C)"]
pub type RegActTable4cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_4D` reader - 8-bit activation value (Index 0x4D)"]
pub type RegActTable4dR = crate::FieldReader;
#[doc = "Field `reg_act_table_4D` writer - 8-bit activation value (Index 0x4D)"]
pub type RegActTable4dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_4E` reader - 8-bit activation value (Index 0x4E)"]
pub type RegActTable4eR = crate::FieldReader;
#[doc = "Field `reg_act_table_4E` writer - 8-bit activation value (Index 0x4E)"]
pub type RegActTable4eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_4F` reader - 8-bit activation value (Index 0x4F)"]
pub type RegActTable4fR = crate::FieldReader;
#[doc = "Field `reg_act_table_4F` writer - 8-bit activation value (Index 0x4F)"]
pub type RegActTable4fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x4C)"]
    #[inline(always)]
    pub fn reg_act_table_4c(&self) -> RegActTable4cR {
        RegActTable4cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x4D)"]
    #[inline(always)]
    pub fn reg_act_table_4d(&self) -> RegActTable4dR {
        RegActTable4dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x4E)"]
    #[inline(always)]
    pub fn reg_act_table_4e(&self) -> RegActTable4eR {
        RegActTable4eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x4F)"]
    #[inline(always)]
    pub fn reg_act_table_4f(&self) -> RegActTable4fR {
        RegActTable4fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x4C)"]
    #[inline(always)]
    pub fn reg_act_table_4c(&mut self) -> RegActTable4cW<'_, BlaiActTable4f4cSpec> {
        RegActTable4cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x4D)"]
    #[inline(always)]
    pub fn reg_act_table_4d(&mut self) -> RegActTable4dW<'_, BlaiActTable4f4cSpec> {
        RegActTable4dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x4E)"]
    #[inline(always)]
    pub fn reg_act_table_4e(&mut self) -> RegActTable4eW<'_, BlaiActTable4f4cSpec> {
        RegActTable4eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x4F)"]
    #[inline(always)]
    pub fn reg_act_table_4f(&mut self) -> RegActTable4fW<'_, BlaiActTable4f4cSpec> {
        RegActTable4fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x4C-0x4F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_4f_4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_4f_4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable4f4cSpec;
impl crate::RegisterSpec for BlaiActTable4f4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_4f_4c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable4f4cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_4f_4c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable4f4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_4F_4C to value 0"]
impl crate::Resettable for BlaiActTable4f4cSpec {}
