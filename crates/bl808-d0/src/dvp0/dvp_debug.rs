#[doc = "Register `DVP_DEBUG` reader"]
pub type R = crate::R<DvpDebugSpec>;
#[doc = "Register `DVP_DEBUG` writer"]
pub type W = crate::W<DvpDebugSpec>;
#[doc = "Field `reg_dvp_dbg_en` reader - Debug enable"]
pub type RegDvpDbgEnR = crate::BitReader;
#[doc = "Field `reg_dvp_dbg_en` writer - Debug enable"]
pub type RegDvpDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dvp_dbg_sel` reader - Debug signal selection"]
pub type RegDvpDbgSelR = crate::FieldReader;
#[doc = "Field `reg_dvp_dbg_sel` writer - Debug signal selection"]
pub type RegDvpDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_id_latch_line` reader - Frame ID latch line"]
pub type RegIdLatchLineR = crate::FieldReader;
#[doc = "Field `reg_id_latch_line` writer - Frame ID latch line"]
pub type RegIdLatchLineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Debug enable"]
    #[inline(always)]
    pub fn reg_dvp_dbg_en(&self) -> RegDvpDbgEnR {
        RegDvpDbgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Debug signal selection"]
    #[inline(always)]
    pub fn reg_dvp_dbg_sel(&self) -> RegDvpDbgSelR {
        RegDvpDbgSelR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Frame ID latch line"]
    #[inline(always)]
    pub fn reg_id_latch_line(&self) -> RegIdLatchLineR {
        RegIdLatchLineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug enable"]
    #[inline(always)]
    pub fn reg_dvp_dbg_en(&mut self) -> RegDvpDbgEnW<'_, DvpDebugSpec> {
        RegDvpDbgEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Debug signal selection"]
    #[inline(always)]
    pub fn reg_dvp_dbg_sel(&mut self) -> RegDvpDbgSelW<'_, DvpDebugSpec> {
        RegDvpDbgSelW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Frame ID latch line"]
    #[inline(always)]
    pub fn reg_id_latch_line(&mut self) -> RegIdLatchLineW<'_, DvpDebugSpec> {
        RegIdLatchLineW::new(self, 8)
    }
}
#[doc = "DVP Debug Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvpDebugSpec;
impl crate::RegisterSpec for DvpDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp_debug::R`](R) reader structure"]
impl crate::Readable for DvpDebugSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp_debug::W`](W) writer structure"]
impl crate::Writable for DvpDebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP_DEBUG to value 0x0500"]
impl crate::Resettable for DvpDebugSpec {
    const RESET_VALUE: u32 = 0x0500;
}
