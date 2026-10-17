#[doc = "Register `PDS_CTL3` reader"]
pub type R = crate::R<PdsCtl3Spec>;
#[doc = "Register `PDS_CTL3` writer"]
pub type W = crate::W<PdsCtl3Spec>;
#[doc = "Field `cr_pds_force_misc_pwr_off` reader - "]
pub type CrPdsForceMiscPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_pwr_off` writer - "]
pub type CrPdsForceMiscPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_iso_en` reader - "]
pub type CrPdsForceMiscIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_iso_en` writer - "]
pub type CrPdsForceMiscIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_pds_rst` reader - "]
pub type CrPdsForceMiscPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_pds_rst` writer - "]
pub type CrPdsForceMiscPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_mem_stby` reader - "]
pub type CrPdsForceMiscMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_mem_stby` writer - "]
pub type CrPdsForceMiscMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_misc_gate_clk` reader - "]
pub type CrPdsForceMiscGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_misc_gate_clk` writer - "]
pub type CrPdsForceMiscGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mm_iso_en` reader - "]
pub type CrPdsMmIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_mm_iso_en` writer - "]
pub type CrPdsMmIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_iso_en` reader - "]
pub type CrPdsUsbIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_usb_iso_en` writer - "]
pub type CrPdsUsbIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_iso_en` reader - "]
pub type CrPdsMiscIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_misc_iso_en` writer - "]
pub type CrPdsMiscIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pwr_off(&self) -> CrPdsForceMiscPwrOffR {
        CrPdsForceMiscPwrOffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_misc_iso_en(&self) -> CrPdsForceMiscIsoEnR {
        CrPdsForceMiscIsoEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pds_rst(&self) -> CrPdsForceMiscPdsRstR {
        CrPdsForceMiscPdsRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_misc_mem_stby(&self) -> CrPdsForceMiscMemStbyR {
        CrPdsForceMiscMemStbyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_misc_gate_clk(&self) -> CrPdsForceMiscGateClkR {
        CrPdsForceMiscGateClkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_mm_iso_en(&self) -> CrPdsMmIsoEnR {
        CrPdsMmIsoEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_pds_usb_iso_en(&self) -> CrPdsUsbIsoEnR {
        CrPdsUsbIsoEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_iso_en(&self) -> CrPdsMiscIsoEnR {
        CrPdsMiscIsoEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pwr_off(&mut self) -> CrPdsForceMiscPwrOffW<'_, PdsCtl3Spec> {
        CrPdsForceMiscPwrOffW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_misc_iso_en(&mut self) -> CrPdsForceMiscIsoEnW<'_, PdsCtl3Spec> {
        CrPdsForceMiscIsoEnW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pds_rst(&mut self) -> CrPdsForceMiscPdsRstW<'_, PdsCtl3Spec> {
        CrPdsForceMiscPdsRstW::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_misc_mem_stby(&mut self) -> CrPdsForceMiscMemStbyW<'_, PdsCtl3Spec> {
        CrPdsForceMiscMemStbyW::new(self, 10)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_misc_gate_clk(&mut self) -> CrPdsForceMiscGateClkW<'_, PdsCtl3Spec> {
        CrPdsForceMiscGateClkW::new(self, 13)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_mm_iso_en(&mut self) -> CrPdsMmIsoEnW<'_, PdsCtl3Spec> {
        CrPdsMmIsoEnW::new(self, 26)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_pds_usb_iso_en(&mut self) -> CrPdsUsbIsoEnW<'_, PdsCtl3Spec> {
        CrPdsUsbIsoEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_iso_en(&mut self) -> CrPdsMiscIsoEnW<'_, PdsCtl3Spec> {
        CrPdsMiscIsoEnW::new(self, 30)
    }
}
#[doc = "PDS Force Sleep Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtl3Spec;
impl crate::RegisterSpec for PdsCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl3::R`](R) reader structure"]
impl crate::Readable for PdsCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl3::W`](W) writer structure"]
impl crate::Writable for PdsCtl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL3 to value 0x6400_0000"]
impl crate::Resettable for PdsCtl3Spec {
    const RESET_VALUE: u32 = 0x6400_0000;
}
