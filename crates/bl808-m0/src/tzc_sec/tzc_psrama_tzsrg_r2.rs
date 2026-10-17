#[doc = "Register `TZC_PSRAMA_TZSRG_R2` reader"]
pub type R = crate::R<TzcPsramaTzsrgR2Spec>;
#[doc = "Register `TZC_PSRAMA_TZSRG_R2` writer"]
pub type W = crate::W<TzcPsramaTzsrgR2Spec>;
#[doc = "Field `tzc_psrama_tzsrg_r2_end` reader - End address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_r2_end` writer - End address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psrama_tzsrg_r2_start` reader - Start address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR2StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_r2_start` writer - Start address boundary (16-bit offset)"]
pub type TzcPsramaTzsrgR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_end(&self) -> TzcPsramaTzsrgR2EndR {
        TzcPsramaTzsrgR2EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_start(&self) -> TzcPsramaTzsrgR2StartR {
        TzcPsramaTzsrgR2StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_end(&mut self) -> TzcPsramaTzsrgR2EndW<'_, TzcPsramaTzsrgR2Spec> {
        TzcPsramaTzsrgR2EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_start(
        &mut self,
    ) -> TzcPsramaTzsrgR2StartW<'_, TzcPsramaTzsrgR2Spec> {
        TzcPsramaTzsrgR2StartW::new(self, 16)
    }
}
#[doc = "PSRAM-A Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgR2Spec;
impl crate::RegisterSpec for TzcPsramaTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgR2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psrama_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TzcPsramaTzsrgR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_R2 to value 0xffff"]
impl crate::Resettable for TzcPsramaTzsrgR2Spec {
    const RESET_VALUE: u32 = 0xffff;
}
