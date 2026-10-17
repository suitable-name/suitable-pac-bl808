#[doc = "Register `OTG_ISR` reader"]
pub type R = crate::R<OtgIsrSpec>;
#[doc = "Register `OTG_ISR` writer"]
pub type W = crate::W<OtgIsrSpec>;
#[doc = "Field `B_SRP_DN` reader - B-device SRP done (W1C)"]
pub type BSrpDnR = crate::BitReader;
#[doc = "Field `B_SRP_DN` writer - B-device SRP done (W1C)"]
pub type BSrpDnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_SRP_DET` reader - A-device SRP Detect (W1C)"]
pub type ASrpDetR = crate::BitReader;
#[doc = "Field `A_SRP_DET` writer - A-device SRP Detect (W1C)"]
pub type ASrpDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_VBUS_ERR_HOV` reader - A-device VBUS Error (Host Overwrite) (W1C)"]
pub type AVbusErrHovR = crate::BitReader;
#[doc = "Field `A_VBUS_ERR_HOV` writer - A-device VBUS Error (Host Overwrite) (W1C)"]
pub type AVbusErrHovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `B_SESS_END_POV` reader - B-Session End Interrupt (Peripheral Overwrite) (W1C)"]
pub type BSessEndPovR = crate::BitReader;
#[doc = "Field `B_SESS_END_POV` writer - B-Session End Interrupt (Peripheral Overwrite) (W1C)"]
pub type BSessEndPovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RLCHG` reader - Role Change Interrupt (W1C)"]
pub type RlchgR = crate::BitReader;
#[doc = "Field `RLCHG` writer - Role Change Interrupt (W1C)"]
pub type RlchgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IDCHG` reader - ID Change Interrupt (W1C)"]
pub type IdchgR = crate::BitReader;
#[doc = "Field `IDCHG` writer - ID Change Interrupt (W1C)"]
pub type IdchgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVC_HOV` reader - Over-Current Interrupt (Host Overwrite) (W1C)"]
pub type OvcHovR = crate::BitReader;
#[doc = "Field `OVC_HOV` writer - Over-Current Interrupt (Host Overwrite) (W1C)"]
pub type OvcHovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `A_WAIT_CON_HOV` reader - A-Wait-Connect Interrupt (Host Overwrite) (W1C)"]
pub type AWaitConHovR = crate::BitReader;
#[doc = "Field `A_WAIT_CON_HOV` writer - A-Wait-Connect Interrupt (Host Overwrite) (W1C)"]
pub type AWaitConHovW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APLGRMV` reader - A-Plug-Remove Interrupt (W1C)"]
pub type AplgrmvR = crate::BitReader;
#[doc = "Field `APLGRMV` writer - A-Plug-Remove Interrupt (W1C)"]
pub type AplgrmvW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - B-device SRP done (W1C)"]
    #[inline(always)]
    pub fn b_srp_dn(&self) -> BSrpDnR {
        BSrpDnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - A-device SRP Detect (W1C)"]
    #[inline(always)]
    pub fn a_srp_det(&self) -> ASrpDetR {
        ASrpDetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-device VBUS Error (Host Overwrite) (W1C)"]
    #[inline(always)]
    pub fn a_vbus_err_hov(&self) -> AVbusErrHovR {
        AVbusErrHovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B-Session End Interrupt (Peripheral Overwrite) (W1C)"]
    #[inline(always)]
    pub fn b_sess_end_pov(&self) -> BSessEndPovR {
        BSessEndPovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Role Change Interrupt (W1C)"]
    #[inline(always)]
    pub fn rlchg(&self) -> RlchgR {
        RlchgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ID Change Interrupt (W1C)"]
    #[inline(always)]
    pub fn idchg(&self) -> IdchgR {
        IdchgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Over-Current Interrupt (Host Overwrite) (W1C)"]
    #[inline(always)]
    pub fn ovc_hov(&self) -> OvcHovR {
        OvcHovR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A-Wait-Connect Interrupt (Host Overwrite) (W1C)"]
    #[inline(always)]
    pub fn a_wait_con_hov(&self) -> AWaitConHovR {
        AWaitConHovR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A-Plug-Remove Interrupt (W1C)"]
    #[inline(always)]
    pub fn aplgrmv(&self) -> AplgrmvR {
        AplgrmvR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B-device SRP done (W1C)"]
    #[inline(always)]
    pub fn b_srp_dn(&mut self) -> BSrpDnW<'_, OtgIsrSpec> {
        BSrpDnW::new(self, 0)
    }
    #[doc = "Bit 4 - A-device SRP Detect (W1C)"]
    #[inline(always)]
    pub fn a_srp_det(&mut self) -> ASrpDetW<'_, OtgIsrSpec> {
        ASrpDetW::new(self, 4)
    }
    #[doc = "Bit 5 - A-device VBUS Error (Host Overwrite) (W1C)"]
    #[inline(always)]
    pub fn a_vbus_err_hov(&mut self) -> AVbusErrHovW<'_, OtgIsrSpec> {
        AVbusErrHovW::new(self, 5)
    }
    #[doc = "Bit 6 - B-Session End Interrupt (Peripheral Overwrite) (W1C)"]
    #[inline(always)]
    pub fn b_sess_end_pov(&mut self) -> BSessEndPovW<'_, OtgIsrSpec> {
        BSessEndPovW::new(self, 6)
    }
    #[doc = "Bit 8 - Role Change Interrupt (W1C)"]
    #[inline(always)]
    pub fn rlchg(&mut self) -> RlchgW<'_, OtgIsrSpec> {
        RlchgW::new(self, 8)
    }
    #[doc = "Bit 9 - ID Change Interrupt (W1C)"]
    #[inline(always)]
    pub fn idchg(&mut self) -> IdchgW<'_, OtgIsrSpec> {
        IdchgW::new(self, 9)
    }
    #[doc = "Bit 10 - Over-Current Interrupt (Host Overwrite) (W1C)"]
    #[inline(always)]
    pub fn ovc_hov(&mut self) -> OvcHovW<'_, OtgIsrSpec> {
        OvcHovW::new(self, 10)
    }
    #[doc = "Bit 11 - A-Wait-Connect Interrupt (Host Overwrite) (W1C)"]
    #[inline(always)]
    pub fn a_wait_con_hov(&mut self) -> AWaitConHovW<'_, OtgIsrSpec> {
        AWaitConHovW::new(self, 11)
    }
    #[doc = "Bit 12 - A-Plug-Remove Interrupt (W1C)"]
    #[inline(always)]
    pub fn aplgrmv(&mut self) -> AplgrmvW<'_, OtgIsrSpec> {
        AplgrmvW::new(self, 12)
    }
}
#[doc = "OTG Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgIsrSpec;
impl crate::RegisterSpec for OtgIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_isr::R`](R) reader structure"]
impl crate::Readable for OtgIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_isr::W`](W) writer structure"]
impl crate::Writable for OtgIsrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f71;
}
#[doc = "`reset()` method sets OTG_ISR to value 0"]
impl crate::Resettable for OtgIsrSpec {}
