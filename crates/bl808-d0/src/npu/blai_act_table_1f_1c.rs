#[doc = "Register `BLAI_ACT_TABLE_1F_1C` reader"]
pub type R = crate::R<BlaiActTable1f1cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_1F_1C` writer"]
pub type W = crate::W<BlaiActTable1f1cSpec>;
#[doc = "Field `reg_act_table_1C` reader - 8-bit activation value (Index 0x1C)"]
pub type RegActTable1cR = crate::FieldReader;
#[doc = "Field `reg_act_table_1C` writer - 8-bit activation value (Index 0x1C)"]
pub type RegActTable1cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_1D` reader - 8-bit activation value (Index 0x1D)"]
pub type RegActTable1dR = crate::FieldReader;
#[doc = "Field `reg_act_table_1D` writer - 8-bit activation value (Index 0x1D)"]
pub type RegActTable1dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_1E` reader - 8-bit activation value (Index 0x1E)"]
pub type RegActTable1eR = crate::FieldReader;
#[doc = "Field `reg_act_table_1E` writer - 8-bit activation value (Index 0x1E)"]
pub type RegActTable1eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_1F` reader - 8-bit activation value (Index 0x1F)"]
pub type RegActTable1fR = crate::FieldReader;
#[doc = "Field `reg_act_table_1F` writer - 8-bit activation value (Index 0x1F)"]
pub type RegActTable1fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x1C)"]
    #[inline(always)]
    pub fn reg_act_table_1c(&self) -> RegActTable1cR {
        RegActTable1cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x1D)"]
    #[inline(always)]
    pub fn reg_act_table_1d(&self) -> RegActTable1dR {
        RegActTable1dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x1E)"]
    #[inline(always)]
    pub fn reg_act_table_1e(&self) -> RegActTable1eR {
        RegActTable1eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x1F)"]
    #[inline(always)]
    pub fn reg_act_table_1f(&self) -> RegActTable1fR {
        RegActTable1fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x1C)"]
    #[inline(always)]
    pub fn reg_act_table_1c(&mut self) -> RegActTable1cW<'_, BlaiActTable1f1cSpec> {
        RegActTable1cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x1D)"]
    #[inline(always)]
    pub fn reg_act_table_1d(&mut self) -> RegActTable1dW<'_, BlaiActTable1f1cSpec> {
        RegActTable1dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x1E)"]
    #[inline(always)]
    pub fn reg_act_table_1e(&mut self) -> RegActTable1eW<'_, BlaiActTable1f1cSpec> {
        RegActTable1eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x1F)"]
    #[inline(always)]
    pub fn reg_act_table_1f(&mut self) -> RegActTable1fW<'_, BlaiActTable1f1cSpec> {
        RegActTable1fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x1C-0x1F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_1f_1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_1f_1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable1f1cSpec;
impl crate::RegisterSpec for BlaiActTable1f1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_1f_1c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable1f1cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_1f_1c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable1f1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_1F_1C to value 0"]
impl crate::Resettable for BlaiActTable1f1cSpec {}
