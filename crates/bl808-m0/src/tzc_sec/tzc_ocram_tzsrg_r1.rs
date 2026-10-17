#[doc = "Register `TZC_OCRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcOcramTzsrgR1Spec>;
#[doc = "Register `TZC_OCRAM_TZSRG_R1` writer"]
pub type W = crate::W<TzcOcramTzsrgR1Spec>;
#[doc = "Field `tzc_ocram_tzsrg_r1_end` reader - End address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r1_end` writer - End address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR1EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_ocram_tzsrg_r1_start` reader - Start address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r1_start` writer - Start address boundary (10-bit offset)"]
pub type TzcOcramTzsrgR1StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_end(&self) -> TzcOcramTzsrgR1EndR {
        TzcOcramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_start(&self) -> TzcOcramTzsrgR1StartR {
        TzcOcramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_end(&mut self) -> TzcOcramTzsrgR1EndW<'_, TzcOcramTzsrgR1Spec> {
        TzcOcramTzsrgR1EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_start(&mut self) -> TzcOcramTzsrgR1StartW<'_, TzcOcramTzsrgR1Spec> {
        TzcOcramTzsrgR1StartW::new(self, 16)
    }
}
#[doc = "OCRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgR1Spec;
impl crate::RegisterSpec for TzcOcramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgR1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_ocram_tzsrg_r1::W`](W) writer structure"]
impl crate::Writable for TzcOcramTzsrgR1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_R1 to value 0x03ff"]
impl crate::Resettable for TzcOcramTzsrgR1Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
