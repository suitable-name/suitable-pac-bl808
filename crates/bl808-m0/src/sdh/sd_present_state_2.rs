#[doc = "Register `SD_PRESENT_STATE_2` reader"]
pub type R = crate::R<SdPresentState2Spec>;
#[doc = "Field `CARD_INSERTED` reader - "]
pub type CardInsertedR = crate::BitReader;
#[doc = "Field `CARD_STABLE` reader - "]
pub type CardStableR = crate::BitReader;
#[doc = "Field `CARD_DET` reader - "]
pub type CardDetR = crate::BitReader;
#[doc = "Field `WRITE_PROT` reader - "]
pub type WriteProtR = crate::BitReader;
#[doc = "Field `DAT_LEVEL` reader - "]
pub type DatLevelR = crate::FieldReader;
#[doc = "Field `CMD_LEVEL` reader - "]
pub type CmdLevelR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn card_inserted(&self) -> CardInsertedR {
        CardInsertedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn card_stable(&self) -> CardStableR {
        CardStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn card_det(&self) -> CardDetR {
        CardDetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn write_prot(&self) -> WriteProtR {
        WriteProtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dat_level(&self) -> DatLevelR {
        DatLevelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cmd_level(&self) -> CmdLevelR {
        CmdLevelR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Present State Register 2 (Card Detection)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_present_state_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdPresentState2Spec;
impl crate::RegisterSpec for SdPresentState2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_present_state_2::R`](R) reader structure"]
impl crate::Readable for SdPresentState2Spec {}
#[doc = "`reset()` method sets SD_PRESENT_STATE_2 to value 0"]
impl crate::Resettable for SdPresentState2Spec {}
