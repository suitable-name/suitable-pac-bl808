#[doc = "Register `GPADC_PIR_TRAIN` reader"]
pub type R = crate::R<GpadcPirTrainSpec>;
#[doc = "Register `GPADC_PIR_TRAIN` writer"]
pub type W = crate::W<GpadcPirTrainSpec>;
#[doc = "Field `pir_extend` reader - GPADC Record Extension after PIR interrupt"]
pub type PirExtendR = crate::FieldReader;
#[doc = "Field `pir_extend` writer - GPADC Record Extension after PIR interrupt"]
pub type PirExtendW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pir_cnt_v` reader - GPADC Record Extension Counter Value"]
pub type PirCntVR = crate::FieldReader;
#[doc = "Field `pir_train` reader - PIR Training Mode"]
pub type PirTrainR = crate::BitReader;
#[doc = "Field `pir_train` writer - PIR Training Mode"]
pub type PirTrainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pir_stop` reader - PIR Training End"]
pub type PirStopR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - GPADC Record Extension after PIR interrupt"]
    #[inline(always)]
    pub fn pir_extend(&self) -> PirExtendR {
        PirExtendR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - GPADC Record Extension Counter Value"]
    #[inline(always)]
    pub fn pir_cnt_v(&self) -> PirCntVR {
        PirCntVR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - PIR Training Mode"]
    #[inline(always)]
    pub fn pir_train(&self) -> PirTrainR {
        PirTrainR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PIR Training End"]
    #[inline(always)]
    pub fn pir_stop(&self) -> PirStopR {
        PirStopR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPADC Record Extension after PIR interrupt"]
    #[inline(always)]
    pub fn pir_extend(&mut self) -> PirExtendW<'_, GpadcPirTrainSpec> {
        PirExtendW::new(self, 0)
    }
    #[doc = "Bit 16 - PIR Training Mode"]
    #[inline(always)]
    pub fn pir_train(&mut self) -> PirTrainW<'_, GpadcPirTrainSpec> {
        PirTrainW::new(self, 16)
    }
}
#[doc = "GPADC PIR Training Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_pir_train::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_pir_train::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcPirTrainSpec;
impl crate::RegisterSpec for GpadcPirTrainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_pir_train::R`](R) reader structure"]
impl crate::Readable for GpadcPirTrainSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_pir_train::W`](W) writer structure"]
impl crate::Writable for GpadcPirTrainSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_PIR_TRAIN to value 0x0f"]
impl crate::Resettable for GpadcPirTrainSpec {
    const RESET_VALUE: u32 = 0x0f;
}
