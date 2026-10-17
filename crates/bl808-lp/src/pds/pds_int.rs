#[doc = "Register `PDS_INT` reader"]
pub type R = crate::R<PdsIntSpec>;
#[doc = "Register `PDS_INT` writer"]
pub type W = crate::W<PdsIntSpec>;
#[doc = "Field `ro_pds_wake_int` reader - "]
pub type RoPdsWakeIntR = crate::BitReader;
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub type RoPdsRfDoneIntR = crate::BitReader;
#[doc = "Field `ro_pds_wifi_tbtt_sleep_irq` reader - "]
pub type RoPdsWifiTbttSleepIrqR = crate::BitReader;
#[doc = "Field `ro_pds_wifi_tbtt_wakeup_irq` reader - "]
pub type RoPdsWifiTbttWakeupIrqR = crate::BitReader;
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub type CrPdsWakeIntMaskR = crate::BitReader;
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub type CrPdsWakeIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub type CrPdsRfDoneIntMaskR = crate::BitReader;
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub type CrPdsRfDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wifi_tbtt_sleep_irq_mask` reader - "]
pub type CrPdsWifiTbttSleepIrqMaskR = crate::BitReader;
#[doc = "Field `cr_pds_wifi_tbtt_sleep_irq_mask` writer - "]
pub type CrPdsWifiTbttSleepIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wifi_tbtt_wakeup_irq_mask` reader - "]
pub type CrPdsWifiTbttWakeupIrqMaskR = crate::BitReader;
#[doc = "Field `cr_pds_wifi_tbtt_wakeup_irq_mask` writer - "]
pub type CrPdsWifiTbttWakeupIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_int_clr` reader - "]
pub type CrPdsIntClrR = crate::BitReader;
#[doc = "Field `cr_pds_int_clr` writer - "]
pub type CrPdsIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wakeup_src_en` reader - "]
pub type CrPdsWakeupSrcEnR = crate::FieldReader<u16>;
#[doc = "Field `cr_pds_wakeup_src_en` writer - "]
pub type CrPdsWakeupSrcEnW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ro_pds_wakeup_event` reader - "]
pub type RoPdsWakeupEventR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RoPdsWakeIntR {
        RoPdsWakeIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RoPdsRfDoneIntR {
        RoPdsRfDoneIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_wifi_tbtt_sleep_irq(&self) -> RoPdsWifiTbttSleepIrqR {
        RoPdsWifiTbttSleepIrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_wifi_tbtt_wakeup_irq(&self) -> RoPdsWifiTbttWakeupIrqR {
        RoPdsWifiTbttWakeupIrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CrPdsWakeIntMaskR {
        CrPdsWakeIntMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CrPdsRfDoneIntMaskR {
        CrPdsRfDoneIntMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_sleep_irq_mask(&self) -> CrPdsWifiTbttSleepIrqMaskR {
        CrPdsWifiTbttSleepIrqMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_wakeup_irq_mask(&self) -> CrPdsWifiTbttWakeupIrqMaskR {
        CrPdsWifiTbttWakeupIrqMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CrPdsIntClrR {
        CrPdsIntClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:20"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&self) -> CrPdsWakeupSrcEnR {
        CrPdsWakeupSrcEnR::new(((self.bits >> 10) & 0x07ff) as u16)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&self) -> RoPdsWakeupEventR {
        RoPdsWakeupEventR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&mut self) -> CrPdsWakeIntMaskW<'_, PdsIntSpec> {
        CrPdsWakeIntMaskW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CrPdsRfDoneIntMaskW<'_, PdsIntSpec> {
        CrPdsRfDoneIntMaskW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_sleep_irq_mask(
        &mut self,
    ) -> CrPdsWifiTbttSleepIrqMaskW<'_, PdsIntSpec> {
        CrPdsWifiTbttSleepIrqMaskW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_wakeup_irq_mask(
        &mut self,
    ) -> CrPdsWifiTbttWakeupIrqMaskW<'_, PdsIntSpec> {
        CrPdsWifiTbttWakeupIrqMaskW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&mut self) -> CrPdsIntClrW<'_, PdsIntSpec> {
        CrPdsIntClrW::new(self, 8)
    }
    #[doc = "Bits 10:20"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&mut self) -> CrPdsWakeupSrcEnW<'_, PdsIntSpec> {
        CrPdsWakeupSrcEnW::new(self, 10)
    }
}
#[doc = "PDS Interrupt Status and Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsIntSpec;
impl crate::RegisterSpec for PdsIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_int::R`](R) reader structure"]
impl crate::Readable for PdsIntSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_int::W`](W) writer structure"]
impl crate::Writable for PdsIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_INT to value 0x001f_fc00"]
impl crate::Resettable for PdsIntSpec {
    const RESET_VALUE: u32 = 0x001f_fc00;
}
