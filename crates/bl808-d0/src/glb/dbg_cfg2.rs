#[doc = "Register `DBG_CFG2` reader"]
pub type R = crate::R<DbgCfg2Spec>;
#[doc = "Register `DBG_CFG2` writer"]
pub type W = crate::W<DbgCfg2Spec>;
#[doc = "Field `reg_dbg_hl_ctrl` reader - "]
pub type RegDbgHlCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_hl_ctrl` writer - "]
pub type RegDbgHlCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `reg_dbg_hl_sel` reader - "]
pub type RegDbgHlSelR = crate::FieldReader;
#[doc = "Field `reg_dbg_hl_sel` writer - "]
pub type RegDbgHlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_hl_ctrl(&self) -> RegDbgHlCtrlR {
        RegDbgHlCtrlR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_hl_sel(&self) -> RegDbgHlSelR {
        RegDbgHlSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_hl_ctrl(&mut self) -> RegDbgHlCtrlW<'_, DbgCfg2Spec> {
        RegDbgHlCtrlW::new(self, 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_hl_sel(&mut self) -> RegDbgHlSelW<'_, DbgCfg2Spec> {
        RegDbgHlSelW::new(self, 30)
    }
}
#[doc = "Debug Config 2 (High-Low)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCfg2Spec;
impl crate::RegisterSpec for DbgCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cfg2::R`](R) reader structure"]
impl crate::Readable for DbgCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_cfg2::W`](W) writer structure"]
impl crate::Writable for DbgCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CFG2 to value 0"]
impl crate::Resettable for DbgCfg2Spec {}
