#[doc = "Register `URX_ABR_PRD_B45` reader"]
pub type R = crate::R<UrxAbrPrdB45Spec>;
#[doc = "Field `sts_urx_abr_prd_bit4` reader - "]
pub type StsUrxAbrPrdBit4R = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_bit5` reader - "]
pub type StsUrxAbrPrdBit5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit4(&self) -> StsUrxAbrPrdBit4R {
        StsUrxAbrPrdBit4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_bit5(&self) -> StsUrxAbrPrdBit5R {
        StsUrxAbrPrdBit5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ABR Bit 4 and 5 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b45::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxAbrPrdB45Spec;
impl crate::RegisterSpec for UrxAbrPrdB45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_abr_prd_b45::R`](R) reader structure"]
impl crate::Readable for UrxAbrPrdB45Spec {}
#[doc = "`reset()` method sets URX_ABR_PRD_B45 to value 0"]
impl crate::Resettable for UrxAbrPrdB45Spec {}
