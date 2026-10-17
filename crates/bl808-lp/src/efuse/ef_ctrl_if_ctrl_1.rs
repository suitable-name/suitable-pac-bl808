#[doc = "Register `EF_CTRL_IF_CTRL_1` reader"]
pub type R = crate::R<EfCtrlIfCtrl1Spec>;
#[doc = "Register `EF_CTRL_IF_CTRL_1` writer"]
pub type W = crate::W<EfCtrlIfCtrl1Spec>;
#[doc = "Field `ef_if_1_busy` reader - eFuse interface 1 busy status"]
pub type EfIf1BusyR = crate::BitReader;
#[doc = "Field `ef_if_1_rw` reader - 0: Read operation, 1: Write/Program operation"]
pub type EfIf1RwR = crate::BitReader;
#[doc = "Field `ef_if_1_rw` writer - 0: Read operation, 1: Write/Program operation"]
pub type EfIf1RwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_1_trig` reader - Trigger read/write operation (active high)"]
pub type EfIf1TrigR = crate::BitReader;
#[doc = "Field `ef_if_1_trig` writer - Trigger read/write operation (active high)"]
pub type EfIf1TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_1_manual_en` reader - Manual mode enable"]
pub type EfIf1ManualEnR = crate::BitReader;
#[doc = "Field `ef_if_1_manual_en` writer - Manual mode enable"]
pub type EfIf1ManualEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_1_cyc_modify` reader - eFuse cycle timing modification enable"]
pub type EfIf1CycModifyR = crate::BitReader;
#[doc = "Field `ef_if_1_cyc_modify` writer - eFuse cycle timing modification enable"]
pub type EfIf1CycModifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_1_int` reader - eFuse interface 1 interrupt status"]
pub type EfIf1IntR = crate::BitReader;
#[doc = "Field `ef_if_1_int_clr` reader - Clear eFuse interface 1 interrupt"]
pub type EfIf1IntClrR = crate::BitReader;
#[doc = "Field `ef_if_1_int_clr` writer - Clear eFuse interface 1 interrupt"]
pub type EfIf1IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ef_if_1_int_set` reader - Set eFuse interface 1 interrupt"]
pub type EfIf1IntSetR = crate::BitReader;
#[doc = "Field `ef_if_1_int_set` writer - Set eFuse interface 1 interrupt"]
pub type EfIf1IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - eFuse interface 1 busy status"]
    #[inline(always)]
    pub fn ef_if_1_busy(&self) -> EfIf1BusyR {
        EfIf1BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Read operation, 1: Write/Program operation"]
    #[inline(always)]
    pub fn ef_if_1_rw(&self) -> EfIf1RwR {
        EfIf1RwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Trigger read/write operation (active high)"]
    #[inline(always)]
    pub fn ef_if_1_trig(&self) -> EfIf1TrigR {
        EfIf1TrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Manual mode enable"]
    #[inline(always)]
    pub fn ef_if_1_manual_en(&self) -> EfIf1ManualEnR {
        EfIf1ManualEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - eFuse cycle timing modification enable"]
    #[inline(always)]
    pub fn ef_if_1_cyc_modify(&self) -> EfIf1CycModifyR {
        EfIf1CycModifyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 20 - eFuse interface 1 interrupt status"]
    #[inline(always)]
    pub fn ef_if_1_int(&self) -> EfIf1IntR {
        EfIf1IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear eFuse interface 1 interrupt"]
    #[inline(always)]
    pub fn ef_if_1_int_clr(&self) -> EfIf1IntClrR {
        EfIf1IntClrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set eFuse interface 1 interrupt"]
    #[inline(always)]
    pub fn ef_if_1_int_set(&self) -> EfIf1IntSetR {
        EfIf1IntSetR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: Read operation, 1: Write/Program operation"]
    #[inline(always)]
    pub fn ef_if_1_rw(&mut self) -> EfIf1RwW<'_, EfCtrlIfCtrl1Spec> {
        EfIf1RwW::new(self, 3)
    }
    #[doc = "Bit 4 - Trigger read/write operation (active high)"]
    #[inline(always)]
    pub fn ef_if_1_trig(&mut self) -> EfIf1TrigW<'_, EfCtrlIfCtrl1Spec> {
        EfIf1TrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Manual mode enable"]
    #[inline(always)]
    pub fn ef_if_1_manual_en(&mut self) -> EfIf1ManualEnW<'_, EfCtrlIfCtrl1Spec> {
        EfIf1ManualEnW::new(self, 5)
    }
    #[doc = "Bit 6 - eFuse cycle timing modification enable"]
    #[inline(always)]
    pub fn ef_if_1_cyc_modify(&mut self) -> EfIf1CycModifyW<'_, EfCtrlIfCtrl1Spec> {
        EfIf1CycModifyW::new(self, 6)
    }
    #[doc = "Bit 21 - Clear eFuse interface 1 interrupt"]
    #[inline(always)]
    pub fn ef_if_1_int_clr(&mut self) -> EfIf1IntClrW<'_, EfCtrlIfCtrl1Spec> {
        EfIf1IntClrW::new(self, 21)
    }
    #[doc = "Bit 22 - Set eFuse interface 1 interrupt"]
    #[inline(always)]
    pub fn ef_if_1_int_set(&mut self) -> EfIf1IntSetW<'_, EfCtrlIfCtrl1Spec> {
        EfIf1IntSetW::new(self, 22)
    }
}
#[doc = "eFuse Interface 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfCtrl1Spec;
impl crate::RegisterSpec for EfCtrlIfCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_ctrl_1::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_ctrl_1::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfCtrl1Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0020_0000;
}
#[doc = "`reset()` method sets EF_CTRL_IF_CTRL_1 to value 0x0020_0000"]
impl crate::Resettable for EfCtrlIfCtrl1Spec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
