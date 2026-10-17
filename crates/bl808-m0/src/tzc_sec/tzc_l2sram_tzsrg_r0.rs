#[doc = "Register `TZC_L2SRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcL2sramTzsrgR0Spec>;
#[doc = "Register `TZC_L2SRAM_TZSRG_R0` writer"]
pub type W = crate::W<TzcL2sramTzsrgR0Spec>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_end` reader - End address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_end` writer - End address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR0EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_start` reader - Start address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR0StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_start` writer - Start address boundary (10-bit offset)"]
pub type TzcL2sramTzsrgR0StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_end(&self) -> TzcL2sramTzsrgR0EndR {
        TzcL2sramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_start(&self) -> TzcL2sramTzsrgR0StartR {
        TzcL2sramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_end(&mut self) -> TzcL2sramTzsrgR0EndW<'_, TzcL2sramTzsrgR0Spec> {
        TzcL2sramTzsrgR0EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_start(
        &mut self,
    ) -> TzcL2sramTzsrgR0StartW<'_, TzcL2sramTzsrgR0Spec> {
        TzcL2sramTzsrgR0StartW::new(self, 16)
    }
}
#[doc = "L2SRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_r0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgR0Spec;
impl crate::RegisterSpec for TzcL2sramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgR0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_l2sram_tzsrg_r0::W`](W) writer structure"]
impl crate::Writable for TzcL2sramTzsrgR0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_R0 to value 0x03ff"]
impl crate::Resettable for TzcL2sramTzsrgR0Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
