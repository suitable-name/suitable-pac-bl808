#[doc = "Register `SE_CTRL_RESERVED_0` reader"]
pub type R = crate::R<SeCtrlReserved0Spec>;
#[doc = "Register `SE_CTRL_RESERVED_0` writer"]
pub type W = crate::W<SeCtrlReserved0Spec>;
#[doc = "Field `se_ctrl_reserved_0` reader - Internal configuration word 0."]
pub type SeCtrlReserved0R = crate::FieldReader<u32>;
#[doc = "Field `se_ctrl_reserved_0` writer - Internal configuration word 0."]
pub type SeCtrlReserved0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Internal configuration word 0."]
    #[inline(always)]
    pub fn se_ctrl_reserved_0(&self) -> SeCtrlReserved0R {
        SeCtrlReserved0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Internal configuration word 0."]
    #[inline(always)]
    pub fn se_ctrl_reserved_0(&mut self) -> SeCtrlReserved0W<'_, SeCtrlReserved0Spec> {
        SeCtrlReserved0W::new(self, 0)
    }
}
#[doc = "Security engine internal bus configuration. Reserved for hardware-specific arbitration and timing tuning.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCtrlReserved0Spec;
impl crate::RegisterSpec for SeCtrlReserved0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_ctrl_reserved_0::R`](R) reader structure"]
impl crate::Readable for SeCtrlReserved0Spec {}
#[doc = "`write(|w| ..)` method takes [`se_ctrl_reserved_0::W`](W) writer structure"]
impl crate::Writable for SeCtrlReserved0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_CTRL_RESERVED_0 to value 0"]
impl crate::Resettable for SeCtrlReserved0Spec {}
