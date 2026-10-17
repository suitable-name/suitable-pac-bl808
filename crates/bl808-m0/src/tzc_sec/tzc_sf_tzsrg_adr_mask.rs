#[doc = "Register `TZC_SF_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcSfTzsrgAdrMaskSpec>;
#[doc = "Register `TZC_SF_TZSRG_ADR_MASK` writer"]
pub type W = crate::W<TzcSfTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask` reader - Address mask bits \\[18:0\\]"]
pub type TzcSfTzsrgAdrMaskR = crate::FieldReader<u32>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask` writer - Address mask bits \\[18:0\\]"]
pub type TzcSfTzsrgAdrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` reader - Address mask lock (Write-Once)"]
pub type TzcSfTzsrgAdrMaskLockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` writer - Address mask lock (Write-Once)"]
pub type TzcSfTzsrgAdrMaskLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - Address mask bits \\[18:0\\]"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask(&self) -> TzcSfTzsrgAdrMaskR {
        TzcSfTzsrgAdrMaskR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 31 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask_lock(&self) -> TzcSfTzsrgAdrMaskLockR {
        TzcSfTzsrgAdrMaskLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - Address mask bits \\[18:0\\]"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask(&mut self) -> TzcSfTzsrgAdrMaskW<'_, TzcSfTzsrgAdrMaskSpec> {
        TzcSfTzsrgAdrMaskW::new(self, 0)
    }
    #[doc = "Bit 31 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TzcSfTzsrgAdrMaskLockW<'_, TzcSfTzsrgAdrMaskSpec> {
        TzcSfTzsrgAdrMaskLockW::new(self, 31)
    }
}
#[doc = "Serial Flash Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_adr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcSfTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgAdrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TzcSfTzsrgAdrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SF_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcSfTzsrgAdrMaskSpec {}
