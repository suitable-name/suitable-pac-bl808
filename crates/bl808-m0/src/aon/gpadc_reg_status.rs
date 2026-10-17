#[doc = "Register `GPADC_REG_STATUS` reader"]
pub type R = crate::R<GpadcRegStatusSpec>;
#[doc = "Register `GPADC_REG_STATUS` writer"]
pub type W = crate::W<GpadcRegStatusSpec>;
#[doc = "Field `gpadc_data_rdy` reader - "]
pub type GpadcDataRdyR = crate::BitReader;
#[doc = "Field `gpadc_reserved` reader - "]
pub type GpadcReservedR = crate::FieldReader<u16>;
#[doc = "Field `gpadc_reserved` writer - "]
pub type GpadcReservedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_data_rdy(&self) -> GpadcDataRdyR {
        GpadcDataRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&self) -> GpadcReservedR {
        GpadcReservedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&mut self) -> GpadcReservedW<'_, GpadcRegStatusSpec> {
        GpadcReservedW::new(self, 16)
    }
}
#[doc = "GPADC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegStatusSpec;
impl crate::RegisterSpec for GpadcRegStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_status::R`](R) reader structure"]
impl crate::Readable for GpadcRegStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_status::W`](W) writer structure"]
impl crate::Writable for GpadcRegStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_STATUS to value 0"]
impl crate::Resettable for GpadcRegStatusSpec {}
