#[doc = "Register `GPADC_REG_SCN_NEG2` reader"]
pub type R = crate::R<GpadcRegScnNeg2Spec>;
#[doc = "Register `GPADC_REG_SCN_NEG2` writer"]
pub type W = crate::W<GpadcRegScnNeg2Spec>;
#[doc = "Field `gpadc_scan_neg_6` reader - "]
pub type GpadcScanNeg6R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_6` writer - "]
pub type GpadcScanNeg6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_7` reader - "]
pub type GpadcScanNeg7R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_7` writer - "]
pub type GpadcScanNeg7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_8` reader - "]
pub type GpadcScanNeg8R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_8` writer - "]
pub type GpadcScanNeg8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_9` reader - "]
pub type GpadcScanNeg9R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_9` writer - "]
pub type GpadcScanNeg9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_10` reader - "]
pub type GpadcScanNeg10R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_10` writer - "]
pub type GpadcScanNeg10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_11` reader - "]
pub type GpadcScanNeg11R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_11` writer - "]
pub type GpadcScanNeg11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_6(&self) -> GpadcScanNeg6R {
        GpadcScanNeg6R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_7(&self) -> GpadcScanNeg7R {
        GpadcScanNeg7R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_8(&self) -> GpadcScanNeg8R {
        GpadcScanNeg8R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_9(&self) -> GpadcScanNeg9R {
        GpadcScanNeg9R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_10(&self) -> GpadcScanNeg10R {
        GpadcScanNeg10R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_11(&self) -> GpadcScanNeg11R {
        GpadcScanNeg11R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_6(&mut self) -> GpadcScanNeg6W<'_, GpadcRegScnNeg2Spec> {
        GpadcScanNeg6W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_7(&mut self) -> GpadcScanNeg7W<'_, GpadcRegScnNeg2Spec> {
        GpadcScanNeg7W::new(self, 5)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_8(&mut self) -> GpadcScanNeg8W<'_, GpadcRegScnNeg2Spec> {
        GpadcScanNeg8W::new(self, 10)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_9(&mut self) -> GpadcScanNeg9W<'_, GpadcRegScnNeg2Spec> {
        GpadcScanNeg9W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_10(&mut self) -> GpadcScanNeg10W<'_, GpadcRegScnNeg2Spec> {
        GpadcScanNeg10W::new(self, 20)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_11(&mut self) -> GpadcScanNeg11W<'_, GpadcRegScnNeg2Spec> {
        GpadcScanNeg11W::new(self, 25)
    }
}
#[doc = "GPADC Conversion Sequence 4 (Negative Channels 6-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_neg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_neg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegScnNeg2Spec;
impl crate::RegisterSpec for GpadcRegScnNeg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_scn_neg2::R`](R) reader structure"]
impl crate::Readable for GpadcRegScnNeg2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_scn_neg2::W`](W) writer structure"]
impl crate::Writable for GpadcRegScnNeg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_SCN_NEG2 to value 0"]
impl crate::Resettable for GpadcRegScnNeg2Spec {}
