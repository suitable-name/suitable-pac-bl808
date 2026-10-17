#[doc = "Register `TZC_OCRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcOcramTzsrgR2Spec>;
#[doc = "Register `TZC_OCRAM_TZSRG_R2` writer"]
pub type W = crate::W<TzcOcramTzsrgR2Spec>;
#[doc = "Field `tzc_ocram_tzsrg_r2_end` reader - End address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_end` writer - End address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_start` reader - Start address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR2StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_start` writer - Start address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_end(&self) -> TzcOcramTzsrgR2EndR {
        TzcOcramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_start(&self) -> TzcOcramTzsrgR2StartR {
        TzcOcramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_end(&mut self) -> TzcOcramTzsrgR2EndW<'_, TzcOcramTzsrgR2Spec> {
        TzcOcramTzsrgR2EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_start(&mut self) -> TzcOcramTzsrgR2StartW<'_, TzcOcramTzsrgR2Spec> {
        TzcOcramTzsrgR2StartW::new(self, 16)
    }
}
#[doc = "OCRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgR2Spec;
impl crate::RegisterSpec for TzcOcramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgR2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_ocram_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TzcOcramTzsrgR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_R2 to value 0x03ff"]
impl crate::Resettable for TzcOcramTzsrgR2Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
