#[doc = "Register `HBN_PIR_CFG` reader"]
pub type R = crate::R<HbnPirCfgSpec>;
#[doc = "Register `HBN_PIR_CFG` writer"]
pub type W = crate::W<HbnPirCfgSpec>;
#[doc = "Field `pir_hpf_sel` reader - "]
pub type PirHpfSelR = crate::FieldReader;
#[doc = "Field `pir_hpf_sel` writer - "]
pub type PirHpfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pir_lpf_sel` reader - "]
pub type PirLpfSelR = crate::BitReader;
#[doc = "Field `pir_lpf_sel` writer - "]
pub type PirLpfSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pir_dis` reader - "]
pub type PirDisR = crate::FieldReader;
#[doc = "Field `pir_dis` writer - "]
pub type PirDisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pir_en` reader - "]
pub type PirEnR = crate::BitReader;
#[doc = "Field `pir_en` writer - "]
pub type PirEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_cs` reader - "]
pub type GpadcCsR = crate::BitReader;
#[doc = "Field `gpadc_cs` writer - "]
pub type GpadcCsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&self) -> PirHpfSelR {
        PirHpfSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&self) -> PirLpfSelR {
        PirLpfSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&self) -> PirDisR {
        PirDisR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&self) -> PirEnR {
        PirEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cs(&self) -> GpadcCsR {
        GpadcCsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&mut self) -> PirHpfSelW<'_, HbnPirCfgSpec> {
        PirHpfSelW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&mut self) -> PirLpfSelW<'_, HbnPirCfgSpec> {
        PirLpfSelW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&mut self) -> PirDisW<'_, HbnPirCfgSpec> {
        PirDisW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&mut self) -> PirEnW<'_, HbnPirCfgSpec> {
        PirEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cs(&mut self) -> GpadcCsW<'_, HbnPirCfgSpec> {
        GpadcCsW::new(self, 8)
    }
}
#[doc = "PIR Sensor Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pir_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pir_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnPirCfgSpec;
impl crate::RegisterSpec for HbnPirCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pir_cfg::R`](R) reader structure"]
impl crate::Readable for HbnPirCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_pir_cfg::W`](W) writer structure"]
impl crate::Writable for HbnPirCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_PIR_CFG to value 0"]
impl crate::Resettable for HbnPirCfgSpec {}
