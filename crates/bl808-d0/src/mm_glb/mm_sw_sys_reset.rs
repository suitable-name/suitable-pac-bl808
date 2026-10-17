#[doc = "Register `MM_SW_SYS_RESET` reader"]
pub type R = crate::R<MmSwSysResetSpec>;
#[doc = "Register `MM_SW_SYS_RESET` writer"]
pub type W = crate::W<MmSwSysResetSpec>;
#[doc = "Field `reg_ctrl_sys_reset` reader - Multimedia system software reset"]
pub type RegCtrlSysResetR = crate::BitReader;
#[doc = "Field `reg_ctrl_sys_reset` writer - Multimedia system software reset"]
pub type RegCtrlSysResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_pwron_rst` reader - Multimedia power-on reset"]
pub type RegCtrlPwronRstR = crate::BitReader;
#[doc = "Field `reg_ctrl_pwron_rst` writer - Multimedia power-on reset"]
pub type RegCtrlPwronRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_mmcpu0_reset` reader - Multimedia CPU0 software reset"]
pub type RegCtrlMmcpu0ResetR = crate::BitReader;
#[doc = "Field `reg_ctrl_mmcpu0_reset` writer - Multimedia CPU0 software reset"]
pub type RegCtrlMmcpu0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Multimedia system software reset"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> RegCtrlSysResetR {
        RegCtrlSysResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Multimedia power-on reset"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> RegCtrlPwronRstR {
        RegCtrlPwronRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Multimedia CPU0 software reset"]
    #[inline(always)]
    pub fn reg_ctrl_mmcpu0_reset(&self) -> RegCtrlMmcpu0ResetR {
        RegCtrlMmcpu0ResetR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multimedia system software reset"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&mut self) -> RegCtrlSysResetW<'_, MmSwSysResetSpec> {
        RegCtrlSysResetW::new(self, 0)
    }
    #[doc = "Bit 2 - Multimedia power-on reset"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&mut self) -> RegCtrlPwronRstW<'_, MmSwSysResetSpec> {
        RegCtrlPwronRstW::new(self, 2)
    }
    #[doc = "Bit 8 - Multimedia CPU0 software reset"]
    #[inline(always)]
    pub fn reg_ctrl_mmcpu0_reset(&mut self) -> RegCtrlMmcpu0ResetW<'_, MmSwSysResetSpec> {
        RegCtrlMmcpu0ResetW::new(self, 8)
    }
}
#[doc = "Multimedia System Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_sw_sys_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_sw_sys_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmSwSysResetSpec;
impl crate::RegisterSpec for MmSwSysResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_sw_sys_reset::R`](R) reader structure"]
impl crate::Readable for MmSwSysResetSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_sw_sys_reset::W`](W) writer structure"]
impl crate::Writable for MmSwSysResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_SW_SYS_RESET to value 0x0100"]
impl crate::Resettable for MmSwSysResetSpec {
    const RESET_VALUE: u32 = 0x0100;
}
