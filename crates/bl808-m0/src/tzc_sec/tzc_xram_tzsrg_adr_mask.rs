#[doc = "Register `TZC_XRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcXramTzsrgAdrMaskSpec>;
#[doc = "Register `TZC_XRAM_TZSRG_ADR_MASK` writer"]
pub type W = crate::W<TzcXramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_xram_tzsrg_adr_mask` reader - Address mask bits \\[9:0\\]"]
pub type TzcXramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_xram_tzsrg_adr_mask` writer - Address mask bits \\[9:0\\]"]
pub type TzcXramTzsrgAdrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_xram_tzsrg_adr_mask_lock` reader - Address mask lock (Write-Once)"]
pub type TzcXramTzsrgAdrMaskLockR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_adr_mask_lock` writer - Address mask lock (Write-Once)"]
pub type TzcXramTzsrgAdrMaskLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Address mask bits \\[9:0\\]"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_adr_mask(&self) -> TzcXramTzsrgAdrMaskR {
        TzcXramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_adr_mask_lock(&self) -> TzcXramTzsrgAdrMaskLockR {
        TzcXramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address mask bits \\[9:0\\]"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_adr_mask(&mut self) -> TzcXramTzsrgAdrMaskW<'_, TzcXramTzsrgAdrMaskSpec> {
        TzcXramTzsrgAdrMaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TzcXramTzsrgAdrMaskLockW<'_, TzcXramTzsrgAdrMaskSpec> {
        TzcXramTzsrgAdrMaskLockW::new(self, 16)
    }
}
#[doc = "XRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_adr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcXramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgAdrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_xram_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TzcXramTzsrgAdrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcXramTzsrgAdrMaskSpec {}
