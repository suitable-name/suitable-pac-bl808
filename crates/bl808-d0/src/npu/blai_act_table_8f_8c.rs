#[doc = "Register `BLAI_ACT_TABLE_8F_8C` reader"]
pub type R = crate::R<BlaiActTable8f8cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_8F_8C` writer"]
pub type W = crate::W<BlaiActTable8f8cSpec>;
#[doc = "Field `reg_act_table_8C` reader - 8-bit activation value (Index 0x8C)"]
pub type RegActTable8cR = crate::FieldReader;
#[doc = "Field `reg_act_table_8C` writer - 8-bit activation value (Index 0x8C)"]
pub type RegActTable8cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_8D` reader - 8-bit activation value (Index 0x8D)"]
pub type RegActTable8dR = crate::FieldReader;
#[doc = "Field `reg_act_table_8D` writer - 8-bit activation value (Index 0x8D)"]
pub type RegActTable8dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_8E` reader - 8-bit activation value (Index 0x8E)"]
pub type RegActTable8eR = crate::FieldReader;
#[doc = "Field `reg_act_table_8E` writer - 8-bit activation value (Index 0x8E)"]
pub type RegActTable8eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_8F` reader - 8-bit activation value (Index 0x8F)"]
pub type RegActTable8fR = crate::FieldReader;
#[doc = "Field `reg_act_table_8F` writer - 8-bit activation value (Index 0x8F)"]
pub type RegActTable8fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x8C)"]
    #[inline(always)]
    pub fn reg_act_table_8c(&self) -> RegActTable8cR {
        RegActTable8cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x8D)"]
    #[inline(always)]
    pub fn reg_act_table_8d(&self) -> RegActTable8dR {
        RegActTable8dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x8E)"]
    #[inline(always)]
    pub fn reg_act_table_8e(&self) -> RegActTable8eR {
        RegActTable8eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x8F)"]
    #[inline(always)]
    pub fn reg_act_table_8f(&self) -> RegActTable8fR {
        RegActTable8fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x8C)"]
    #[inline(always)]
    pub fn reg_act_table_8c(&mut self) -> RegActTable8cW<'_, BlaiActTable8f8cSpec> {
        RegActTable8cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x8D)"]
    #[inline(always)]
    pub fn reg_act_table_8d(&mut self) -> RegActTable8dW<'_, BlaiActTable8f8cSpec> {
        RegActTable8dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x8E)"]
    #[inline(always)]
    pub fn reg_act_table_8e(&mut self) -> RegActTable8eW<'_, BlaiActTable8f8cSpec> {
        RegActTable8eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x8F)"]
    #[inline(always)]
    pub fn reg_act_table_8f(&mut self) -> RegActTable8fW<'_, BlaiActTable8f8cSpec> {
        RegActTable8fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x8C-0x8F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_8f_8c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_8f_8c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable8f8cSpec;
impl crate::RegisterSpec for BlaiActTable8f8cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_8f_8c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable8f8cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_8f_8c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable8f8cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_8F_8C to value 0"]
impl crate::Resettable for BlaiActTable8f8cSpec {}
