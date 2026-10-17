#[doc = "Register `DBG_CFG3` reader"]
pub type R = crate::R<DbgCfg3Spec>;
#[doc = "Register `DBG_CFG3` writer"]
pub type W = crate::W<DbgCfg3Spec>;
#[doc = "Field `reg_dbg_hh_ctrl` reader - "]
pub type RegDbgHhCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_hh_ctrl` writer - "]
pub type RegDbgHhCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `reg_dbg_hh_sel` reader - "]
pub type RegDbgHhSelR = crate::FieldReader;
#[doc = "Field `reg_dbg_hh_sel` writer - "]
pub type RegDbgHhSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_hh_ctrl(&self) -> RegDbgHhCtrlR {
        RegDbgHhCtrlR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_hh_sel(&self) -> RegDbgHhSelR {
        RegDbgHhSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_hh_ctrl(&mut self) -> RegDbgHhCtrlW<'_, DbgCfg3Spec> {
        RegDbgHhCtrlW::new(self, 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_hh_sel(&mut self) -> RegDbgHhSelW<'_, DbgCfg3Spec> {
        RegDbgHhSelW::new(self, 30)
    }
}
#[doc = "Debug Config 3 (High-High)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCfg3Spec;
impl crate::RegisterSpec for DbgCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cfg3::R`](R) reader structure"]
impl crate::Readable for DbgCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_cfg3::W`](W) writer structure"]
impl crate::Writable for DbgCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CFG3 to value 0"]
impl crate::Resettable for DbgCfg3Spec {}
