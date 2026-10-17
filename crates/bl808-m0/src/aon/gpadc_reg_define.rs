#[doc = "Register `GPADC_REG_DEFINE` reader"]
pub type R = crate::R<GpadcRegDefineSpec>;
#[doc = "Register `GPADC_REG_DEFINE` writer"]
pub type W = crate::W<GpadcRegDefineSpec>;
#[doc = "Field `gpadc_os_cal_data` reader - "]
pub type GpadcOsCalDataR = crate::FieldReader<u16>;
#[doc = "Field `gpadc_os_cal_data` writer - "]
pub type GpadcOsCalDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpadc_os_cal_data(&self) -> GpadcOsCalDataR {
        GpadcOsCalDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpadc_os_cal_data(&mut self) -> GpadcOsCalDataW<'_, GpadcRegDefineSpec> {
        GpadcOsCalDataW::new(self, 0)
    }
}
#[doc = "GPADC Defined Value (e.g., Offset Calibration)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_define::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_define::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegDefineSpec;
impl crate::RegisterSpec for GpadcRegDefineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_define::R`](R) reader structure"]
impl crate::Readable for GpadcRegDefineSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_define::W`](W) writer structure"]
impl crate::Writable for GpadcRegDefineSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_DEFINE to value 0"]
impl crate::Resettable for GpadcRegDefineSpec {}
