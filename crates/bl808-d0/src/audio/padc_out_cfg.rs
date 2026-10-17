#[doc = "Register `padc_out_cfg` reader"]
pub type R = crate::R<PadcOutCfgSpec>;
#[doc = "Register `padc_out_cfg` writer"]
pub type W = crate::W<PadcOutCfgSpec>;
#[doc = "Field `padc_valid_4s_val` reader - "]
pub type PadcValid4sValR = crate::BitReader;
#[doc = "Field `padc_valid_4s_val` writer - "]
pub type PadcValid4sValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_valid_4s_en` reader - "]
pub type PadcValid4sEnR = crate::BitReader;
#[doc = "Field `padc_valid_4s_en` writer - "]
pub type PadcValid4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_global_chop_en` reader - "]
pub type PadcGlobalChopEnR = crate::BitReader;
#[doc = "Field `padc_global_chop_en` writer - "]
pub type PadcGlobalChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_filter_type` reader - "]
pub type PadcFilterTypeR = crate::BitReader;
#[doc = "Field `padc_filter_type` writer - "]
pub type PadcFilterTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_odr` reader - Output Data Rate selection"]
pub type PadcOdrR = crate::FieldReader;
#[doc = "Field `padc_odr` writer - Output Data Rate selection"]
pub type PadcOdrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn padc_valid_4s_val(&self) -> PadcValid4sValR {
        PadcValid4sValR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn padc_valid_4s_en(&self) -> PadcValid4sEnR {
        PadcValid4sEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn padc_global_chop_en(&self) -> PadcGlobalChopEnR {
        PadcGlobalChopEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn padc_filter_type(&self) -> PadcFilterTypeR {
        PadcFilterTypeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Output Data Rate selection"]
    #[inline(always)]
    pub fn padc_odr(&self) -> PadcOdrR {
        PadcOdrR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn padc_valid_4s_val(&mut self) -> PadcValid4sValW<'_, PadcOutCfgSpec> {
        PadcValid4sValW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn padc_valid_4s_en(&mut self) -> PadcValid4sEnW<'_, PadcOutCfgSpec> {
        PadcValid4sEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn padc_global_chop_en(&mut self) -> PadcGlobalChopEnW<'_, PadcOutCfgSpec> {
        PadcGlobalChopEnW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn padc_filter_type(&mut self) -> PadcFilterTypeW<'_, PadcOutCfgSpec> {
        PadcFilterTypeW::new(self, 24)
    }
    #[doc = "Bits 25:28 - Output Data Rate selection"]
    #[inline(always)]
    pub fn padc_odr(&mut self) -> PadcOdrW<'_, PadcOutCfgSpec> {
        PadcOdrW::new(self, 25)
    }
}
#[doc = "Passthrough ADC Output Config: Data rate and filter types\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_out_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_out_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOutCfgSpec;
impl crate::RegisterSpec for PadcOutCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_out_cfg::R`](R) reader structure"]
impl crate::Readable for PadcOutCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_out_cfg::W`](W) writer structure"]
impl crate::Writable for PadcOutCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_out_cfg to value 0"]
impl crate::Resettable for PadcOutCfgSpec {}
