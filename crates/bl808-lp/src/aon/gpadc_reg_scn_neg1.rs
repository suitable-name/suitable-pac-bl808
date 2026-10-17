#[doc = "Register `GPADC_REG_SCN_NEG1` reader"]
pub type R = crate::R<GpadcRegScnNeg1Spec>;
#[doc = "Register `GPADC_REG_SCN_NEG1` writer"]
pub type W = crate::W<GpadcRegScnNeg1Spec>;
#[doc = "Field `gpadc_scan_neg_0` reader - "]
pub type GpadcScanNeg0R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_0` writer - "]
pub type GpadcScanNeg0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_1` reader - "]
pub type GpadcScanNeg1R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_1` writer - "]
pub type GpadcScanNeg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_2` reader - "]
pub type GpadcScanNeg2R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_2` writer - "]
pub type GpadcScanNeg2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_3` reader - "]
pub type GpadcScanNeg3R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_3` writer - "]
pub type GpadcScanNeg3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_4` reader - "]
pub type GpadcScanNeg4R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_4` writer - "]
pub type GpadcScanNeg4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_5` reader - "]
pub type GpadcScanNeg5R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_5` writer - "]
pub type GpadcScanNeg5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_0(&self) -> GpadcScanNeg0R {
        GpadcScanNeg0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_1(&self) -> GpadcScanNeg1R {
        GpadcScanNeg1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_2(&self) -> GpadcScanNeg2R {
        GpadcScanNeg2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_3(&self) -> GpadcScanNeg3R {
        GpadcScanNeg3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_4(&self) -> GpadcScanNeg4R {
        GpadcScanNeg4R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_5(&self) -> GpadcScanNeg5R {
        GpadcScanNeg5R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_0(&mut self) -> GpadcScanNeg0W<'_, GpadcRegScnNeg1Spec> {
        GpadcScanNeg0W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_1(&mut self) -> GpadcScanNeg1W<'_, GpadcRegScnNeg1Spec> {
        GpadcScanNeg1W::new(self, 5)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_2(&mut self) -> GpadcScanNeg2W<'_, GpadcRegScnNeg1Spec> {
        GpadcScanNeg2W::new(self, 10)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_3(&mut self) -> GpadcScanNeg3W<'_, GpadcRegScnNeg1Spec> {
        GpadcScanNeg3W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_4(&mut self) -> GpadcScanNeg4W<'_, GpadcRegScnNeg1Spec> {
        GpadcScanNeg4W::new(self, 20)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_5(&mut self) -> GpadcScanNeg5W<'_, GpadcRegScnNeg1Spec> {
        GpadcScanNeg5W::new(self, 25)
    }
}
#[doc = "GPADC Conversion Sequence 3 (Negative Channels 0-5)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_neg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_neg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegScnNeg1Spec;
impl crate::RegisterSpec for GpadcRegScnNeg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_scn_neg1::R`](R) reader structure"]
impl crate::Readable for GpadcRegScnNeg1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_scn_neg1::W`](W) writer structure"]
impl crate::Writable for GpadcRegScnNeg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_SCN_NEG1 to value 0"]
impl crate::Resettable for GpadcRegScnNeg1Spec {}
