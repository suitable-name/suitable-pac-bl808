#[doc = "Register `PHY_CFG_38` reader"]
pub type R = crate::R<PhyCfg38Spec>;
#[doc = "Register `PHY_CFG_38` writer"]
pub type W = crate::W<PhyCfg38Spec>;
#[doc = "Field `reg_timer_auto_refresh` reader - Auto-refresh timer"]
pub type RegTimerAutoRefreshR = crate::FieldReader;
#[doc = "Field `reg_timer_auto_refresh` writer - Auto-refresh timer"]
pub type RegTimerAutoRefreshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_reg_write` reader - Register write timer"]
pub type RegTimerRegWriteR = crate::FieldReader;
#[doc = "Field `reg_timer_reg_write` writer - Register write timer"]
pub type RegTimerRegWriteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_reg_read` reader - Register read timer"]
pub type RegTimerRegReadR = crate::FieldReader;
#[doc = "Field `reg_timer_reg_read` writer - Register read timer"]
pub type RegTimerRegReadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_dqs_stop` reader - DQS stop timer"]
pub type RegTimerDqsStopR = crate::FieldReader;
#[doc = "Field `reg_timer_dqs_stop` writer - DQS stop timer"]
pub type RegTimerDqsStopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Auto-refresh timer"]
    #[inline(always)]
    pub fn reg_timer_auto_refresh(&self) -> RegTimerAutoRefreshR {
        RegTimerAutoRefreshR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Register write timer"]
    #[inline(always)]
    pub fn reg_timer_reg_write(&self) -> RegTimerRegWriteR {
        RegTimerRegWriteR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Register read timer"]
    #[inline(always)]
    pub fn reg_timer_reg_read(&self) -> RegTimerRegReadR {
        RegTimerRegReadR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DQS stop timer"]
    #[inline(always)]
    pub fn reg_timer_dqs_stop(&self) -> RegTimerDqsStopR {
        RegTimerDqsStopR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Auto-refresh timer"]
    #[inline(always)]
    pub fn reg_timer_auto_refresh(&mut self) -> RegTimerAutoRefreshW<'_, PhyCfg38Spec> {
        RegTimerAutoRefreshW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Register write timer"]
    #[inline(always)]
    pub fn reg_timer_reg_write(&mut self) -> RegTimerRegWriteW<'_, PhyCfg38Spec> {
        RegTimerRegWriteW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Register read timer"]
    #[inline(always)]
    pub fn reg_timer_reg_read(&mut self) -> RegTimerRegReadW<'_, PhyCfg38Spec> {
        RegTimerRegReadW::new(self, 16)
    }
    #[doc = "Bits 24:31 - DQS stop timer"]
    #[inline(always)]
    pub fn reg_timer_dqs_stop(&mut self) -> RegTimerDqsStopW<'_, PhyCfg38Spec> {
        RegTimerDqsStopW::new(self, 24)
    }
}
#[doc = "PHY Timer Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg38Spec;
impl crate::RegisterSpec for PhyCfg38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_38::R`](R) reader structure"]
impl crate::Readable for PhyCfg38Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_38::W`](W) writer structure"]
impl crate::Writable for PhyCfg38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_38 to value 0x0208_0107"]
impl crate::Resettable for PhyCfg38Spec {
    const RESET_VALUE: u32 = 0x0208_0107;
}
