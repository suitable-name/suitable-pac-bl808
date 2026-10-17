#[doc = "Register `PSRAM_DBG_SEL` reader"]
pub type R = crate::R<PsramDbgSelSpec>;
#[doc = "Register `PSRAM_DBG_SEL` writer"]
pub type W = crate::W<PsramDbgSelSpec>;
#[doc = "Field `reg_psram_dbg_en` reader - PSRAM debug enable"]
pub type RegPsramDbgEnR = crate::BitReader;
#[doc = "Field `reg_psram_dbg_en` writer - PSRAM debug enable"]
pub type RegPsramDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psram_dbg_sel` reader - PSRAM debug signal selection"]
pub type RegPsramDbgSelR = crate::FieldReader;
#[doc = "Field `reg_psram_dbg_sel` writer - PSRAM debug signal selection"]
pub type RegPsramDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - PSRAM debug enable"]
    #[inline(always)]
    pub fn reg_psram_dbg_en(&self) -> RegPsramDbgEnR {
        RegPsramDbgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - PSRAM debug signal selection"]
    #[inline(always)]
    pub fn reg_psram_dbg_sel(&self) -> RegPsramDbgSelR {
        RegPsramDbgSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PSRAM debug enable"]
    #[inline(always)]
    pub fn reg_psram_dbg_en(&mut self) -> RegPsramDbgEnW<'_, PsramDbgSelSpec> {
        RegPsramDbgEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - PSRAM debug signal selection"]
    #[inline(always)]
    pub fn reg_psram_dbg_sel(&mut self) -> RegPsramDbgSelW<'_, PsramDbgSelSpec> {
        RegPsramDbgSelW::new(self, 4)
    }
}
#[doc = "PSRAM Debug Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dbg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dbg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramDbgSelSpec;
impl crate::RegisterSpec for PsramDbgSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_dbg_sel::R`](R) reader structure"]
impl crate::Readable for PsramDbgSelSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_dbg_sel::W`](W) writer structure"]
impl crate::Writable for PsramDbgSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_DBG_SEL to value 0"]
impl crate::Resettable for PsramDbgSelSpec {}
