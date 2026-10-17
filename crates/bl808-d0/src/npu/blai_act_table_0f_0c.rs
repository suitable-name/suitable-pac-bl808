#[doc = "Register `BLAI_ACT_TABLE_0F_0C` reader"]
pub type R = crate::R<BlaiActTable0f0cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_0F_0C` writer"]
pub type W = crate::W<BlaiActTable0f0cSpec>;
#[doc = "Field `reg_act_table_0C` reader - 8-bit activation value (Index 0x0C)"]
pub type RegActTable0cR = crate::FieldReader;
#[doc = "Field `reg_act_table_0C` writer - 8-bit activation value (Index 0x0C)"]
pub type RegActTable0cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_0D` reader - 8-bit activation value (Index 0x0D)"]
pub type RegActTable0dR = crate::FieldReader;
#[doc = "Field `reg_act_table_0D` writer - 8-bit activation value (Index 0x0D)"]
pub type RegActTable0dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_0E` reader - 8-bit activation value (Index 0x0E)"]
pub type RegActTable0eR = crate::FieldReader;
#[doc = "Field `reg_act_table_0E` writer - 8-bit activation value (Index 0x0E)"]
pub type RegActTable0eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_0F` reader - 8-bit activation value (Index 0x0F)"]
pub type RegActTable0fR = crate::FieldReader;
#[doc = "Field `reg_act_table_0F` writer - 8-bit activation value (Index 0x0F)"]
pub type RegActTable0fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x0C)"]
    #[inline(always)]
    pub fn reg_act_table_0c(&self) -> RegActTable0cR {
        RegActTable0cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x0D)"]
    #[inline(always)]
    pub fn reg_act_table_0d(&self) -> RegActTable0dR {
        RegActTable0dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x0E)"]
    #[inline(always)]
    pub fn reg_act_table_0e(&self) -> RegActTable0eR {
        RegActTable0eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x0F)"]
    #[inline(always)]
    pub fn reg_act_table_0f(&self) -> RegActTable0fR {
        RegActTable0fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x0C)"]
    #[inline(always)]
    pub fn reg_act_table_0c(&mut self) -> RegActTable0cW<'_, BlaiActTable0f0cSpec> {
        RegActTable0cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x0D)"]
    #[inline(always)]
    pub fn reg_act_table_0d(&mut self) -> RegActTable0dW<'_, BlaiActTable0f0cSpec> {
        RegActTable0dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x0E)"]
    #[inline(always)]
    pub fn reg_act_table_0e(&mut self) -> RegActTable0eW<'_, BlaiActTable0f0cSpec> {
        RegActTable0eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x0F)"]
    #[inline(always)]
    pub fn reg_act_table_0f(&mut self) -> RegActTable0fW<'_, BlaiActTable0f0cSpec> {
        RegActTable0fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x0C-0x0F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_0f_0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_0f_0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable0f0cSpec;
impl crate::RegisterSpec for BlaiActTable0f0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_0f_0c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable0f0cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_0f_0c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable0f0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_0F_0C to value 0"]
impl crate::Resettable for BlaiActTable0f0cSpec {}
