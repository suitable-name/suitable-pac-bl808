#[doc = "Register `URX_ABR_PRD_B67` reader"]
pub type R = crate::R<UrxAbrPrdB67Spec>;
#[doc = "Field `sts_urx_abr_prd_bit6` reader - "]
pub type StsUrxAbrPrdBit6R = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_bit7` reader - "]
pub type StsUrxAbrPrdBit7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit6(&self) -> StsUrxAbrPrdBit6R {
        StsUrxAbrPrdBit6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit7(&self) -> StsUrxAbrPrdBit7R {
        StsUrxAbrPrdBit7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ABR Bit 6 and 7 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b67::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxAbrPrdB67Spec;
impl crate::RegisterSpec for UrxAbrPrdB67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_abr_prd_b67::R`](R) reader structure"]
impl crate::Readable for UrxAbrPrdB67Spec {}
#[doc = "`reset()` method sets URX_ABR_PRD_B67 to value 0"]
impl crate::Resettable for UrxAbrPrdB67Spec {}
