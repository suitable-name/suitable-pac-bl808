#[doc = "Register `BLAI_ACT_TABLE_9F_9C` reader"]
pub type R = crate::R<BlaiActTable9f9cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_9F_9C` writer"]
pub type W = crate::W<BlaiActTable9f9cSpec>;
#[doc = "Field `reg_act_table_9C` reader - 8-bit activation value (Index 0x9C)"]
pub type RegActTable9cR = crate::FieldReader;
#[doc = "Field `reg_act_table_9C` writer - 8-bit activation value (Index 0x9C)"]
pub type RegActTable9cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_9D` reader - 8-bit activation value (Index 0x9D)"]
pub type RegActTable9dR = crate::FieldReader;
#[doc = "Field `reg_act_table_9D` writer - 8-bit activation value (Index 0x9D)"]
pub type RegActTable9dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_9E` reader - 8-bit activation value (Index 0x9E)"]
pub type RegActTable9eR = crate::FieldReader;
#[doc = "Field `reg_act_table_9E` writer - 8-bit activation value (Index 0x9E)"]
pub type RegActTable9eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_9F` reader - 8-bit activation value (Index 0x9F)"]
pub type RegActTable9fR = crate::FieldReader;
#[doc = "Field `reg_act_table_9F` writer - 8-bit activation value (Index 0x9F)"]
pub type RegActTable9fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x9C)"]
    #[inline(always)]
    pub fn reg_act_table_9c(&self) -> RegActTable9cR {
        RegActTable9cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x9D)"]
    #[inline(always)]
    pub fn reg_act_table_9d(&self) -> RegActTable9dR {
        RegActTable9dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x9E)"]
    #[inline(always)]
    pub fn reg_act_table_9e(&self) -> RegActTable9eR {
        RegActTable9eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x9F)"]
    #[inline(always)]
    pub fn reg_act_table_9f(&self) -> RegActTable9fR {
        RegActTable9fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x9C)"]
    #[inline(always)]
    pub fn reg_act_table_9c(&mut self) -> RegActTable9cW<'_, BlaiActTable9f9cSpec> {
        RegActTable9cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x9D)"]
    #[inline(always)]
    pub fn reg_act_table_9d(&mut self) -> RegActTable9dW<'_, BlaiActTable9f9cSpec> {
        RegActTable9dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x9E)"]
    #[inline(always)]
    pub fn reg_act_table_9e(&mut self) -> RegActTable9eW<'_, BlaiActTable9f9cSpec> {
        RegActTable9eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x9F)"]
    #[inline(always)]
    pub fn reg_act_table_9f(&mut self) -> RegActTable9fW<'_, BlaiActTable9f9cSpec> {
        RegActTable9fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x9C-0x9F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_9f_9c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_9f_9c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable9f9cSpec;
impl crate::RegisterSpec for BlaiActTable9f9cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_9f_9c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable9f9cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_9f_9c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable9f9cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_9F_9C to value 0"]
impl crate::Resettable for BlaiActTable9f9cSpec {}
