#[doc = "Register `BLAI_ACT_TABLE_5F_5C` reader"]
pub type R = crate::R<BlaiActTable5f5cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_5F_5C` writer"]
pub type W = crate::W<BlaiActTable5f5cSpec>;
#[doc = "Field `reg_act_table_5C` reader - 8-bit activation value (Index 0x5C)"]
pub type RegActTable5cR = crate::FieldReader;
#[doc = "Field `reg_act_table_5C` writer - 8-bit activation value (Index 0x5C)"]
pub type RegActTable5cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_5D` reader - 8-bit activation value (Index 0x5D)"]
pub type RegActTable5dR = crate::FieldReader;
#[doc = "Field `reg_act_table_5D` writer - 8-bit activation value (Index 0x5D)"]
pub type RegActTable5dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_5E` reader - 8-bit activation value (Index 0x5E)"]
pub type RegActTable5eR = crate::FieldReader;
#[doc = "Field `reg_act_table_5E` writer - 8-bit activation value (Index 0x5E)"]
pub type RegActTable5eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_5F` reader - 8-bit activation value (Index 0x5F)"]
pub type RegActTable5fR = crate::FieldReader;
#[doc = "Field `reg_act_table_5F` writer - 8-bit activation value (Index 0x5F)"]
pub type RegActTable5fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x5C)"]
    #[inline(always)]
    pub fn reg_act_table_5c(&self) -> RegActTable5cR {
        RegActTable5cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x5D)"]
    #[inline(always)]
    pub fn reg_act_table_5d(&self) -> RegActTable5dR {
        RegActTable5dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x5E)"]
    #[inline(always)]
    pub fn reg_act_table_5e(&self) -> RegActTable5eR {
        RegActTable5eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x5F)"]
    #[inline(always)]
    pub fn reg_act_table_5f(&self) -> RegActTable5fR {
        RegActTable5fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x5C)"]
    #[inline(always)]
    pub fn reg_act_table_5c(&mut self) -> RegActTable5cW<'_, BlaiActTable5f5cSpec> {
        RegActTable5cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x5D)"]
    #[inline(always)]
    pub fn reg_act_table_5d(&mut self) -> RegActTable5dW<'_, BlaiActTable5f5cSpec> {
        RegActTable5dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x5E)"]
    #[inline(always)]
    pub fn reg_act_table_5e(&mut self) -> RegActTable5eW<'_, BlaiActTable5f5cSpec> {
        RegActTable5eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x5F)"]
    #[inline(always)]
    pub fn reg_act_table_5f(&mut self) -> RegActTable5fW<'_, BlaiActTable5f5cSpec> {
        RegActTable5fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x5C-0x5F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_5f_5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_5f_5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable5f5cSpec;
impl crate::RegisterSpec for BlaiActTable5f5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_5f_5c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable5f5cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_5f_5c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable5f5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_5F_5C to value 0"]
impl crate::Resettable for BlaiActTable5f5cSpec {}
