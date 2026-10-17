#[doc = "Register `PIX_DATA_RANGE` reader"]
pub type R = crate::R<PixDataRangeSpec>;
#[doc = "Register `PIX_DATA_RANGE` writer"]
pub type W = crate::W<PixDataRangeSpec>;
#[doc = "Field `cr_data_min` reader - Minimum pixel data value"]
pub type CrDataMinR = crate::FieldReader<u16>;
#[doc = "Field `cr_data_min` writer - Minimum pixel data value"]
pub type CrDataMinW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cr_data_max` reader - Maximum pixel data value"]
pub type CrDataMaxR = crate::FieldReader<u16>;
#[doc = "Field `cr_data_max` writer - Maximum pixel data value"]
pub type CrDataMaxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum pixel data value"]
    #[inline(always)]
    pub fn cr_data_min(&self) -> CrDataMinR {
        CrDataMinR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Maximum pixel data value"]
    #[inline(always)]
    pub fn cr_data_max(&self) -> CrDataMaxR {
        CrDataMaxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum pixel data value"]
    #[inline(always)]
    pub fn cr_data_min(&mut self) -> CrDataMinW<'_, PixDataRangeSpec> {
        CrDataMinW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Maximum pixel data value"]
    #[inline(always)]
    pub fn cr_data_max(&mut self) -> CrDataMaxW<'_, PixDataRangeSpec> {
        CrDataMaxW::new(self, 16)
    }
}
#[doc = "Pixel Data Range (Clipping) Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pix_data_range::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pix_data_range::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PixDataRangeSpec;
impl crate::RegisterSpec for PixDataRangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pix_data_range::R`](R) reader structure"]
impl crate::Readable for PixDataRangeSpec {}
#[doc = "`write(|w| ..)` method takes [`pix_data_range::W`](W) writer structure"]
impl crate::Writable for PixDataRangeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIX_DATA_RANGE to value 0xffff_0000"]
impl crate::Resettable for PixDataRangeSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
