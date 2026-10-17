#[doc = "Register `DBG_CFG1` reader"]
pub type R = crate::R<DbgCfg1Spec>;
#[doc = "Register `DBG_CFG1` writer"]
pub type W = crate::W<DbgCfg1Spec>;
#[doc = "Field `reg_dbg_lh_ctrl` reader - "]
pub type RegDbgLhCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_lh_ctrl` writer - "]
pub type RegDbgLhCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `reg_dbg_lh_sel` reader - "]
pub type RegDbgLhSelR = crate::FieldReader;
#[doc = "Field `reg_dbg_lh_sel` writer - "]
pub type RegDbgLhSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&self) -> RegDbgLhCtrlR {
        RegDbgLhCtrlR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_sel(&self) -> RegDbgLhSelR {
        RegDbgLhSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&mut self) -> RegDbgLhCtrlW<'_, DbgCfg1Spec> {
        RegDbgLhCtrlW::new(self, 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_sel(&mut self) -> RegDbgLhSelW<'_, DbgCfg1Spec> {
        RegDbgLhSelW::new(self, 30)
    }
}
#[doc = "Debug Config 1 (Low-High)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCfg1Spec;
impl crate::RegisterSpec for DbgCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cfg1::R`](R) reader structure"]
impl crate::Readable for DbgCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_cfg1::W`](W) writer structure"]
impl crate::Writable for DbgCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CFG1 to value 0"]
impl crate::Resettable for DbgCfg1Spec {}
