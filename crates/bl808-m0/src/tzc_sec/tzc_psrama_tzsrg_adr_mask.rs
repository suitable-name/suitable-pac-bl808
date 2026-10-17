#[doc = "Register `TZC_PSRAMA_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcPsramaTzsrgAdrMaskSpec>;
#[doc = "Register `TZC_PSRAMA_TZSRG_ADR_MASK` writer"]
pub type W = crate::W<TzcPsramaTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_psrama_tzsrg_adr_mask` reader - Address mask bits \\[15:0\\]"]
pub type TzcPsramaTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_adr_mask` writer - Address mask bits \\[15:0\\]"]
pub type TzcPsramaTzsrgAdrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psrama_tzsrg_adr_mask_lock` reader - Address mask lock (Write-Once)"]
pub type TzcPsramaTzsrgAdrMaskLockR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_adr_mask_lock` writer - Address mask lock (Write-Once)"]
pub type TzcPsramaTzsrgAdrMaskLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Address mask bits \\[15:0\\]"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_adr_mask(&self) -> TzcPsramaTzsrgAdrMaskR {
        TzcPsramaTzsrgAdrMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_adr_mask_lock(&self) -> TzcPsramaTzsrgAdrMaskLockR {
        TzcPsramaTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Address mask bits \\[15:0\\]"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_adr_mask(
        &mut self,
    ) -> TzcPsramaTzsrgAdrMaskW<'_, TzcPsramaTzsrgAdrMaskSpec> {
        TzcPsramaTzsrgAdrMaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TzcPsramaTzsrgAdrMaskLockW<'_, TzcPsramaTzsrgAdrMaskSpec> {
        TzcPsramaTzsrgAdrMaskLockW::new(self, 16)
    }
}
#[doc = "PSRAM-A Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_adr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcPsramaTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgAdrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psrama_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TzcPsramaTzsrgAdrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcPsramaTzsrgAdrMaskSpec {}
