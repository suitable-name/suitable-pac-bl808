#[doc = "Register `STS_URX_ABR_PRD` reader"]
pub type R = crate::R<StsUrxAbrPrdSpec>;
#[doc = "Field `sts_urx_abr_prd_start` reader - "]
pub type StsUrxAbrPrdStartR = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_0x55` reader - "]
pub type StsUrxAbrPrd0x55R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&self) -> StsUrxAbrPrdStartR {
        StsUrxAbrPrdStartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&self) -> StsUrxAbrPrd0x55R {
        StsUrxAbrPrd0x55R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Auto Baud Rate Detection Results\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_urx_abr_prd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsUrxAbrPrdSpec;
impl crate::RegisterSpec for StsUrxAbrPrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts_urx_abr_prd::R`](R) reader structure"]
impl crate::Readable for StsUrxAbrPrdSpec {}
#[doc = "`reset()` method sets STS_URX_ABR_PRD to value 0"]
impl crate::Resettable for StsUrxAbrPrdSpec {}
