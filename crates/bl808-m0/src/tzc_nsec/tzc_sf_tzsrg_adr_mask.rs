#[doc = "Register `TZC_SF_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcSfTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask` reader - "]
pub type TzcSfTzsrgAdrMaskR = crate::FieldReader<u32>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` reader - "]
pub type TzcSfTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask(&self) -> TzcSfTzsrgAdrMaskR {
        TzcSfTzsrgAdrMaskR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask_lock(&self) -> TzcSfTzsrgAdrMaskLockR {
        TzcSfTzsrgAdrMaskLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SF TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcSfTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcSfTzsrgAdrMaskSpec {}
