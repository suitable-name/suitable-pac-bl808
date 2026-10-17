#[doc = "Register `TZC_ROM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcRomTzsrgAdrMaskSpec>;
#[doc = "Register `TZC_ROM_TZSRG_ADR_MASK` writer"]
pub type W = crate::W<TzcRomTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask` reader - Address mask bits \\[9:0\\]"]
pub type TzcRomTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask` writer - Address mask bits \\[9:0\\]"]
pub type TzcRomTzsrgAdrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask_lock` reader - Address mask lock (Write-Once)"]
pub type TzcRomTzsrgAdrMaskLockR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_adr_mask_lock` writer - Address mask lock (Write-Once)"]
pub type TzcRomTzsrgAdrMaskLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Address mask bits \\[9:0\\]"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask(&self) -> TzcRomTzsrgAdrMaskR {
        TzcRomTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask_lock(&self) -> TzcRomTzsrgAdrMaskLockR {
        TzcRomTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address mask bits \\[9:0\\]"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask(&mut self) -> TzcRomTzsrgAdrMaskW<'_, TzcRomTzsrgAdrMaskSpec> {
        TzcRomTzsrgAdrMaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TzcRomTzsrgAdrMaskLockW<'_, TzcRomTzsrgAdrMaskSpec> {
        TzcRomTzsrgAdrMaskLockW::new(self, 16)
    }
}
#[doc = "ROM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_adr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcRomTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgAdrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TzcRomTzsrgAdrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcRomTzsrgAdrMaskSpec {}
