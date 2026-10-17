#[doc = "Register `SE_CTRL_RESERVED_1` reader"]
pub type R = crate::R<SeCtrlReserved1Spec>;
#[doc = "Register `SE_CTRL_RESERVED_1` writer"]
pub type W = crate::W<SeCtrlReserved1Spec>;
#[doc = "Field `se_ctrl_reserved_1` reader - Internal configuration word 1."]
pub type SeCtrlReserved1R = crate::FieldReader<u32>;
#[doc = "Field `se_ctrl_reserved_1` writer - Internal configuration word 1."]
pub type SeCtrlReserved1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Internal configuration word 1."]
    #[inline(always)]
    pub fn se_ctrl_reserved_1(&self) -> SeCtrlReserved1R {
        SeCtrlReserved1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Internal configuration word 1."]
    #[inline(always)]
    pub fn se_ctrl_reserved_1(&mut self) -> SeCtrlReserved1W<'_, SeCtrlReserved1Spec> {
        SeCtrlReserved1W::new(self, 0)
    }
}
#[doc = "Security engine internal status and control. Reserved for factory-specific hardware configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCtrlReserved1Spec;
impl crate::RegisterSpec for SeCtrlReserved1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_ctrl_reserved_1::R`](R) reader structure"]
impl crate::Readable for SeCtrlReserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`se_ctrl_reserved_1::W`](W) writer structure"]
impl crate::Writable for SeCtrlReserved1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_CTRL_RESERVED_1 to value 0xffff_ffff"]
impl crate::Resettable for SeCtrlReserved1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
