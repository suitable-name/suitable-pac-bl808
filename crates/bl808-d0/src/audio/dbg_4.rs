#[doc = "Register `dbg_4` reader"]
pub type R = crate::R<Dbg4Spec>;
#[doc = "Register `dbg_4` writer"]
pub type W = crate::W<Dbg4Spec>;
#[doc = "Field `adc_in_ratio_4s_val` reader - "]
pub type AdcInRatio4sValR = crate::BitReader;
#[doc = "Field `adc_in_ratio_4s_val` writer - "]
pub type AdcInRatio4sValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_in_ch0_ratio_4s_val` reader - "]
pub type AdcInCh0Ratio4sValR = crate::BitReader;
#[doc = "Field `adc_in_ch0_ratio_4s_val` writer - "]
pub type AdcInCh0Ratio4sValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_in_ch1_ratio_4s_val` reader - "]
pub type AdcInCh1Ratio4sValR = crate::BitReader;
#[doc = "Field `adc_in_ch1_ratio_4s_val` writer - "]
pub type AdcInCh1Ratio4sValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_in_ch2_ratio_4s_val` reader - "]
pub type AdcInCh2Ratio4sValR = crate::BitReader;
#[doc = "Field `adc_in_ch2_ratio_4s_val` writer - "]
pub type AdcInCh2Ratio4sValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdm_in_ratio_4s_val` reader - "]
pub type PdmInRatio4sValR = crate::BitReader;
#[doc = "Field `pdm_in_ratio_4s_val` writer - "]
pub type PdmInRatio4sValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdm_in_ratio_4s` reader - "]
pub type PdmInRatio4sR = crate::BitReader;
#[doc = "Field `pdm_in_ratio_4s` writer - "]
pub type PdmInRatio4sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_out_ratio_4s_val` reader - "]
pub type DacOutRatio4sValR = crate::FieldReader;
#[doc = "Field `dac_out_ratio_4s_val` writer - "]
pub type DacOutRatio4sValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aec_fs_rate_4s_val` reader - "]
pub type AecFsRate4sValR = crate::FieldReader;
#[doc = "Field `aec_fs_rate_4s_val` writer - "]
pub type AecFsRate4sValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_in_ratio_4s_val(&self) -> AdcInRatio4sValR {
        AdcInRatio4sValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_in_ch0_ratio_4s_val(&self) -> AdcInCh0Ratio4sValR {
        AdcInCh0Ratio4sValR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_in_ch1_ratio_4s_val(&self) -> AdcInCh1Ratio4sValR {
        AdcInCh1Ratio4sValR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adc_in_ch2_ratio_4s_val(&self) -> AdcInCh2Ratio4sValR {
        AdcInCh2Ratio4sValR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pdm_in_ratio_4s_val(&self) -> PdmInRatio4sValR {
        PdmInRatio4sValR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pdm_in_ratio_4s(&self) -> PdmInRatio4sR {
        PdmInRatio4sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn dac_out_ratio_4s_val(&self) -> DacOutRatio4sValR {
        DacOutRatio4sValR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn aec_fs_rate_4s_val(&self) -> AecFsRate4sValR {
        AecFsRate4sValR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_in_ratio_4s_val(&mut self) -> AdcInRatio4sValW<'_, Dbg4Spec> {
        AdcInRatio4sValW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_in_ch0_ratio_4s_val(&mut self) -> AdcInCh0Ratio4sValW<'_, Dbg4Spec> {
        AdcInCh0Ratio4sValW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_in_ch1_ratio_4s_val(&mut self) -> AdcInCh1Ratio4sValW<'_, Dbg4Spec> {
        AdcInCh1Ratio4sValW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adc_in_ch2_ratio_4s_val(&mut self) -> AdcInCh2Ratio4sValW<'_, Dbg4Spec> {
        AdcInCh2Ratio4sValW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pdm_in_ratio_4s_val(&mut self) -> PdmInRatio4sValW<'_, Dbg4Spec> {
        PdmInRatio4sValW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pdm_in_ratio_4s(&mut self) -> PdmInRatio4sW<'_, Dbg4Spec> {
        PdmInRatio4sW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn dac_out_ratio_4s_val(&mut self) -> DacOutRatio4sValW<'_, Dbg4Spec> {
        DacOutRatio4sValW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn aec_fs_rate_4s_val(&mut self) -> AecFsRate4sValW<'_, Dbg4Spec> {
        AecFsRate4sValW::new(self, 8)
    }
}
#[doc = "Debug Register 4: Audio synchronization ratio monitoring\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg4Spec;
impl crate::RegisterSpec for Dbg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_4::R`](R) reader structure"]
impl crate::Readable for Dbg4Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_4::W`](W) writer structure"]
impl crate::Writable for Dbg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dbg_4 to value 0"]
impl crate::Resettable for Dbg4Spec {}
