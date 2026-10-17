#[doc = "Register `TZC_L2SRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcL2sramTzsrgR1Spec>;
#[doc = "Register `TZC_L2SRAM_TZSRG_R1` writer"]
pub type W = crate::W<TzcL2sramTzsrgR1Spec>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_end` reader - End address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_end` writer - End address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR1EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_start` reader - Start address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_start` writer - Start address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR1StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_end(&self) -> TzcL2sramTzsrgR1EndR {
        TzcL2sramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_start(&self) -> TzcL2sramTzsrgR1StartR {
        TzcL2sramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_end(&mut self) -> TzcL2sramTzsrgR1EndW<'_, TzcL2sramTzsrgR1Spec> {
        TzcL2sramTzsrgR1EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_start(
        &mut self,
    ) -> TzcL2sramTzsrgR1StartW<'_, TzcL2sramTzsrgR1Spec> {
        TzcL2sramTzsrgR1StartW::new(self, 16)
    }
}
#[doc = "L2SRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgR1Spec;
impl crate::RegisterSpec for TzcL2sramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgR1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_l2sram_tzsrg_r1::W`](W) writer structure"]
impl crate::Writable for TzcL2sramTzsrgR1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_R1 to value 0x03ff"]
impl crate::Resettable for TzcL2sramTzsrgR1Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
