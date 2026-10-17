#[doc = "Register `BLAI_ACT_TABLE_7F_7C` reader"]
pub type R = crate::R<BlaiActTable7f7cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_7F_7C` writer"]
pub type W = crate::W<BlaiActTable7f7cSpec>;
#[doc = "Field `reg_act_table_7C` reader - 8-bit activation value (Index 0x7C)"]
pub type RegActTable7cR = crate::FieldReader;
#[doc = "Field `reg_act_table_7C` writer - 8-bit activation value (Index 0x7C)"]
pub type RegActTable7cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_7D` reader - 8-bit activation value (Index 0x7D)"]
pub type RegActTable7dR = crate::FieldReader;
#[doc = "Field `reg_act_table_7D` writer - 8-bit activation value (Index 0x7D)"]
pub type RegActTable7dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_7E` reader - 8-bit activation value (Index 0x7E)"]
pub type RegActTable7eR = crate::FieldReader;
#[doc = "Field `reg_act_table_7E` writer - 8-bit activation value (Index 0x7E)"]
pub type RegActTable7eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_7F` reader - 8-bit activation value (Index 0x7F)"]
pub type RegActTable7fR = crate::FieldReader;
#[doc = "Field `reg_act_table_7F` writer - 8-bit activation value (Index 0x7F)"]
pub type RegActTable7fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x7C)"]
    #[inline(always)]
    pub fn reg_act_table_7c(&self) -> RegActTable7cR {
        RegActTable7cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x7D)"]
    #[inline(always)]
    pub fn reg_act_table_7d(&self) -> RegActTable7dR {
        RegActTable7dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x7E)"]
    #[inline(always)]
    pub fn reg_act_table_7e(&self) -> RegActTable7eR {
        RegActTable7eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x7F)"]
    #[inline(always)]
    pub fn reg_act_table_7f(&self) -> RegActTable7fR {
        RegActTable7fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x7C)"]
    #[inline(always)]
    pub fn reg_act_table_7c(&mut self) -> RegActTable7cW<'_, BlaiActTable7f7cSpec> {
        RegActTable7cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x7D)"]
    #[inline(always)]
    pub fn reg_act_table_7d(&mut self) -> RegActTable7dW<'_, BlaiActTable7f7cSpec> {
        RegActTable7dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x7E)"]
    #[inline(always)]
    pub fn reg_act_table_7e(&mut self) -> RegActTable7eW<'_, BlaiActTable7f7cSpec> {
        RegActTable7eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x7F)"]
    #[inline(always)]
    pub fn reg_act_table_7f(&mut self) -> RegActTable7fW<'_, BlaiActTable7f7cSpec> {
        RegActTable7fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x7C-0x7F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_7f_7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_7f_7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable7f7cSpec;
impl crate::RegisterSpec for BlaiActTable7f7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_7f_7c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable7f7cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_7f_7c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable7f7cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_7F_7C to value 0"]
impl crate::Resettable for BlaiActTable7f7cSpec {}
