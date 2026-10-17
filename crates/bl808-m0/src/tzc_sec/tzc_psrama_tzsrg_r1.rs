#[doc = "Register `TZC_PSRAMA_TZSRG_R1` reader"]
pub type R = crate::R<TzcPsramaTzsrgR1Spec>;
#[doc = "Register `TZC_PSRAMA_TZSRG_R1` writer"]
pub type W = crate::W<TzcPsramaTzsrgR1Spec>;
#[doc = "Field `tzc_psrama_tzsrg_r1_end` reader - End address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_r1_end` writer - End address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR1EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psrama_tzsrg_r1_start` reader - Start address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_r1_start` writer - Start address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR1StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_end(&self) -> TzcPsramaTzsrgR1EndR {
        TzcPsramaTzsrgR1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_start(&self) -> TzcPsramaTzsrgR1StartR {
        TzcPsramaTzsrgR1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_end(&mut self) -> TzcPsramaTzsrgR1EndW<'_, TzcPsramaTzsrgR1Spec> {
        TzcPsramaTzsrgR1EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_start(
        &mut self,
    ) -> TzcPsramaTzsrgR1StartW<'_, TzcPsramaTzsrgR1Spec> {
        TzcPsramaTzsrgR1StartW::new(self, 16)
    }
}
#[doc = "PSRAM-A Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgR1Spec;
impl crate::RegisterSpec for TzcPsramaTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgR1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psrama_tzsrg_r1::W`](W) writer structure"]
impl crate::Writable for TzcPsramaTzsrgR1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_R1 to value 0xffff"]
impl crate::Resettable for TzcPsramaTzsrgR1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
