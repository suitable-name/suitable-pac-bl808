#[doc = "Register `SOC_INFO0` reader"]
pub type R = crate::R<SocInfo0Spec>;
#[doc = "Field `chip_rdy` reader - "]
pub type ChipRdyR = crate::BitReader;
#[doc = "Field `glb_id` reader - "]
pub type GlbIdR = crate::FieldReader;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&self) -> ChipRdyR {
        ChipRdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&self) -> GlbIdR {
        GlbIdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "SoC Information 0\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocInfo0Spec;
impl crate::RegisterSpec for SocInfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_info0::R`](R) reader structure"]
impl crate::Readable for SocInfo0Spec {}
#[doc = "`reset()` method sets SOC_INFO0 to value 0"]
impl crate::Resettable for SocInfo0Spec {}
