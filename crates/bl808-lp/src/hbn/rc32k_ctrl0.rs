#[doc = "Register `RC32K_CTRL0` reader"]
pub type R = crate::R<Rc32kCtrl0Spec>;
#[doc = "Register `RC32K_CTRL0` writer"]
pub type W = crate::W<Rc32kCtrl0Spec>;
#[doc = "Field `rc32k_cal_done` reader - "]
pub type Rc32kCalDoneR = crate::BitReader;
#[doc = "Field `rc32k_rdy` reader - "]
pub type Rc32kRdyR = crate::BitReader;
#[doc = "Field `rc32k_cal_inprogress` reader - "]
pub type Rc32kCalInprogressR = crate::BitReader;
#[doc = "Field `rc32k_cal_div` reader - "]
pub type Rc32kCalDivR = crate::FieldReader;
#[doc = "Field `rc32k_cal_div` writer - "]
pub type Rc32kCalDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rc32k_cal_precharge` reader - "]
pub type Rc32kCalPrechargeR = crate::BitReader;
#[doc = "Field `rc32k_dig_code_fr_cal` reader - "]
pub type Rc32kDigCodeFrCalR = crate::FieldReader<u16>;
#[doc = "Field `rc32k_vref_dly` reader - "]
pub type Rc32kVrefDlyR = crate::FieldReader;
#[doc = "Field `rc32k_vref_dly` writer - "]
pub type Rc32kVrefDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rc32k_allow_cal` reader - "]
pub type Rc32kAllowCalR = crate::BitReader;
#[doc = "Field `rc32k_allow_cal` writer - "]
pub type Rc32kAllowCalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_ext_code_en` reader - "]
pub type Rc32kExtCodeEnR = crate::BitReader;
#[doc = "Field `rc32k_ext_code_en` writer - "]
pub type Rc32kExtCodeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_cal_en` reader - "]
pub type Rc32kCalEnR = crate::BitReader;
#[doc = "Field `rc32k_cal_en` writer - "]
pub type Rc32kCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rc32k` reader - "]
pub type PuRc32kR = crate::BitReader;
#[doc = "Field `pu_rc32k` writer - "]
pub type PuRc32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32k_code_fr_ext` reader - "]
pub type Rc32kCodeFrExtR = crate::FieldReader<u16>;
#[doc = "Field `rc32k_code_fr_ext` writer - "]
pub type Rc32kCodeFrExtW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32k_cal_done(&self) -> Rc32kCalDoneR {
        Rc32kCalDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32k_rdy(&self) -> Rc32kRdyR {
        Rc32kRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32k_cal_inprogress(&self) -> Rc32kCalInprogressR {
        Rc32kCalInprogressR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&self) -> Rc32kCalDivR {
        Rc32kCalDivR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_cal_precharge(&self) -> Rc32kCalPrechargeR {
        Rc32kCalPrechargeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rc32k_dig_code_fr_cal(&self) -> Rc32kDigCodeFrCalR {
        Rc32kDigCodeFrCalR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&self) -> Rc32kVrefDlyR {
        Rc32kVrefDlyR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&self) -> Rc32kAllowCalR {
        Rc32kAllowCalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&self) -> Rc32kExtCodeEnR {
        Rc32kExtCodeEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&self) -> Rc32kCalEnR {
        Rc32kCalEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_rc32k(&self) -> PuRc32kR {
        PuRc32kR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&self) -> Rc32kCodeFrExtR {
        Rc32kCodeFrExtR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&mut self) -> Rc32kCalDivW<'_, Rc32kCtrl0Spec> {
        Rc32kCalDivW::new(self, 3)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&mut self) -> Rc32kVrefDlyW<'_, Rc32kCtrl0Spec> {
        Rc32kVrefDlyW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&mut self) -> Rc32kAllowCalW<'_, Rc32kCtrl0Spec> {
        Rc32kAllowCalW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&mut self) -> Rc32kExtCodeEnW<'_, Rc32kCtrl0Spec> {
        Rc32kExtCodeEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&mut self) -> Rc32kCalEnW<'_, Rc32kCtrl0Spec> {
        Rc32kCalEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_rc32k(&mut self) -> PuRc32kW<'_, Rc32kCtrl0Spec> {
        PuRc32kW::new(self, 21)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&mut self) -> Rc32kCodeFrExtW<'_, Rc32kCtrl0Spec> {
        Rc32kCodeFrExtW::new(self, 22)
    }
}
#[doc = "RC32K Internal Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32k_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32k_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc32kCtrl0Spec;
impl crate::RegisterSpec for Rc32kCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32k_ctrl0::R`](R) reader structure"]
impl crate::Readable for Rc32kCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rc32k_ctrl0::W`](W) writer structure"]
impl crate::Writable for Rc32kCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC32K_CTRL0 to value 0x4b28_801b"]
impl crate::Resettable for Rc32kCtrl0Spec {
    const RESET_VALUE: u32 = 0x4b28_801b;
}
