#[doc = "Register `TZC_XRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcXramTzsrgR0Spec>;
#[doc = "Field `tzc_xram_tzsrg_r0_end` reader - "]
pub type TzcXramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_xram_tzsrg_r0_start` reader - "]
pub type TzcXramTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_end(&self) -> TzcXramTzsrgR0EndR {
        TzcXramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_start(&self) -> TzcXramTzsrgR0StartR {
        TzcXramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "XRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgR0Spec;
impl crate::RegisterSpec for TzcXramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcXramTzsrgR0Spec {}
