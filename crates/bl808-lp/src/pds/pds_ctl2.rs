#[doc = "Register `PDS_CTL2` reader"]
pub type R = crate::R<PdsCtl2Spec>;
#[doc = "Register `PDS_CTL2` writer"]
pub type W = crate::W<PdsCtl2Spec>;
#[doc = "Field `cr_pds_force_mm_pwr_off` reader - "]
pub type CrPdsForceMmPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_force_mm_pwr_off` writer - "]
pub type CrPdsForceMmPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_pwr_off` reader - "]
pub type CrPdsForceUsbPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_pwr_off` writer - "]
pub type CrPdsForceUsbPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_mm_iso_en` reader - "]
pub type CrPdsForceMmIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_force_mm_iso_en` writer - "]
pub type CrPdsForceMmIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_iso_en` reader - "]
pub type CrPdsForceUsbIsoEnR = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_iso_en` writer - "]
pub type CrPdsForceUsbIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_pds_rst` reader - "]
pub type CrPdsForceNpPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_pds_rst` writer - "]
pub type CrPdsForceNpPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_mm_pds_rst` reader - "]
pub type CrPdsForceMmPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_mm_pds_rst` writer - "]
pub type CrPdsForceMmPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_pds_rst` reader - "]
pub type CrPdsForceWbPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_pds_rst` writer - "]
pub type CrPdsForceWbPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_pds_rst` reader - "]
pub type CrPdsForceUsbPdsRstR = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_pds_rst` writer - "]
pub type CrPdsForceUsbPdsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_mem_stby` reader - "]
pub type CrPdsForceNpMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_mem_stby` writer - "]
pub type CrPdsForceNpMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_mm_mem_stby` reader - "]
pub type CrPdsForceMmMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_mm_mem_stby` writer - "]
pub type CrPdsForceMmMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_mem_stby` reader - "]
pub type CrPdsForceWbMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_mem_stby` writer - "]
pub type CrPdsForceWbMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_mem_stby` reader - "]
pub type CrPdsForceUsbMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_mem_stby` writer - "]
pub type CrPdsForceUsbMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_np_gate_clk` reader - "]
pub type CrPdsForceNpGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_np_gate_clk` writer - "]
pub type CrPdsForceNpGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_mm_gate_clk` reader - "]
pub type CrPdsForceMmGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_mm_gate_clk` writer - "]
pub type CrPdsForceMmGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_wb_gate_clk` reader - "]
pub type CrPdsForceWbGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_wb_gate_clk` writer - "]
pub type CrPdsForceWbGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_force_usb_gate_clk` reader - "]
pub type CrPdsForceUsbGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_force_usb_gate_clk` writer - "]
pub type CrPdsForceUsbGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_mm_pwr_off(&self) -> CrPdsForceMmPwrOffR {
        CrPdsForceMmPwrOffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pwr_off(&self) -> CrPdsForceUsbPwrOffR {
        CrPdsForceUsbPwrOffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_force_mm_iso_en(&self) -> CrPdsForceMmIsoEnR {
        CrPdsForceMmIsoEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_usb_iso_en(&self) -> CrPdsForceUsbIsoEnR {
        CrPdsForceUsbIsoEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&self) -> CrPdsForceNpPdsRstR {
        CrPdsForceNpPdsRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_force_mm_pds_rst(&self) -> CrPdsForceMmPdsRstR {
        CrPdsForceMmPdsRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&self) -> CrPdsForceWbPdsRstR {
        CrPdsForceWbPdsRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pds_rst(&self) -> CrPdsForceUsbPdsRstR {
        CrPdsForceUsbPdsRstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&self) -> CrPdsForceNpMemStbyR {
        CrPdsForceNpMemStbyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_mm_mem_stby(&self) -> CrPdsForceMmMemStbyR {
        CrPdsForceMmMemStbyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&self) -> CrPdsForceWbMemStbyR {
        CrPdsForceWbMemStbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_force_usb_mem_stby(&self) -> CrPdsForceUsbMemStbyR {
        CrPdsForceUsbMemStbyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&self) -> CrPdsForceNpGateClkR {
        CrPdsForceNpGateClkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_force_mm_gate_clk(&self) -> CrPdsForceMmGateClkR {
        CrPdsForceMmGateClkR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&self) -> CrPdsForceWbGateClkR {
        CrPdsForceWbGateClkR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_force_usb_gate_clk(&self) -> CrPdsForceUsbGateClkR {
        CrPdsForceUsbGateClkR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_mm_pwr_off(&mut self) -> CrPdsForceMmPwrOffW<'_, PdsCtl2Spec> {
        CrPdsForceMmPwrOffW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pwr_off(&mut self) -> CrPdsForceUsbPwrOffW<'_, PdsCtl2Spec> {
        CrPdsForceUsbPwrOffW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_force_mm_iso_en(&mut self) -> CrPdsForceMmIsoEnW<'_, PdsCtl2Spec> {
        CrPdsForceMmIsoEnW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_usb_iso_en(&mut self) -> CrPdsForceUsbIsoEnW<'_, PdsCtl2Spec> {
        CrPdsForceUsbIsoEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&mut self) -> CrPdsForceNpPdsRstW<'_, PdsCtl2Spec> {
        CrPdsForceNpPdsRstW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_force_mm_pds_rst(&mut self) -> CrPdsForceMmPdsRstW<'_, PdsCtl2Spec> {
        CrPdsForceMmPdsRstW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&mut self) -> CrPdsForceWbPdsRstW<'_, PdsCtl2Spec> {
        CrPdsForceWbPdsRstW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pds_rst(&mut self) -> CrPdsForceUsbPdsRstW<'_, PdsCtl2Spec> {
        CrPdsForceUsbPdsRstW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&mut self) -> CrPdsForceNpMemStbyW<'_, PdsCtl2Spec> {
        CrPdsForceNpMemStbyW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_mm_mem_stby(&mut self) -> CrPdsForceMmMemStbyW<'_, PdsCtl2Spec> {
        CrPdsForceMmMemStbyW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&mut self) -> CrPdsForceWbMemStbyW<'_, PdsCtl2Spec> {
        CrPdsForceWbMemStbyW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_force_usb_mem_stby(&mut self) -> CrPdsForceUsbMemStbyW<'_, PdsCtl2Spec> {
        CrPdsForceUsbMemStbyW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&mut self) -> CrPdsForceNpGateClkW<'_, PdsCtl2Spec> {
        CrPdsForceNpGateClkW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_force_mm_gate_clk(&mut self) -> CrPdsForceMmGateClkW<'_, PdsCtl2Spec> {
        CrPdsForceMmGateClkW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&mut self) -> CrPdsForceWbGateClkW<'_, PdsCtl2Spec> {
        CrPdsForceWbGateClkW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_force_usb_gate_clk(&mut self) -> CrPdsForceUsbGateClkW<'_, PdsCtl2Spec> {
        CrPdsForceUsbGateClkW::new(self, 19)
    }
}
#[doc = "PDS Force Sleep Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtl2Spec;
impl crate::RegisterSpec for PdsCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl2::R`](R) reader structure"]
impl crate::Readable for PdsCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl2::W`](W) writer structure"]
impl crate::Writable for PdsCtl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL2 to value 0x0002_2222"]
impl crate::Resettable for PdsCtl2Spec {
    const RESET_VALUE: u32 = 0x0002_2222;
}
