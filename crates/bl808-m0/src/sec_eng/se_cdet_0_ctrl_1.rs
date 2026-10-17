#[doc = "Register `SE_CDET_0_CTRL_1` reader"]
pub type R = crate::R<SeCdet0Ctrl1Spec>;
#[doc = "Register `SE_CDET_0_CTRL_1` writer"]
pub type W = crate::W<SeCdet0Ctrl1Spec>;
#[doc = "Field `se_cdet_0_t_loop_n` reader - "]
pub type SeCdet0TLoopNR = crate::FieldReader;
#[doc = "Field `se_cdet_0_t_loop_n` writer - "]
pub type SeCdet0TLoopNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `se_cdet_0_t_dly_n` reader - "]
pub type SeCdet0TDlyNR = crate::FieldReader;
#[doc = "Field `se_cdet_0_t_dly_n` writer - "]
pub type SeCdet0TDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `se_cdet_0_g_slp_n` reader - "]
pub type SeCdet0GSlpNR = crate::FieldReader;
#[doc = "Field `se_cdet_0_g_slp_n` writer - "]
pub type SeCdet0GSlpNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_cdet_0_t_loop_n(&self) -> SeCdet0TLoopNR {
        SeCdet0TLoopNR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_cdet_0_t_dly_n(&self) -> SeCdet0TDlyNR {
        SeCdet0TDlyNR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_slp_n(&self) -> SeCdet0GSlpNR {
        SeCdet0GSlpNR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_cdet_0_t_loop_n(&mut self) -> SeCdet0TLoopNW<'_, SeCdet0Ctrl1Spec> {
        SeCdet0TLoopNW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_cdet_0_t_dly_n(&mut self) -> SeCdet0TDlyNW<'_, SeCdet0Ctrl1Spec> {
        SeCdet0TDlyNW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_slp_n(&mut self) -> SeCdet0GSlpNW<'_, SeCdet0Ctrl1Spec> {
        SeCdet0GSlpNW::new(self, 16)
    }
}
#[doc = "Clock Detection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCdet0Ctrl1Spec;
impl crate::RegisterSpec for SeCdet0Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_cdet_0_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SeCdet0Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`se_cdet_0_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SeCdet0Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_CDET_0_CTRL_1 to value 0x00ff_0332"]
impl crate::Resettable for SeCdet0Ctrl1Spec {
    const RESET_VALUE: u32 = 0x00ff_0332;
}
