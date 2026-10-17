#[doc = "Register `OTG_IER` reader"]
pub type R = crate::R<OtgIerSpec>;
#[doc = "Register `OTG_IER` writer"]
pub type W = crate::W<OtgIerSpec>;
#[doc = "Field `B_SRP_DN_EN` reader - "]
pub type BSrpDnEnR = crate::BitReader;
#[doc = "Field `B_SRP_DN_EN` writer - "]
pub type BSrpDnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_SRP_DET_INT_EN` reader - "]
pub type ASrpDetIntEnR = crate::BitReader;
#[doc = "Field `A_SRP_DET_INT_EN` writer - "]
pub type ASrpDetIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_VBUS_ERR_EN_HOV` reader - "]
pub type AVbusErrEnHovR = crate::BitReader;
#[doc = "Field `A_VBUS_ERR_EN_HOV` writer - "]
pub type AVbusErrEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_SESS_END_EN_POV` reader - "]
pub type BSessEndEnPovR = crate::BitReader;
#[doc = "Field `B_SESS_END_EN_POV` writer - "]
pub type BSessEndEnPovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLCHG_EN` reader - "]
pub type RlchgEnR = crate::BitReader;
#[doc = "Field `RLCHG_EN` writer - "]
pub type RlchgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDCHG_EN` reader - "]
pub type IdchgEnR = crate::BitReader;
#[doc = "Field `IDCHG_EN` writer - "]
pub type IdchgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVC_EN_HOV` reader - "]
pub type OvcEnHovR = crate::BitReader;
#[doc = "Field `OVC_EN_HOV` writer - "]
pub type OvcEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_WAIT_CON_EN_HOV` reader - "]
pub type AWaitConEnHovR = crate::BitReader;
#[doc = "Field `A_WAIT_CON_EN_HOV` writer - "]
pub type AWaitConEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APLGRMV_EN` reader - "]
pub type AplgrmvEnR = crate::BitReader;
#[doc = "Field `APLGRMV_EN` writer - "]
pub type AplgrmvEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn b_srp_dn_en(&self) -> BSrpDnEnR {
        BSrpDnEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn a_srp_det_int_en(&self) -> ASrpDetIntEnR {
        ASrpDetIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn a_vbus_err_en_hov(&self) -> AVbusErrEnHovR {
        AVbusErrEnHovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn b_sess_end_en_pov(&self) -> BSessEndEnPovR {
        BSessEndEnPovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rlchg_en(&self) -> RlchgEnR {
        RlchgEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idchg_en(&self) -> IdchgEnR {
        IdchgEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovc_en_hov(&self) -> OvcEnHovR {
        OvcEnHovR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn a_wait_con_en_hov(&self) -> AWaitConEnHovR {
        AWaitConEnHovR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn aplgrmv_en(&self) -> AplgrmvEnR {
        AplgrmvEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn b_srp_dn_en(&mut self) -> BSrpDnEnW<'_, OtgIerSpec> {
        BSrpDnEnW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn a_srp_det_int_en(&mut self) -> ASrpDetIntEnW<'_, OtgIerSpec> {
        ASrpDetIntEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn a_vbus_err_en_hov(&mut self) -> AVbusErrEnHovW<'_, OtgIerSpec> {
        AVbusErrEnHovW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn b_sess_end_en_pov(&mut self) -> BSessEndEnPovW<'_, OtgIerSpec> {
        BSessEndEnPovW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rlchg_en(&mut self) -> RlchgEnW<'_, OtgIerSpec> {
        RlchgEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idchg_en(&mut self) -> IdchgEnW<'_, OtgIerSpec> {
        IdchgEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovc_en_hov(&mut self) -> OvcEnHovW<'_, OtgIerSpec> {
        OvcEnHovW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn a_wait_con_en_hov(&mut self) -> AWaitConEnHovW<'_, OtgIerSpec> {
        AWaitConEnHovW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn aplgrmv_en(&mut self) -> AplgrmvEnW<'_, OtgIerSpec> {
        AplgrmvEnW::new(self, 12)
    }
}
#[doc = "OTG Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgIerSpec;
impl crate::RegisterSpec for OtgIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_ier::R`](R) reader structure"]
impl crate::Readable for OtgIerSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_ier::W`](W) writer structure"]
impl crate::Writable for OtgIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_IER to value 0"]
impl crate::Resettable for OtgIerSpec {}
