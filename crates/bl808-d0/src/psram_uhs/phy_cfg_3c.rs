#[doc = "Register `PHY_CFG_3C` reader"]
pub type R = crate::R<PhyCfg3cSpec>;
#[doc = "Register `PHY_CFG_3C` writer"]
pub type W = crate::W<PhyCfg3cSpec>;
#[doc = "Field `reg_timer_self_refresh1_in` reader - Self-refresh entry timer"]
pub type RegTimerSelfRefresh1InR = crate::FieldReader;
#[doc = "Field `reg_timer_self_refresh1_in` writer - Self-refresh entry timer"]
pub type RegTimerSelfRefresh1InW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_self_refresh1_exit` reader - Self-refresh exit timer"]
pub type RegTimerSelfRefresh1ExitR = crate::FieldReader;
#[doc = "Field `reg_timer_self_refresh1_exit` writer - Self-refresh exit timer"]
pub type RegTimerSelfRefresh1ExitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_timer_global_rst` reader - Global reset timer"]
pub type RegTimerGlobalRstR = crate::FieldReader<u16>;
#[doc = "Field `reg_timer_global_rst` writer - Global reset timer"]
pub type RegTimerGlobalRstW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:7 - Self-refresh entry timer"]
    #[inline(always)]
    pub fn reg_timer_self_refresh1_in(&self) -> RegTimerSelfRefresh1InR {
        RegTimerSelfRefresh1InR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Self-refresh exit timer"]
    #[inline(always)]
    pub fn reg_timer_self_refresh1_exit(&self) -> RegTimerSelfRefresh1ExitR {
        RegTimerSelfRefresh1ExitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:29 - Global reset timer"]
    #[inline(always)]
    pub fn reg_timer_global_rst(&self) -> RegTimerGlobalRstR {
        RegTimerGlobalRstR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self-refresh entry timer"]
    #[inline(always)]
    pub fn reg_timer_self_refresh1_in(&mut self) -> RegTimerSelfRefresh1InW<'_, PhyCfg3cSpec> {
        RegTimerSelfRefresh1InW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Self-refresh exit timer"]
    #[inline(always)]
    pub fn reg_timer_self_refresh1_exit(&mut self) -> RegTimerSelfRefresh1ExitW<'_, PhyCfg3cSpec> {
        RegTimerSelfRefresh1ExitW::new(self, 8)
    }
    #[doc = "Bits 16:29 - Global reset timer"]
    #[inline(always)]
    pub fn reg_timer_global_rst(&mut self) -> RegTimerGlobalRstW<'_, PhyCfg3cSpec> {
        RegTimerGlobalRstW::new(self, 16)
    }
}
#[doc = "PHY Timer Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg3cSpec;
impl crate::RegisterSpec for PhyCfg3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_3c::R`](R) reader structure"]
impl crate::Readable for PhyCfg3cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_3c::W`](W) writer structure"]
impl crate::Writable for PhyCfg3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_3C to value 0x0272_0808"]
impl crate::Resettable for PhyCfg3cSpec {
    const RESET_VALUE: u32 = 0x0272_0808;
}
