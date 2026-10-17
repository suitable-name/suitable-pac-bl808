#[doc = "Register `PERI_APB_CTRL` reader"]
pub type R = crate::R<PeriApbCtrlSpec>;
#[doc = "Register `PERI_APB_CTRL` writer"]
pub type W = crate::W<PeriApbCtrlSpec>;
#[doc = "Field `reg_mminfra_berr_int_en` reader - Enable interrupt for Multimedia Infrastructure Bus Errors."]
pub type RegMminfraBerrIntEnR = crate::BitReader;
#[doc = "Field `reg_mminfra_berr_int_en` writer - Enable interrupt for Multimedia Infrastructure Bus Errors."]
pub type RegMminfraBerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_berr_int_en` reader - Enable general Bus Error interrupt."]
pub type RegBerrIntEnR = crate::BitReader;
#[doc = "Field `reg_berr_int_en` writer - Enable general Bus Error interrupt."]
pub type RegBerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_codec_berr_int_en` reader - Enable CODEC subsystem Bus Error interrupt."]
pub type RegCodecBerrIntEnR = crate::BitReader;
#[doc = "Field `reg_codec_berr_int_en` writer - Enable CODEC subsystem Bus Error interrupt."]
pub type RegCodecBerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mmcpu_berr_int_en` reader - Enable MM CPU Bus Error interrupt."]
pub type RegMmcpuBerrIntEnR = crate::BitReader;
#[doc = "Field `reg_mmcpu_berr_int_en` writer - Enable MM CPU Bus Error interrupt."]
pub type RegMmcpuBerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mm_x2hs_sp_bypass` reader - Bypass MM AXI to AHB single port bridge."]
pub type RegMmX2hsSpBypassR = crate::BitReader;
#[doc = "Field `reg_mm_x2hs_sp_bypass` writer - Bypass MM AXI to AHB single port bridge."]
pub type RegMmX2hsSpBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rg_pclk_force_on` reader - Force Peripheral PCLK active (16-bit mask for various blocks)."]
pub type RgPclkForceOnR = crate::FieldReader<u16>;
#[doc = "Field `rg_pclk_force_on` writer - Force Peripheral PCLK active (16-bit mask for various blocks)."]
pub type RgPclkForceOnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable interrupt for Multimedia Infrastructure Bus Errors."]
    #[inline(always)]
    pub fn reg_mminfra_berr_int_en(&self) -> RegMminfraBerrIntEnR {
        RegMminfraBerrIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable general Bus Error interrupt."]
    #[inline(always)]
    pub fn reg_berr_int_en(&self) -> RegBerrIntEnR {
        RegBerrIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable CODEC subsystem Bus Error interrupt."]
    #[inline(always)]
    pub fn reg_codec_berr_int_en(&self) -> RegCodecBerrIntEnR {
        RegCodecBerrIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable MM CPU Bus Error interrupt."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_int_en(&self) -> RegMmcpuBerrIntEnR {
        RegMmcpuBerrIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Bypass MM AXI to AHB single port bridge."]
    #[inline(always)]
    pub fn reg_mm_x2hs_sp_bypass(&self) -> RegMmX2hsSpBypassR {
        RegMmX2hsSpBypassR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Force Peripheral PCLK active (16-bit mask for various blocks)."]
    #[inline(always)]
    pub fn rg_pclk_force_on(&self) -> RgPclkForceOnR {
        RgPclkForceOnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt for Multimedia Infrastructure Bus Errors."]
    #[inline(always)]
    pub fn reg_mminfra_berr_int_en(&mut self) -> RegMminfraBerrIntEnW<'_, PeriApbCtrlSpec> {
        RegMminfraBerrIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable general Bus Error interrupt."]
    #[inline(always)]
    pub fn reg_berr_int_en(&mut self) -> RegBerrIntEnW<'_, PeriApbCtrlSpec> {
        RegBerrIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable CODEC subsystem Bus Error interrupt."]
    #[inline(always)]
    pub fn reg_codec_berr_int_en(&mut self) -> RegCodecBerrIntEnW<'_, PeriApbCtrlSpec> {
        RegCodecBerrIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable MM CPU Bus Error interrupt."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_int_en(&mut self) -> RegMmcpuBerrIntEnW<'_, PeriApbCtrlSpec> {
        RegMmcpuBerrIntEnW::new(self, 3)
    }
    #[doc = "Bit 8 - Bypass MM AXI to AHB single port bridge."]
    #[inline(always)]
    pub fn reg_mm_x2hs_sp_bypass(&mut self) -> RegMmX2hsSpBypassW<'_, PeriApbCtrlSpec> {
        RegMmX2hsSpBypassW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Force Peripheral PCLK active (16-bit mask for various blocks)."]
    #[inline(always)]
    pub fn rg_pclk_force_on(&mut self) -> RgPclkForceOnW<'_, PeriApbCtrlSpec> {
        RgPclkForceOnW::new(self, 16)
    }
}
#[doc = "Multimedia Peripheral APB and Bus Error Control\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_apb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_apb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriApbCtrlSpec;
impl crate::RegisterSpec for PeriApbCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_apb_ctrl::R`](R) reader structure"]
impl crate::Readable for PeriApbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`peri_apb_ctrl::W`](W) writer structure"]
impl crate::Writable for PeriApbCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_APB_CTRL to value 0"]
impl crate::Resettable for PeriApbCtrlSpec {}
