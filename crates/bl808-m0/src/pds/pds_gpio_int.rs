#[doc = "Register `PDS_GPIO_INT` reader"]
pub type R = crate::R<PdsGpioIntSpec>;
#[doc = "Register `PDS_GPIO_INT` writer"]
pub type W = crate::W<PdsGpioIntSpec>;
#[doc = "Field `pds_gpio_set1_int_clr` reader - "]
pub type PdsGpioSet1IntClrR = crate::BitReader;
#[doc = "Field `pds_gpio_set1_int_clr` writer - "]
pub type PdsGpioSet1IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set1_int_mode` reader - "]
pub type PdsGpioSet1IntModeR = crate::FieldReader;
#[doc = "Field `pds_gpio_set1_int_mode` writer - "]
pub type PdsGpioSet1IntModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_set2_int_clr` reader - "]
pub type PdsGpioSet2IntClrR = crate::BitReader;
#[doc = "Field `pds_gpio_set2_int_clr` writer - "]
pub type PdsGpioSet2IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set2_int_mode` reader - "]
pub type PdsGpioSet2IntModeR = crate::FieldReader;
#[doc = "Field `pds_gpio_set2_int_mode` writer - "]
pub type PdsGpioSet2IntModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_set3_int_clr` reader - "]
pub type PdsGpioSet3IntClrR = crate::BitReader;
#[doc = "Field `pds_gpio_set3_int_clr` writer - "]
pub type PdsGpioSet3IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set3_int_mode` reader - "]
pub type PdsGpioSet3IntModeR = crate::FieldReader;
#[doc = "Field `pds_gpio_set3_int_mode` writer - "]
pub type PdsGpioSet3IntModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_gpio_set4_int_clr` reader - "]
pub type PdsGpioSet4IntClrR = crate::BitReader;
#[doc = "Field `pds_gpio_set4_int_clr` writer - "]
pub type PdsGpioSet4IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pds_gpio_set4_int_mode` reader - "]
pub type PdsGpioSet4IntModeR = crate::FieldReader;
#[doc = "Field `pds_gpio_set4_int_mode` writer - "]
pub type PdsGpioSet4IntModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_clr(&self) -> PdsGpioSet1IntClrR {
        PdsGpioSet1IntClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_mode(&self) -> PdsGpioSet1IntModeR {
        PdsGpioSet1IntModeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_clr(&self) -> PdsGpioSet2IntClrR {
        PdsGpioSet2IntClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_mode(&self) -> PdsGpioSet2IntModeR {
        PdsGpioSet2IntModeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_clr(&self) -> PdsGpioSet3IntClrR {
        PdsGpioSet3IntClrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_mode(&self) -> PdsGpioSet3IntModeR {
        PdsGpioSet3IntModeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_clr(&self) -> PdsGpioSet4IntClrR {
        PdsGpioSet4IntClrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_mode(&self) -> PdsGpioSet4IntModeR {
        PdsGpioSet4IntModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_clr(&mut self) -> PdsGpioSet1IntClrW<'_, PdsGpioIntSpec> {
        PdsGpioSet1IntClrW::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pds_gpio_set1_int_mode(&mut self) -> PdsGpioSet1IntModeW<'_, PdsGpioIntSpec> {
        PdsGpioSet1IntModeW::new(self, 4)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_clr(&mut self) -> PdsGpioSet2IntClrW<'_, PdsGpioIntSpec> {
        PdsGpioSet2IntClrW::new(self, 10)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pds_gpio_set2_int_mode(&mut self) -> PdsGpioSet2IntModeW<'_, PdsGpioIntSpec> {
        PdsGpioSet2IntModeW::new(self, 12)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_clr(&mut self) -> PdsGpioSet3IntClrW<'_, PdsGpioIntSpec> {
        PdsGpioSet3IntClrW::new(self, 18)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn pds_gpio_set3_int_mode(&mut self) -> PdsGpioSet3IntModeW<'_, PdsGpioIntSpec> {
        PdsGpioSet3IntModeW::new(self, 20)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_clr(&mut self) -> PdsGpioSet4IntClrW<'_, PdsGpioIntSpec> {
        PdsGpioSet4IntClrW::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pds_gpio_set4_int_mode(&mut self) -> PdsGpioSet4IntModeW<'_, PdsGpioIntSpec> {
        PdsGpioSet4IntModeW::new(self, 28)
    }
}
#[doc = "PDS GPIO Interrupt Mode and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsGpioIntSpec;
impl crate::RegisterSpec for PdsGpioIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_int::R`](R) reader structure"]
impl crate::Readable for PdsGpioIntSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_int::W`](W) writer structure"]
impl crate::Writable for PdsGpioIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_GPIO_INT to value 0"]
impl crate::Resettable for PdsGpioIntSpec {}
