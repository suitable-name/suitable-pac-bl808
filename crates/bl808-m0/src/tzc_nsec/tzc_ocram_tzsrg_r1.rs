#[doc = "Register `TZC_OCRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcOcramTzsrgR1Spec>;
#[doc = "Field `tzc_ocram_tzsrg_r1_end` reader - "]
pub type TzcOcramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r1_start` reader - "]
pub type TzcOcramTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_end(&self) -> TzcOcramTzsrgR1EndR {
        TzcOcramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_start(&self) -> TzcOcramTzsrgR1StartR {
        TzcOcramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "OCRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgR1Spec;
impl crate::RegisterSpec for TzcOcramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcOcramTzsrgR1Spec {}
