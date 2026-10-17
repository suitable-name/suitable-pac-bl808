#[doc = "Register `EF_CTRL_IF_CTRL_0` reader"]
pub type R = crate::R<EfCtrlIfCtrl0Spec>;
#[doc = "Register `EF_CTRL_IF_CTRL_0` writer"]
pub type W = crate::W<EfCtrlIfCtrl0Spec>;
#[doc = "Field `ef_if_0_autoload_p1_done` reader - Autoload Phase 1 completion status"]
pub type EfIf0AutoloadP1DoneR = crate::BitReader;
#[doc = "Field `ef_if_0_autoload_done` reader - Autoload complete status"]
pub type EfIf0AutoloadDoneR = crate::BitReader;
#[doc = "Field `ef_if_0_busy` reader - eFuse interface busy status"]
pub type EfIf0BusyR = crate::BitReader;
#[doc = "Field `ef_if_0_rw` reader - 0: Read operation, 1: Write/Program operation"]
pub type EfIf0RwR = crate::BitReader;
#[doc = "Field `ef_if_0_rw` writer - 0: Read operation, 1: Write/Program operation"]
pub type EfIf0RwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_trig` reader - Trigger read/write operation (active high)"]
pub type EfIf0TrigR = crate::BitReader;
#[doc = "Field `ef_if_0_trig` writer - Trigger read/write operation (active high)"]
pub type EfIf0TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_manual_en` reader - Manual mode enable (disables hardware sequencer)"]
pub type EfIf0ManualEnR = crate::BitReader;
#[doc = "Field `ef_if_0_manual_en` writer - Manual mode enable (disables hardware sequencer)"]
pub type EfIf0ManualEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_cyc_modify` reader - eFuse cycle timing modification enable"]
pub type EfIf0CycModifyR = crate::BitReader;
#[doc = "Field `ef_if_0_cyc_modify` writer - eFuse cycle timing modification enable"]
pub type EfIf0CycModifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_prot_code_ctrl` reader - eFuse protection code for control interface"]
pub type EfIfProtCodeCtrlR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_ctrl` writer - eFuse protection code for control interface"]
pub type EfIfProtCodeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ef_if_por_dig` reader - Power on reset digital logic"]
pub type EfIfPorDigR = crate::BitReader;
#[doc = "Field `ef_if_por_dig` writer - Power on reset digital logic"]
pub type EfIfPorDigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_pclk_force_on` reader - Force peripheral clock on"]
pub type EfPclkForceOnR = crate::BitReader;
#[doc = "Field `ef_pclk_force_on` writer - Force peripheral clock on"]
pub type EfPclkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_auto_rd_en` reader - Auto read enable after program"]
pub type EfIfAutoRdEnR = crate::BitReader;
#[doc = "Field `ef_if_auto_rd_en` writer - Auto read enable after program"]
pub type EfIfAutoRdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_cyc_modify_lock` reader - Lock eFuse cycle modification"]
pub type EfIfCycModifyLockR = crate::BitReader;
#[doc = "Field `ef_if_cyc_modify_lock` writer - Lock eFuse cycle modification"]
pub type EfIfCycModifyLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_int` reader - eFuse interface 0 interrupt status"]
pub type EfIf0IntR = crate::BitReader;
#[doc = "Field `ef_if_0_int_clr` reader - Clear eFuse interface 0 interrupt"]
pub type EfIf0IntClrR = crate::BitReader;
#[doc = "Field `ef_if_0_int_clr` writer - Clear eFuse interface 0 interrupt"]
pub type EfIf0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ef_if_0_int_set` reader - Set eFuse interface 0 interrupt"]
pub type EfIf0IntSetR = crate::BitReader;
#[doc = "Field `ef_if_0_int_set` writer - Set eFuse interface 0 interrupt"]
pub type EfIf0IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_prot_code_cyc` reader - eFuse protection code for cycle modification"]
pub type EfIfProtCodeCycR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_cyc` writer - eFuse protection code for cycle modification"]
pub type EfIfProtCodeCycW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Autoload Phase 1 completion status"]
    #[inline(always)]
    pub fn ef_if_0_autoload_p1_done(&self) -> EfIf0AutoloadP1DoneR {
        EfIf0AutoloadP1DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoload complete status"]
    #[inline(always)]
    pub fn ef_if_0_autoload_done(&self) -> EfIf0AutoloadDoneR {
        EfIf0AutoloadDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - eFuse interface busy status"]
    #[inline(always)]
    pub fn ef_if_0_busy(&self) -> EfIf0BusyR {
        EfIf0BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Read operation, 1: Write/Program operation"]
    #[inline(always)]
    pub fn ef_if_0_rw(&self) -> EfIf0RwR {
        EfIf0RwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Trigger read/write operation (active high)"]
    #[inline(always)]
    pub fn ef_if_0_trig(&self) -> EfIf0TrigR {
        EfIf0TrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Manual mode enable (disables hardware sequencer)"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&self) -> EfIf0ManualEnR {
        EfIf0ManualEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - eFuse cycle timing modification enable"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&self) -> EfIf0CycModifyR {
        EfIf0CycModifyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - eFuse protection code for control interface"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&self) -> EfIfProtCodeCtrlR {
        EfIfProtCodeCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Power on reset digital logic"]
    #[inline(always)]
    pub fn ef_if_por_dig(&self) -> EfIfPorDigR {
        EfIfPorDigR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force peripheral clock on"]
    #[inline(always)]
    pub fn ef_pclk_force_on(&self) -> EfPclkForceOnR {
        EfPclkForceOnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Auto read enable after program"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&self) -> EfIfAutoRdEnR {
        EfIfAutoRdEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock eFuse cycle modification"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&self) -> EfIfCycModifyLockR {
        EfIfCycModifyLockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - eFuse interface 0 interrupt status"]
    #[inline(always)]
    pub fn ef_if_0_int(&self) -> EfIf0IntR {
        EfIf0IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear eFuse interface 0 interrupt"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&self) -> EfIf0IntClrR {
        EfIf0IntClrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set eFuse interface 0 interrupt"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&self) -> EfIf0IntSetR {
        EfIf0IntSetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:31 - eFuse protection code for cycle modification"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&self) -> EfIfProtCodeCycR {
        EfIfProtCodeCycR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - 0: Read operation, 1: Write/Program operation"]
    #[inline(always)]
    pub fn ef_if_0_rw(&mut self) -> EfIf0RwW<'_, EfCtrlIfCtrl0Spec> {
        EfIf0RwW::new(self, 3)
    }
    #[doc = "Bit 4 - Trigger read/write operation (active high)"]
    #[inline(always)]
    pub fn ef_if_0_trig(&mut self) -> EfIf0TrigW<'_, EfCtrlIfCtrl0Spec> {
        EfIf0TrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Manual mode enable (disables hardware sequencer)"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&mut self) -> EfIf0ManualEnW<'_, EfCtrlIfCtrl0Spec> {
        EfIf0ManualEnW::new(self, 5)
    }
    #[doc = "Bit 6 - eFuse cycle timing modification enable"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&mut self) -> EfIf0CycModifyW<'_, EfCtrlIfCtrl0Spec> {
        EfIf0CycModifyW::new(self, 6)
    }
    #[doc = "Bits 8:15 - eFuse protection code for control interface"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&mut self) -> EfIfProtCodeCtrlW<'_, EfCtrlIfCtrl0Spec> {
        EfIfProtCodeCtrlW::new(self, 8)
    }
    #[doc = "Bit 16 - Power on reset digital logic"]
    #[inline(always)]
    pub fn ef_if_por_dig(&mut self) -> EfIfPorDigW<'_, EfCtrlIfCtrl0Spec> {
        EfIfPorDigW::new(self, 16)
    }
    #[doc = "Bit 17 - Force peripheral clock on"]
    #[inline(always)]
    pub fn ef_pclk_force_on(&mut self) -> EfPclkForceOnW<'_, EfCtrlIfCtrl0Spec> {
        EfPclkForceOnW::new(self, 17)
    }
    #[doc = "Bit 18 - Auto read enable after program"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&mut self) -> EfIfAutoRdEnW<'_, EfCtrlIfCtrl0Spec> {
        EfIfAutoRdEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Lock eFuse cycle modification"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&mut self) -> EfIfCycModifyLockW<'_, EfCtrlIfCtrl0Spec> {
        EfIfCycModifyLockW::new(self, 19)
    }
    #[doc = "Bit 21 - Clear eFuse interface 0 interrupt"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&mut self) -> EfIf0IntClrW<'_, EfCtrlIfCtrl0Spec> {
        EfIf0IntClrW::new(self, 21)
    }
    #[doc = "Bit 22 - Set eFuse interface 0 interrupt"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&mut self) -> EfIf0IntSetW<'_, EfCtrlIfCtrl0Spec> {
        EfIf0IntSetW::new(self, 22)
    }
    #[doc = "Bits 24:31 - eFuse protection code for cycle modification"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&mut self) -> EfIfProtCodeCycW<'_, EfCtrlIfCtrl0Spec> {
        EfIfProtCodeCycW::new(self, 24)
    }
}
#[doc = "eFuse Interface 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfCtrl0Spec;
impl crate::RegisterSpec for EfCtrlIfCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_ctrl_0::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_ctrl_0::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfCtrl0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0020_0000;
}
#[doc = "`reset()` method sets EF_CTRL_IF_CTRL_0 to value 0x0024_0003"]
impl crate::Resettable for EfCtrlIfCtrl0Spec {
    const RESET_VALUE: u32 = 0x0024_0003;
}
