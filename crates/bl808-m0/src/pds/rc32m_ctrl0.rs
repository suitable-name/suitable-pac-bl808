#[doc = "Register `RC32M_CTRL0` reader"]
pub type R = crate::R<Rc32mCtrl0Spec>;
#[doc = "Register `RC32M_CTRL0` writer"]
pub type W = crate::W<Rc32mCtrl0Spec>;
#[doc = "Field `rc32m_cal_done` reader - "]
pub type Rc32mCalDoneR = crate::BitReader;
#[doc = "Field `rc32m_rdy` reader - "]
pub type Rc32mRdyR = crate::BitReader;
#[doc = "Field `rc32m_cal_inprogress` reader - "]
pub type Rc32mCalInprogressR = crate::BitReader;
#[doc = "Field `rc32m_cal_div` reader - "]
pub type Rc32mCalDivR = crate::FieldReader;
#[doc = "Field `rc32m_cal_div` writer - "]
pub type Rc32mCalDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rc32m_cal_precharge` reader - "]
pub type Rc32mCalPrechargeR = crate::BitReader;
#[doc = "Field `rc32m_dig_code_fr_cal` reader - "]
pub type Rc32mDigCodeFrCalR = crate::FieldReader;
#[doc = "Field `rc32m_allow_cal` reader - "]
pub type Rc32mAllowCalR = crate::BitReader;
#[doc = "Field `rc32m_allow_cal` writer - "]
pub type Rc32mAllowCalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_refclk_half` reader - "]
pub type Rc32mRefclkHalfR = crate::BitReader;
#[doc = "Field `rc32m_refclk_half` writer - "]
pub type Rc32mRefclkHalfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_ext_code_en` reader - "]
pub type Rc32mExtCodeEnR = crate::BitReader;
#[doc = "Field `rc32m_ext_code_en` writer - "]
pub type Rc32mExtCodeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_cal_en` reader - "]
pub type Rc32mCalEnR = crate::BitReader;
#[doc = "Field `rc32m_cal_en` writer - "]
pub type Rc32mCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_pd` reader - "]
pub type Rc32mPdR = crate::BitReader;
#[doc = "Field `rc32m_pd` writer - "]
pub type Rc32mPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_code_fr_ext` reader - "]
pub type Rc32mCodeFrExtR = crate::FieldReader;
#[doc = "Field `rc32m_code_fr_ext` writer - "]
pub type Rc32mCodeFrExtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_cal_done(&self) -> Rc32mCalDoneR {
        Rc32mCalDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_rdy(&self) -> Rc32mRdyR {
        Rc32mRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_cal_inprogress(&self) -> Rc32mCalInprogressR {
        Rc32mCalInprogressR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&self) -> Rc32mCalDivR {
        Rc32mCalDivR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32m_cal_precharge(&self) -> Rc32mCalPrechargeR {
        Rc32mCalPrechargeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn rc32m_dig_code_fr_cal(&self) -> Rc32mDigCodeFrCalR {
        Rc32mDigCodeFrCalR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&self) -> Rc32mAllowCalR {
        Rc32mAllowCalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&self) -> Rc32mRefclkHalfR {
        Rc32mRefclkHalfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&self) -> Rc32mExtCodeEnR {
        Rc32mExtCodeEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&self) -> Rc32mCalEnR {
        Rc32mCalEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&self) -> Rc32mPdR {
        Rc32mPdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&self) -> Rc32mCodeFrExtR {
        Rc32mCodeFrExtR::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&mut self) -> Rc32mCalDivW<'_, Rc32mCtrl0Spec> {
        Rc32mCalDivW::new(self, 3)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&mut self) -> Rc32mAllowCalW<'_, Rc32mCtrl0Spec> {
        Rc32mAllowCalW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&mut self) -> Rc32mRefclkHalfW<'_, Rc32mCtrl0Spec> {
        Rc32mRefclkHalfW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&mut self) -> Rc32mExtCodeEnW<'_, Rc32mCtrl0Spec> {
        Rc32mExtCodeEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&mut self) -> Rc32mCalEnW<'_, Rc32mCtrl0Spec> {
        Rc32mCalEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&mut self) -> Rc32mPdW<'_, Rc32mCtrl0Spec> {
        Rc32mPdW::new(self, 21)
    }
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&mut self) -> Rc32mCodeFrExtW<'_, Rc32mCtrl0Spec> {
        Rc32mCodeFrExtW::new(self, 22)
    }
}
#[doc = "RC 32MHz oscillator control and calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32m_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32m_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc32mCtrl0Spec;
impl crate::RegisterSpec for Rc32mCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32m_ctrl0::R`](R) reader structure"]
impl crate::Readable for Rc32mCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rc32m_ctrl0::W`](W) writer structure"]
impl crate::Writable for Rc32mCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC32M_CTRL0 to value 0x1808_0018"]
impl crate::Resettable for Rc32mCtrl0Spec {
    const RESET_VALUE: u32 = 0x1808_0018;
}
