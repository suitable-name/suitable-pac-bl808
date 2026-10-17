#[doc = "Register `BLAI_ACT_TABLE_2F_2C` reader"]
pub type R = crate::R<BlaiActTable2f2cSpec>;
#[doc = "Register `BLAI_ACT_TABLE_2F_2C` writer"]
pub type W = crate::W<BlaiActTable2f2cSpec>;
#[doc = "Field `reg_act_table_2C` reader - 8-bit activation value (Index 0x2C)"]
pub type RegActTable2cR = crate::FieldReader;
#[doc = "Field `reg_act_table_2C` writer - 8-bit activation value (Index 0x2C)"]
pub type RegActTable2cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_2D` reader - 8-bit activation value (Index 0x2D)"]
pub type RegActTable2dR = crate::FieldReader;
#[doc = "Field `reg_act_table_2D` writer - 8-bit activation value (Index 0x2D)"]
pub type RegActTable2dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_2E` reader - 8-bit activation value (Index 0x2E)"]
pub type RegActTable2eR = crate::FieldReader;
#[doc = "Field `reg_act_table_2E` writer - 8-bit activation value (Index 0x2E)"]
pub type RegActTable2eW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_2F` reader - 8-bit activation value (Index 0x2F)"]
pub type RegActTable2fR = crate::FieldReader;
#[doc = "Field `reg_act_table_2F` writer - 8-bit activation value (Index 0x2F)"]
pub type RegActTable2fW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x2C)"]
    #[inline(always)]
    pub fn reg_act_table_2c(&self) -> RegActTable2cR {
        RegActTable2cR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x2D)"]
    #[inline(always)]
    pub fn reg_act_table_2d(&self) -> RegActTable2dR {
        RegActTable2dR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x2E)"]
    #[inline(always)]
    pub fn reg_act_table_2e(&self) -> RegActTable2eR {
        RegActTable2eR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x2F)"]
    #[inline(always)]
    pub fn reg_act_table_2f(&self) -> RegActTable2fR {
        RegActTable2fR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x2C)"]
    #[inline(always)]
    pub fn reg_act_table_2c(&mut self) -> RegActTable2cW<'_, BlaiActTable2f2cSpec> {
        RegActTable2cW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x2D)"]
    #[inline(always)]
    pub fn reg_act_table_2d(&mut self) -> RegActTable2dW<'_, BlaiActTable2f2cSpec> {
        RegActTable2dW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x2E)"]
    #[inline(always)]
    pub fn reg_act_table_2e(&mut self) -> RegActTable2eW<'_, BlaiActTable2f2cSpec> {
        RegActTable2eW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x2F)"]
    #[inline(always)]
    pub fn reg_act_table_2f(&mut self) -> RegActTable2fW<'_, BlaiActTable2f2cSpec> {
        RegActTable2fW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x2C-0x2F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_2f_2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_2f_2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable2f2cSpec;
impl crate::RegisterSpec for BlaiActTable2f2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_2f_2c::R`](R) reader structure"]
impl crate::Readable for BlaiActTable2f2cSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_2f_2c::W`](W) writer structure"]
impl crate::Writable for BlaiActTable2f2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_2F_2C to value 0"]
impl crate::Resettable for BlaiActTable2f2cSpec {}
