#[doc = "Register `SD_SLOT_INT_STATUS` reader"]
pub type R = crate::R<SdSlotIntStatusSpec>;
#[doc = "Field `SLOT_INT0` reader - "]
pub type SlotInt0R = crate::BitReader;
#[doc = "Field `SLOT_INT1` reader - "]
pub type SlotInt1R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slot_int0(&self) -> SlotInt0R {
        SlotInt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slot_int1(&self) -> SlotInt1R {
        SlotInt1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Slot Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_slot_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdSlotIntStatusSpec;
impl crate::RegisterSpec for SdSlotIntStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_slot_int_status::R`](R) reader structure"]
impl crate::Readable for SdSlotIntStatusSpec {}
#[doc = "`reset()` method sets SD_SLOT_INT_STATUS to value 0"]
impl crate::Resettable for SdSlotIntStatusSpec {}
