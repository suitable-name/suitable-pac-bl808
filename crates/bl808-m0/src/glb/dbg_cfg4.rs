#[doc = "Register `DBG_CFG4` reader"]
pub type R = crate::R<DbgCfg4Spec>;
#[doc = "Register `DBG_CFG4` writer"]
pub type W = crate::W<DbgCfg4Spec>;
#[doc = "Field `debug_oe` reader - "]
pub type DebugOeR = crate::BitReader;
#[doc = "Field `debug_oe` writer - "]
pub type DebugOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debug_i` reader - "]
pub type DebugIR = crate::FieldReader<u32>;
#[doc = "Field `debug_i` writer - "]
pub type DebugIW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DebugOeR {
        DebugOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DebugIR {
        DebugIR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&mut self) -> DebugOeW<'_, DbgCfg4Spec> {
        DebugOeW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&mut self) -> DebugIW<'_, DbgCfg4Spec> {
        DebugIW::new(self, 1)
    }
}
#[doc = "Debug Config 4 (Output Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCfg4Spec;
impl crate::RegisterSpec for DbgCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cfg4::R`](R) reader structure"]
impl crate::Readable for DbgCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_cfg4::W`](W) writer structure"]
impl crate::Writable for DbgCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CFG4 to value 0"]
impl crate::Resettable for DbgCfg4Spec {}
