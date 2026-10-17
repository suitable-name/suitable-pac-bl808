#[doc = "Register `PHY_CFG_34` reader"]
pub type R = crate::R<PhyCfg34Spec>;
#[doc = "Register `PHY_CFG_34` writer"]
pub type W = crate::W<PhyCfg34Spec>;
#[doc = "Field `reg_timer_dqs_start` reader - DQS start timer"]
pub type RegTimerDqsStartR = crate::FieldReader;
#[doc = "Field `reg_timer_dqs_start` writer - DQS start timer"]
pub type RegTimerDqsStartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_dqs_array_stop` reader - DQS array stop timer"]
pub type RegTimerDqsArrayStopR = crate::FieldReader;
#[doc = "Field `reg_timer_dqs_array_stop` writer - DQS array stop timer"]
pub type RegTimerDqsArrayStopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_array_write` reader - Array write timer"]
pub type RegTimerArrayWriteR = crate::FieldReader;
#[doc = "Field `reg_timer_array_write` writer - Array write timer"]
pub type RegTimerArrayWriteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_array_read` reader - Array read timer"]
pub type RegTimerArrayReadR = crate::FieldReader;
#[doc = "Field `reg_timer_array_read` writer - Array read timer"]
pub type RegTimerArrayReadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DQS start timer"]
    #[inline(always)]
    pub fn reg_timer_dqs_start(&self) -> RegTimerDqsStartR {
        RegTimerDqsStartR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DQS array stop timer"]
    #[inline(always)]
    pub fn reg_timer_dqs_array_stop(&self) -> RegTimerDqsArrayStopR {
        RegTimerDqsArrayStopR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Array write timer"]
    #[inline(always)]
    pub fn reg_timer_array_write(&self) -> RegTimerArrayWriteR {
        RegTimerArrayWriteR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Array read timer"]
    #[inline(always)]
    pub fn reg_timer_array_read(&self) -> RegTimerArrayReadR {
        RegTimerArrayReadR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQS start timer"]
    #[inline(always)]
    pub fn reg_timer_dqs_start(&mut self) -> RegTimerDqsStartW<'_, PhyCfg34Spec> {
        RegTimerDqsStartW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DQS array stop timer"]
    #[inline(always)]
    pub fn reg_timer_dqs_array_stop(&mut self) -> RegTimerDqsArrayStopW<'_, PhyCfg34Spec> {
        RegTimerDqsArrayStopW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Array write timer"]
    #[inline(always)]
    pub fn reg_timer_array_write(&mut self) -> RegTimerArrayWriteW<'_, PhyCfg34Spec> {
        RegTimerArrayWriteW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Array read timer"]
    #[inline(always)]
    pub fn reg_timer_array_read(&mut self) -> RegTimerArrayReadW<'_, PhyCfg34Spec> {
        RegTimerArrayReadW::new(self, 24)
    }
}
#[doc = "PHY Timer Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg34Spec;
impl crate::RegisterSpec for PhyCfg34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_34::R`](R) reader structure"]
impl crate::Readable for PhyCfg34Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_34::W`](W) writer structure"]
impl crate::Writable for PhyCfg34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_34 to value 0x0500_0101"]
impl crate::Resettable for PhyCfg34Spec {
    const RESET_VALUE: u32 = 0x0500_0101;
}
