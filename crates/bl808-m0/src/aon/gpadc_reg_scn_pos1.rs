#[doc = "Register `GPADC_REG_SCN_POS1` reader"]
pub type R = crate::R<GpadcRegScnPos1Spec>;
#[doc = "Register `GPADC_REG_SCN_POS1` writer"]
pub type W = crate::W<GpadcRegScnPos1Spec>;
#[doc = "Field `gpadc_scan_pos_0` reader - "]
pub type GpadcScanPos0R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_0` writer - "]
pub type GpadcScanPos0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_1` reader - "]
pub type GpadcScanPos1R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_1` writer - "]
pub type GpadcScanPos1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_2` reader - "]
pub type GpadcScanPos2R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_2` writer - "]
pub type GpadcScanPos2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_3` reader - "]
pub type GpadcScanPos3R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_3` writer - "]
pub type GpadcScanPos3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_4` reader - "]
pub type GpadcScanPos4R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_4` writer - "]
pub type GpadcScanPos4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_5` reader - "]
pub type GpadcScanPos5R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_5` writer - "]
pub type GpadcScanPos5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_0(&self) -> GpadcScanPos0R {
        GpadcScanPos0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_1(&self) -> GpadcScanPos1R {
        GpadcScanPos1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_2(&self) -> GpadcScanPos2R {
        GpadcScanPos2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_3(&self) -> GpadcScanPos3R {
        GpadcScanPos3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_4(&self) -> GpadcScanPos4R {
        GpadcScanPos4R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_5(&self) -> GpadcScanPos5R {
        GpadcScanPos5R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_0(&mut self) -> GpadcScanPos0W<'_, GpadcRegScnPos1Spec> {
        GpadcScanPos0W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_1(&mut self) -> GpadcScanPos1W<'_, GpadcRegScnPos1Spec> {
        GpadcScanPos1W::new(self, 5)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_2(&mut self) -> GpadcScanPos2W<'_, GpadcRegScnPos1Spec> {
        GpadcScanPos2W::new(self, 10)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_3(&mut self) -> GpadcScanPos3W<'_, GpadcRegScnPos1Spec> {
        GpadcScanPos3W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_4(&mut self) -> GpadcScanPos4W<'_, GpadcRegScnPos1Spec> {
        GpadcScanPos4W::new(self, 20)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_5(&mut self) -> GpadcScanPos5W<'_, GpadcRegScnPos1Spec> {
        GpadcScanPos5W::new(self, 25)
    }
}
#[doc = "GPADC Conversion Sequence 1 (Positive Channels 0-5)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_pos1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_pos1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegScnPos1Spec;
impl crate::RegisterSpec for GpadcRegScnPos1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_scn_pos1::R`](R) reader structure"]
impl crate::Readable for GpadcRegScnPos1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_scn_pos1::W`](W) writer structure"]
impl crate::Writable for GpadcRegScnPos1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_SCN_POS1 to value 0"]
impl crate::Resettable for GpadcRegScnPos1Spec {}
