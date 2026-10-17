#[doc = "Register `PHY_CFG_44` reader"]
pub type R = crate::R<PhyCfg44Spec>;
#[doc = "Register `PHY_CFG_44` writer"]
pub type W = crate::W<PhyCfg44Spec>;
#[doc = "Field `reg_timer_array_read_busy` reader - Array read busy status"]
pub type RegTimerArrayReadBusyR = crate::FieldReader;
#[doc = "Field `reg_timer_array_read_busy` writer - Array read busy status"]
pub type RegTimerArrayReadBusyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_array_write_busy` reader - Array write busy status"]
pub type RegTimerArrayWriteBusyR = crate::FieldReader;
#[doc = "Field `reg_timer_array_write_busy` writer - Array write busy status"]
pub type RegTimerArrayWriteBusyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_reg_read_busy` reader - Register read busy status"]
pub type RegTimerRegReadBusyR = crate::FieldReader;
#[doc = "Field `reg_timer_reg_read_busy` writer - Register read busy status"]
pub type RegTimerRegReadBusyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_reg_write_busy` reader - Register write busy status"]
pub type RegTimerRegWriteBusyR = crate::FieldReader;
#[doc = "Field `reg_timer_reg_write_busy` writer - Register write busy status"]
pub type RegTimerRegWriteBusyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Array read busy status"]
    #[inline(always)]
    pub fn reg_timer_array_read_busy(&self) -> RegTimerArrayReadBusyR {
        RegTimerArrayReadBusyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Array write busy status"]
    #[inline(always)]
    pub fn reg_timer_array_write_busy(&self) -> RegTimerArrayWriteBusyR {
        RegTimerArrayWriteBusyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Register read busy status"]
    #[inline(always)]
    pub fn reg_timer_reg_read_busy(&self) -> RegTimerRegReadBusyR {
        RegTimerRegReadBusyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Register write busy status"]
    #[inline(always)]
    pub fn reg_timer_reg_write_busy(&self) -> RegTimerRegWriteBusyR {
        RegTimerRegWriteBusyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Array read busy status"]
    #[inline(always)]
    pub fn reg_timer_array_read_busy(&mut self) -> RegTimerArrayReadBusyW<'_, PhyCfg44Spec> {
        RegTimerArrayReadBusyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Array write busy status"]
    #[inline(always)]
    pub fn reg_timer_array_write_busy(&mut self) -> RegTimerArrayWriteBusyW<'_, PhyCfg44Spec> {
        RegTimerArrayWriteBusyW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Register read busy status"]
    #[inline(always)]
    pub fn reg_timer_reg_read_busy(&mut self) -> RegTimerRegReadBusyW<'_, PhyCfg44Spec> {
        RegTimerRegReadBusyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Register write busy status"]
    #[inline(always)]
    pub fn reg_timer_reg_write_busy(&mut self) -> RegTimerRegWriteBusyW<'_, PhyCfg44Spec> {
        RegTimerRegWriteBusyW::new(self, 24)
    }
}
#[doc = "PHY Timer Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg44Spec;
impl crate::RegisterSpec for PhyCfg44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_44::R`](R) reader structure"]
impl crate::Readable for PhyCfg44Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_44::W`](W) writer structure"]
impl crate::Writable for PhyCfg44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_44 to value 0x040b_0308"]
impl crate::Resettable for PhyCfg44Spec {
    const RESET_VALUE: u32 = 0x040b_0308;
}
