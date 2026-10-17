#[doc = "Register `TZC_XRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcXramTzsrgR1Spec>;
#[doc = "Field `tzc_xram_tzsrg_r1_end` reader - "]
pub type TzcXramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_xram_tzsrg_r1_start` reader - "]
pub type TzcXramTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_end(&self) -> TzcXramTzsrgR1EndR {
        TzcXramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_start(&self) -> TzcXramTzsrgR1StartR {
        TzcXramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "XRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgR1Spec;
impl crate::RegisterSpec for TzcXramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcXramTzsrgR1Spec {}
