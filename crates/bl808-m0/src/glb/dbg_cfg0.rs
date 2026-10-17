#[doc = "Register `DBG_CFG0` reader"]
pub type R = crate::R<DbgCfg0Spec>;
#[doc = "Register `DBG_CFG0` writer"]
pub type W = crate::W<DbgCfg0Spec>;
#[doc = "Field `reg_dbg_ll_ctrl` reader - "]
pub type RegDbgLlCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_ll_ctrl` writer - "]
pub type RegDbgLlCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `reg_dbg_ll_sel` reader - "]
pub type RegDbgLlSelR = crate::FieldReader;
#[doc = "Field `reg_dbg_ll_sel` writer - "]
pub type RegDbgLlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_ll_ctrl(&self) -> RegDbgLlCtrlR {
        RegDbgLlCtrlR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_ll_sel(&self) -> RegDbgLlSelR {
        RegDbgLlSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_ll_ctrl(&mut self) -> RegDbgLlCtrlW<'_, DbgCfg0Spec> {
        RegDbgLlCtrlW::new(self, 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_ll_sel(&mut self) -> RegDbgLlSelW<'_, DbgCfg0Spec> {
        RegDbgLlSelW::new(self, 30)
    }
}
#[doc = "Debug Config 0 (Low-Low)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCfg0Spec;
impl crate::RegisterSpec for DbgCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cfg0::R`](R) reader structure"]
impl crate::Readable for DbgCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_cfg0::W`](W) writer structure"]
impl crate::Writable for DbgCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CFG0 to value 0"]
impl crate::Resettable for DbgCfg0Spec {}
