#[doc = "Register `UHS_AUTO_FRESH_2` reader"]
pub type R = crate::R<UhsAutoFresh2Spec>;
#[doc = "Register `UHS_AUTO_FRESH_2` writer"]
pub type W = crate::W<UhsAutoFresh2Spec>;
#[doc = "Field `reg_refi_cycle` reader - Refresh interval cycle count"]
pub type RegRefiCycleR = crate::FieldReader<u16>;
#[doc = "Field `reg_refi_cycle` writer - Refresh interval cycle count"]
pub type RegRefiCycleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_win_ref_cnt` reader - Window refresh counter limit"]
pub type RegWinRefCntR = crate::FieldReader<u16>;
#[doc = "Field `reg_win_ref_cnt` writer - Window refresh counter limit"]
pub type RegWinRefCntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:15 - Refresh interval cycle count"]
    #[inline(always)]
    pub fn reg_refi_cycle(&self) -> RegRefiCycleR {
        RegRefiCycleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:28 - Window refresh counter limit"]
    #[inline(always)]
    pub fn reg_win_ref_cnt(&self) -> RegWinRefCntR {
        RegWinRefCntR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Refresh interval cycle count"]
    #[inline(always)]
    pub fn reg_refi_cycle(&mut self) -> RegRefiCycleW<'_, UhsAutoFresh2Spec> {
        RegRefiCycleW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Window refresh counter limit"]
    #[inline(always)]
    pub fn reg_win_ref_cnt(&mut self) -> RegWinRefCntW<'_, UhsAutoFresh2Spec> {
        RegWinRefCntW::new(self, 16)
    }
}
#[doc = "UHS Auto Refresh Window Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsAutoFresh2Spec;
impl crate::RegisterSpec for UhsAutoFresh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_auto_fresh_2::R`](R) reader structure"]
impl crate::Readable for UhsAutoFresh2Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_auto_fresh_2::W`](W) writer structure"]
impl crate::Writable for UhsAutoFresh2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_AUTO_FRESH_2 to value 0x1000_0027"]
impl crate::Resettable for UhsAutoFresh2Spec {
    const RESET_VALUE: u32 = 0x1000_0027;
}
