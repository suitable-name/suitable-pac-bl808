#[doc = "Register `URX_ABR_PRD_B23` reader"]
pub type R = crate::R<UrxAbrPrdB23Spec>;
#[doc = "Field `sts_urx_abr_prd_bit2` reader - "]
pub type StsUrxAbrPrdBit2R = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_bit3` reader - "]
pub type StsUrxAbrPrdBit3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit2(&self) -> StsUrxAbrPrdBit2R {
        StsUrxAbrPrdBit2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit3(&self) -> StsUrxAbrPrdBit3R {
        StsUrxAbrPrdBit3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ABR Bit 2 and 3 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxAbrPrdB23Spec;
impl crate::RegisterSpec for UrxAbrPrdB23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_abr_prd_b23::R`](R) reader structure"]
impl crate::Readable for UrxAbrPrdB23Spec {}
#[doc = "`reset()` method sets URX_ABR_PRD_B23 to value 0"]
impl crate::Resettable for UrxAbrPrdB23Spec {}
