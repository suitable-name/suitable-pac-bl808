#[doc = "Register `dac_drc_0` reader"]
pub type R = crate::R<DacDrc0Spec>;
#[doc = "Register `dac_drc_0` writer"]
pub type W = crate::W<DacDrc0Spec>;
#[doc = "Field `drc_ngt_downsample_en` reader - "]
pub type DrcNgtDownsampleEnR = crate::BitReader;
#[doc = "Field `drc_ngt_downsample_en` writer - "]
pub type DrcNgtDownsampleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `drc_rls_time` reader - "]
pub type DrcRlsTimeR = crate::FieldReader;
#[doc = "Field `drc_rls_time` writer - "]
pub type DrcRlsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `drc_att_time` reader - "]
pub type DrcAttTimeR = crate::FieldReader;
#[doc = "Field `drc_att_time` writer - "]
pub type DrcAttTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `drc_avg_time` reader - "]
pub type DrcAvgTimeR = crate::FieldReader;
#[doc = "Field `drc_avg_time` writer - "]
pub type DrcAvgTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `drc_avg_en` reader - "]
pub type DrcAvgEnR = crate::BitReader;
#[doc = "Field `drc_avg_en` writer - "]
pub type DrcAvgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `drc_avg_sel` reader - "]
pub type DrcAvgSelR = crate::FieldReader;
#[doc = "Field `drc_avg_sel` writer - "]
pub type DrcAvgSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `drc_peak_sel` reader - "]
pub type DrcPeakSelR = crate::FieldReader;
#[doc = "Field `drc_peak_sel` writer - "]
pub type DrcPeakSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `drc_raw_sel` reader - "]
pub type DrcRawSelR = crate::FieldReader;
#[doc = "Field `drc_raw_sel` writer - "]
pub type DrcRawSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `drc_en` reader - "]
pub type DrcEnR = crate::BitReader;
#[doc = "Field `drc_en` writer - "]
pub type DrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn drc_ngt_downsample_en(&self) -> DrcNgtDownsampleEnR {
        DrcNgtDownsampleEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn drc_rls_time(&self) -> DrcRlsTimeR {
        DrcRlsTimeR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18"]
    #[inline(always)]
    pub fn drc_att_time(&self) -> DrcAttTimeR {
        DrcAttTimeR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn drc_avg_time(&self) -> DrcAvgTimeR {
        DrcAvgTimeR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn drc_avg_en(&self) -> DrcAvgEnR {
        DrcAvgEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn drc_avg_sel(&self) -> DrcAvgSelR {
        DrcAvgSelR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn drc_peak_sel(&self) -> DrcPeakSelR {
        DrcPeakSelR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn drc_raw_sel(&self) -> DrcRawSelR {
        DrcRawSelR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn drc_en(&self) -> DrcEnR {
        DrcEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn drc_ngt_downsample_en(&mut self) -> DrcNgtDownsampleEnW<'_, DacDrc0Spec> {
        DrcNgtDownsampleEnW::new(self, 8)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn drc_rls_time(&mut self) -> DrcRlsTimeW<'_, DacDrc0Spec> {
        DrcRlsTimeW::new(self, 9)
    }
    #[doc = "Bits 14:18"]
    #[inline(always)]
    pub fn drc_att_time(&mut self) -> DrcAttTimeW<'_, DacDrc0Spec> {
        DrcAttTimeW::new(self, 14)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn drc_avg_time(&mut self) -> DrcAvgTimeW<'_, DacDrc0Spec> {
        DrcAvgTimeW::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn drc_avg_en(&mut self) -> DrcAvgEnW<'_, DacDrc0Spec> {
        DrcAvgEnW::new(self, 24)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn drc_avg_sel(&mut self) -> DrcAvgSelW<'_, DacDrc0Spec> {
        DrcAvgSelW::new(self, 25)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn drc_peak_sel(&mut self) -> DrcPeakSelW<'_, DacDrc0Spec> {
        DrcPeakSelW::new(self, 27)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn drc_raw_sel(&mut self) -> DrcRawSelW<'_, DacDrc0Spec> {
        DrcRawSelW::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn drc_en(&mut self) -> DrcEnW<'_, DacDrc0Spec> {
        DrcEnW::new(self, 31)
    }
}
#[doc = "DAC Dynamic Range Compression 0: Peak and average timing settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_drc_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_drc_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDrc0Spec;
impl crate::RegisterSpec for DacDrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_drc_0::R`](R) reader structure"]
impl crate::Readable for DacDrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_drc_0::W`](W) writer structure"]
impl crate::Writable for DacDrc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_drc_0 to value 0"]
impl crate::Resettable for DacDrc0Spec {}
