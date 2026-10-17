#[doc = "Register `URX_ABR_PRD_B01` reader"]
pub type R = crate::R<UrxAbrPrdB01Spec>;
#[doc = "Field `sts_urx_abr_prd_bit0` reader - "]
pub type StsUrxAbrPrdBit0R = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_bit1` reader - "]
pub type StsUrxAbrPrdBit1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit0(&self) -> StsUrxAbrPrdBit0R {
        StsUrxAbrPrdBit0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit1(&self) -> StsUrxAbrPrdBit1R {
        StsUrxAbrPrdBit1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ABR Bit 0 and 1 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b01::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxAbrPrdB01Spec;
impl crate::RegisterSpec for UrxAbrPrdB01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_abr_prd_b01::R`](R) reader structure"]
impl crate::Readable for UrxAbrPrdB01Spec {}
#[doc = "`reset()` method sets URX_ABR_PRD_B01 to value 0"]
impl crate::Resettable for UrxAbrPrdB01Spec {}
