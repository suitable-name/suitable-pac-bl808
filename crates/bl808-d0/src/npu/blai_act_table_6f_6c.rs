#[doc = "Register `BLAI_ACT_TABLE_6F_6C` reader"]
pub type R = crate::R<BlaiActTable6f6cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_6F_6C` writer"]
pub type W = crate::W<BlaiActTable6f6cSpec>;
#[doc = "Field `reg_act_table_6C` reader - 8-bit activation value (Index 0x6C)"]
pub type RegActTable6cR = crate::FieldReader;
#[doc = "Field `reg_act_table_6C` writer - 8-bit activation value (Index 0x6C)"]
pub type RegActTable6cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_6D` reader - 8-bit activation value (Index 0x6D)"]
pub type RegActTable6dR = crate::FieldReader;
#[doc = "Field `reg_act_table_6D` writer - 8-bit activation value (Index 0x6D)"]
pub type RegActTable6dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_6E` reader - 8-bit activation value (Index 0x6E)"]
pub type RegActTable6eR = crate::FieldReader;
#[doc = "Field `reg_act_table_6E` writer - 8-bit activation value (Index 0x6E)"]
pub type RegActTable6eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_6F` reader - 8-bit activation value (Index 0x6F)"]
pub type RegActTable6fR = crate::FieldReader;
#[doc = "Field `reg_act_table_6F` writer - 8-bit activation value (Index 0x6F)"]
pub type RegActTable6fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x6C)"]
    #[inline(always)]
    pub fn reg_act_table_6c(&self) -> RegActTable6cR {
        RegActTable6cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x6D)"]
    #[inline(always)]
    pub fn reg_act_table_6d(&self) -> RegActTable6dR {
        RegActTable6dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x6E)"]
    #[inline(always)]
    pub fn reg_act_table_6e(&self) -> RegActTable6eR {
        RegActTable6eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x6F)"]
    #[inline(always)]
    pub fn reg_act_table_6f(&self) -> RegActTable6fR {
        RegActTable6fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x6C)"]
    #[inline(always)]
    pub fn reg_act_table_6c(&mut self) -> RegActTable6cW<'_, BlaiActTable6f6cSpec> {
        RegActTable6cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x6D)"]
    #[inline(always)]
    pub fn reg_act_table_6d(&mut self) -> RegActTable6dW<'_, BlaiActTable6f6cSpec> {
        RegActTable6dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x6E)"]
    #[inline(always)]
    pub fn reg_act_table_6e(&mut self) -> RegActTable6eW<'_, BlaiActTable6f6cSpec> {
        RegActTable6eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x6F)"]
    #[inline(always)]
    pub fn reg_act_table_6f(&mut self) -> RegActTable6fW<'_, BlaiActTable6f6cSpec> {
        RegActTable6fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x6C-0x6F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_6f_6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_6f_6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable6f6cSpec;
impl crate::RegisterSpec for BlaiActTable6f6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_6f_6c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable6f6cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_6f_6c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable6f6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_6F_6C to value 0"]
impl crate::Resettable for BlaiActTable6f6cSpec {}
