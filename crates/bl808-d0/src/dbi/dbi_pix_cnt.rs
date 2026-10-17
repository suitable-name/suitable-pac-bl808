#[doc = "Register `DBI_PIX_CNT` reader"]
pub type R = crate::R<DbiPixCntSpec>;
#[doc = "Register `DBI_PIX_CNT` writer"]
pub type W = crate::W<DbiPixCntSpec>;
#[doc = "Field `cr_dbi_pix_cnt` reader - Pixel count for current transfer"]
pub type CrDbiPixCntR = crate::FieldReader<u32>;
#[doc = "Field `cr_dbi_pix_cnt` writer - Pixel count for current transfer"]
pub type CrDbiPixCntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `cr_dbi_pix_format` reader - Pixel format selection (likely RGB565/RGB888)"]
pub type CrDbiPixFormatR = crate::BitReader;
#[doc = "Field `cr_dbi_pix_format` writer - Pixel format selection (likely RGB565/RGB888)"]
pub type CrDbiPixFormatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Pixel count for current transfer"]
    #[inline(always)]
    pub fn cr_dbi_pix_cnt(&self) -> CrDbiPixCntR {
        CrDbiPixCntR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Pixel format selection (likely RGB565/RGB888)"]
    #[inline(always)]
    pub fn cr_dbi_pix_format(&self) -> CrDbiPixFormatR {
        CrDbiPixFormatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Pixel count for current transfer"]
    #[inline(always)]
    pub fn cr_dbi_pix_cnt(&mut self) -> CrDbiPixCntW<'_, DbiPixCntSpec> {
        CrDbiPixCntW::new(self, 0)
    }
    #[doc = "Bit 31 - Pixel format selection (likely RGB565/RGB888)"]
    #[inline(always)]
    pub fn cr_dbi_pix_format(&mut self) -> CrDbiPixFormatW<'_, DbiPixCntSpec> {
        CrDbiPixFormatW::new(self, 31)
    }
}
#[doc = "DBI Pixel Count and Format\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_pix_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_pix_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiPixCntSpec;
impl crate::RegisterSpec for DbiPixCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_pix_cnt::R`](R) reader structure"]
impl crate::Readable for DbiPixCntSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_pix_cnt::W`](W) writer structure"]
impl crate::Writable for DbiPixCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_PIX_CNT to value 0"]
impl crate::Resettable for DbiPixCntSpec {}
