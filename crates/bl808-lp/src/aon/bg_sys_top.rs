#[doc = "Register `BG_SYS_TOP` reader"]
pub type R = crate::R<BgSysTopSpec>;
#[doc = "Register `BG_SYS_TOP` writer"]
pub type W = crate::W<BgSysTopSpec>;
#[doc = "Field `pu_bg_sys_aon` reader - "]
pub type PuBgSysAonR = crate::BitReader;
#[doc = "Field `pu_bg_sys_aon` writer - "]
pub type PuBgSysAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `istart_ctrl_aon` reader - "]
pub type IstartCtrlAonR = crate::BitReader;
#[doc = "Field `istart_ctrl_aon` writer - "]
pub type IstartCtrlAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PuBgSysAonR {
        PuBgSysAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn istart_ctrl_aon(&self) -> IstartCtrlAonR {
        IstartCtrlAonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&mut self) -> PuBgSysAonW<'_, BgSysTopSpec> {
        PuBgSysAonW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn istart_ctrl_aon(&mut self) -> IstartCtrlAonW<'_, BgSysTopSpec> {
        IstartCtrlAonW::new(self, 1)
    }
}
#[doc = "Bandgap System Top Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bg_sys_top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_sys_top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgSysTopSpec;
impl crate::RegisterSpec for BgSysTopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bg_sys_top::R`](R) reader structure"]
impl crate::Readable for BgSysTopSpec {}
#[doc = "`write(|w| ..)` method takes [`bg_sys_top::W`](W) writer structure"]
impl crate::Writable for BgSysTopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BG_SYS_TOP to value 0"]
impl crate::Resettable for BgSysTopSpec {}
