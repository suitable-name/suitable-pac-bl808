#[doc = "Register `BLAI_ACT_TABLE_3F_3C` reader"]
pub type R = crate::R<BlaiActTable3f3cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_3F_3C` writer"]
pub type W = crate::W<BlaiActTable3f3cSpec>;
#[doc = "Field `reg_act_table_3C` reader - 8-bit activation value (Index 0x3C)"]
pub type RegActTable3cR = crate::FieldReader;
#[doc = "Field `reg_act_table_3C` writer - 8-bit activation value (Index 0x3C)"]
pub type RegActTable3cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_3D` reader - 8-bit activation value (Index 0x3D)"]
pub type RegActTable3dR = crate::FieldReader;
#[doc = "Field `reg_act_table_3D` writer - 8-bit activation value (Index 0x3D)"]
pub type RegActTable3dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_3E` reader - 8-bit activation value (Index 0x3E)"]
pub type RegActTable3eR = crate::FieldReader;
#[doc = "Field `reg_act_table_3E` writer - 8-bit activation value (Index 0x3E)"]
pub type RegActTable3eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_3F` reader - 8-bit activation value (Index 0x3F)"]
pub type RegActTable3fR = crate::FieldReader;
#[doc = "Field `reg_act_table_3F` writer - 8-bit activation value (Index 0x3F)"]
pub type RegActTable3fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x3C)"]
    #[inline(always)]
    pub fn reg_act_table_3c(&self) -> RegActTable3cR {
        RegActTable3cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x3D)"]
    #[inline(always)]
    pub fn reg_act_table_3d(&self) -> RegActTable3dR {
        RegActTable3dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x3E)"]
    #[inline(always)]
    pub fn reg_act_table_3e(&self) -> RegActTable3eR {
        RegActTable3eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x3F)"]
    #[inline(always)]
    pub fn reg_act_table_3f(&self) -> RegActTable3fR {
        RegActTable3fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x3C)"]
    #[inline(always)]
    pub fn reg_act_table_3c(&mut self) -> RegActTable3cW<'_, BlaiActTable3f3cSpec> {
        RegActTable3cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x3D)"]
    #[inline(always)]
    pub fn reg_act_table_3d(&mut self) -> RegActTable3dW<'_, BlaiActTable3f3cSpec> {
        RegActTable3dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x3E)"]
    #[inline(always)]
    pub fn reg_act_table_3e(&mut self) -> RegActTable3eW<'_, BlaiActTable3f3cSpec> {
        RegActTable3eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x3F)"]
    #[inline(always)]
    pub fn reg_act_table_3f(&mut self) -> RegActTable3fW<'_, BlaiActTable3f3cSpec> {
        RegActTable3fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x3C-0x3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_3f_3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_3f_3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable3f3cSpec;
impl crate::RegisterSpec for BlaiActTable3f3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_3f_3c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable3f3cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_3f_3c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable3f3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_3F_3C to value 0"]
impl crate::Resettable for BlaiActTable3f3cSpec {}
