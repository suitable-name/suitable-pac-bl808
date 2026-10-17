#[doc = "Register `UHS_DBG_SEL` reader"]
pub type R = crate::R<UhsDbgSelSpec>;
#[doc = "Register `UHS_DBG_SEL` writer"]
pub type W = crate::W<UhsDbgSelSpec>;
#[doc = "Field `reg_psram_dbg_en` reader - Enable UHS debug output"]
pub type RegPsramDbgEnR = crate::BitReader;
#[doc = "Field `reg_psram_dbg_en` writer - Enable UHS debug output"]
pub type RegPsramDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psram_dbg_sel` reader - Select debug signal source"]
pub type RegPsramDbgSelR = crate::FieldReader;
#[doc = "Field `reg_psram_dbg_sel` writer - Select debug signal source"]
pub type RegPsramDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable UHS debug output"]
    #[inline(always)]
    pub fn reg_psram_dbg_en(&self) -> RegPsramDbgEnR {
        RegPsramDbgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Select debug signal source"]
    #[inline(always)]
    pub fn reg_psram_dbg_sel(&self) -> RegPsramDbgSelR {
        RegPsramDbgSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable UHS debug output"]
    #[inline(always)]
    pub fn reg_psram_dbg_en(&mut self) -> RegPsramDbgEnW<'_, UhsDbgSelSpec> {
        RegPsramDbgEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select debug signal source"]
    #[inline(always)]
    pub fn reg_psram_dbg_sel(&mut self) -> RegPsramDbgSelW<'_, UhsDbgSelSpec> {
        RegPsramDbgSelW::new(self, 4)
    }
}
#[doc = "UHS Debug Source Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_dbg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_dbg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsDbgSelSpec;
impl crate::RegisterSpec for UhsDbgSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_dbg_sel::R`](R) reader structure"]
impl crate::Readable for UhsDbgSelSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_dbg_sel::W`](W) writer structure"]
impl crate::Writable for UhsDbgSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_DBG_SEL to value 0"]
impl crate::Resettable for UhsDbgSelSpec {}
