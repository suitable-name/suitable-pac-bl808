#[doc = "Register `GPADC_REG_SCN_POS2` reader"]
pub type R = crate::R<GpadcRegScnPos2Spec>;
#[doc = "Register `GPADC_REG_SCN_POS2` writer"]
pub type W = crate::W<GpadcRegScnPos2Spec>;
#[doc = "Field `gpadc_scan_pos_6` reader - "]
pub type GpadcScanPos6R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_6` writer - "]
pub type GpadcScanPos6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_7` reader - "]
pub type GpadcScanPos7R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_7` writer - "]
pub type GpadcScanPos7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_8` reader - "]
pub type GpadcScanPos8R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_8` writer - "]
pub type GpadcScanPos8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_9` reader - "]
pub type GpadcScanPos9R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_9` writer - "]
pub type GpadcScanPos9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_10` reader - "]
pub type GpadcScanPos10R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_10` writer - "]
pub type GpadcScanPos10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_11` reader - "]
pub type GpadcScanPos11R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_11` writer - "]
pub type GpadcScanPos11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_6(&self) -> GpadcScanPos6R {
        GpadcScanPos6R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_7(&self) -> GpadcScanPos7R {
        GpadcScanPos7R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_8(&self) -> GpadcScanPos8R {
        GpadcScanPos8R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_9(&self) -> GpadcScanPos9R {
        GpadcScanPos9R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_10(&self) -> GpadcScanPos10R {
        GpadcScanPos10R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_11(&self) -> GpadcScanPos11R {
        GpadcScanPos11R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_6(&mut self) -> GpadcScanPos6W<'_, GpadcRegScnPos2Spec> {
        GpadcScanPos6W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_7(&mut self) -> GpadcScanPos7W<'_, GpadcRegScnPos2Spec> {
        GpadcScanPos7W::new(self, 5)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_8(&mut self) -> GpadcScanPos8W<'_, GpadcRegScnPos2Spec> {
        GpadcScanPos8W::new(self, 10)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_9(&mut self) -> GpadcScanPos9W<'_, GpadcRegScnPos2Spec> {
        GpadcScanPos9W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_10(&mut self) -> GpadcScanPos10W<'_, GpadcRegScnPos2Spec> {
        GpadcScanPos10W::new(self, 20)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_11(&mut self) -> GpadcScanPos11W<'_, GpadcRegScnPos2Spec> {
        GpadcScanPos11W::new(self, 25)
    }
}
#[doc = "GPADC Conversion Sequence 2 (Positive Channels 6-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_pos2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_pos2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegScnPos2Spec;
impl crate::RegisterSpec for GpadcRegScnPos2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_scn_pos2::R`](R) reader structure"]
impl crate::Readable for GpadcRegScnPos2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_scn_pos2::W`](W) writer structure"]
impl crate::Writable for GpadcRegScnPos2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_SCN_POS2 to value 0"]
impl crate::Resettable for GpadcRegScnPos2Spec {}
