#[doc = "Register `TZC_L2SRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcL2sramTzsrgAdrMaskSpec>;
#[doc = "Register `TZC_L2SRAM_TZSRG_ADR_MASK` writer"]
pub type W = crate::W<TzcL2sramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_l2sram_tzsrg_adr_mask` reader - Address mask bits \\[9:0\\]"]
pub type TzcL2sramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_adr_mask` writer - Address mask bits \\[9:0\\]"]
pub type TzcL2sramTzsrgAdrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_l2sram_tzsrg_adr_mask_lock` reader - Address mask lock (Write-Once)"]
pub type TzcL2sramTzsrgAdrMaskLockR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_adr_mask_lock` writer - Address mask lock (Write-Once)"]
pub type TzcL2sramTzsrgAdrMaskLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Address mask bits \\[9:0\\]"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_adr_mask(&self) -> TzcL2sramTzsrgAdrMaskR {
        TzcL2sramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_adr_mask_lock(&self) -> TzcL2sramTzsrgAdrMaskLockR {
        TzcL2sramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address mask bits \\[9:0\\]"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_adr_mask(
        &mut self,
    ) -> TzcL2sramTzsrgAdrMaskW<'_, TzcL2sramTzsrgAdrMaskSpec> {
        TzcL2sramTzsrgAdrMaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TzcL2sramTzsrgAdrMaskLockW<'_, TzcL2sramTzsrgAdrMaskSpec> {
        TzcL2sramTzsrgAdrMaskLockW::new(self, 16)
    }
}
#[doc = "L2SRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_adr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcL2sramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgAdrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_l2sram_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TzcL2sramTzsrgAdrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcL2sramTzsrgAdrMaskSpec {}
