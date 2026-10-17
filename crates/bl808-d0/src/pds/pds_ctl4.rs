#[doc = "Register `PDS_CTL4` reader"]
pub type R = crate::R<PdsCtl4Spec>;
#[doc = "Register `PDS_CTL4` writer"]
pub type W = crate::W<PdsCtl4Spec>;
#[doc = "Field `cr_pds_np_reset` reader - "]
pub type CrPdsNpResetR = crate::BitReader;
#[doc = "Field `cr_pds_np_reset` writer - "]
pub type CrPdsNpResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_mem_stby` reader - "]
pub type CrPdsNpMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_np_mem_stby` writer - "]
pub type CrPdsNpMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_np_gate_clk` reader - "]
pub type CrPdsNpGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_np_gate_clk` writer - "]
pub type CrPdsNpGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mm_pwr_off` reader - "]
pub type CrPdsMmPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_mm_pwr_off` writer - "]
pub type CrPdsMmPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mm_reset` reader - "]
pub type CrPdsMmResetR = crate::BitReader;
#[doc = "Field `cr_pds_mm_reset` writer - "]
pub type CrPdsMmResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mm_mem_stby` reader - "]
pub type CrPdsMmMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_mm_mem_stby` writer - "]
pub type CrPdsMmMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_mm_gate_clk` reader - "]
pub type CrPdsMmGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_mm_gate_clk` writer - "]
pub type CrPdsMmGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_reset` reader - "]
pub type CrPdsWbResetR = crate::BitReader;
#[doc = "Field `cr_pds_wb_reset` writer - "]
pub type CrPdsWbResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_mem_stby` reader - "]
pub type CrPdsWbMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_wb_mem_stby` writer - "]
pub type CrPdsWbMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_wb_gate_clk` reader - "]
pub type CrPdsWbGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_wb_gate_clk` writer - "]
pub type CrPdsWbGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_pwr_off` reader - "]
pub type CrPdsUsbPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_usb_pwr_off` writer - "]
pub type CrPdsUsbPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_reset` reader - "]
pub type CrPdsUsbResetR = crate::BitReader;
#[doc = "Field `cr_pds_usb_reset` writer - "]
pub type CrPdsUsbResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_mem_stby` reader - "]
pub type CrPdsUsbMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_usb_mem_stby` writer - "]
pub type CrPdsUsbMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_usb_gate_clk` reader - "]
pub type CrPdsUsbGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_usb_gate_clk` writer - "]
pub type CrPdsUsbGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_pwr_off` reader - "]
pub type CrPdsMiscPwrOffR = crate::BitReader;
#[doc = "Field `cr_pds_misc_pwr_off` writer - "]
pub type CrPdsMiscPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_reset` reader - "]
pub type CrPdsMiscResetR = crate::BitReader;
#[doc = "Field `cr_pds_misc_reset` writer - "]
pub type CrPdsMiscResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_mem_stby` reader - "]
pub type CrPdsMiscMemStbyR = crate::BitReader;
#[doc = "Field `cr_pds_misc_mem_stby` writer - "]
pub type CrPdsMiscMemStbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_misc_gate_clk` reader - "]
pub type CrPdsMiscGateClkR = crate::BitReader;
#[doc = "Field `cr_pds_misc_gate_clk` writer - "]
pub type CrPdsMiscGateClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&self) -> CrPdsNpResetR {
        CrPdsNpResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&self) -> CrPdsNpMemStbyR {
        CrPdsNpMemStbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&self) -> CrPdsNpGateClkR {
        CrPdsNpGateClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_mm_pwr_off(&self) -> CrPdsMmPwrOffR {
        CrPdsMmPwrOffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mm_reset(&self) -> CrPdsMmResetR {
        CrPdsMmResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_mm_mem_stby(&self) -> CrPdsMmMemStbyR {
        CrPdsMmMemStbyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_mm_gate_clk(&self) -> CrPdsMmGateClkR {
        CrPdsMmGateClkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&self) -> CrPdsWbResetR {
        CrPdsWbResetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&self) -> CrPdsWbMemStbyR {
        CrPdsWbMemStbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&self) -> CrPdsWbGateClkR {
        CrPdsWbGateClkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_usb_pwr_off(&self) -> CrPdsUsbPwrOffR {
        CrPdsUsbPwrOffR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_usb_reset(&self) -> CrPdsUsbResetR {
        CrPdsUsbResetR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_usb_mem_stby(&self) -> CrPdsUsbMemStbyR {
        CrPdsUsbMemStbyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_pds_usb_gate_clk(&self) -> CrPdsUsbGateClkR {
        CrPdsUsbGateClkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&self) -> CrPdsMiscPwrOffR {
        CrPdsMiscPwrOffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&self) -> CrPdsMiscResetR {
        CrPdsMiscResetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&self) -> CrPdsMiscMemStbyR {
        CrPdsMiscMemStbyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&self) -> CrPdsMiscGateClkR {
        CrPdsMiscGateClkR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&mut self) -> CrPdsNpResetW<'_, PdsCtl4Spec> {
        CrPdsNpResetW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&mut self) -> CrPdsNpMemStbyW<'_, PdsCtl4Spec> {
        CrPdsNpMemStbyW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&mut self) -> CrPdsNpGateClkW<'_, PdsCtl4Spec> {
        CrPdsNpGateClkW::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_mm_pwr_off(&mut self) -> CrPdsMmPwrOffW<'_, PdsCtl4Spec> {
        CrPdsMmPwrOffW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mm_reset(&mut self) -> CrPdsMmResetW<'_, PdsCtl4Spec> {
        CrPdsMmResetW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_mm_mem_stby(&mut self) -> CrPdsMmMemStbyW<'_, PdsCtl4Spec> {
        CrPdsMmMemStbyW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_mm_gate_clk(&mut self) -> CrPdsMmGateClkW<'_, PdsCtl4Spec> {
        CrPdsMmGateClkW::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&mut self) -> CrPdsWbResetW<'_, PdsCtl4Spec> {
        CrPdsWbResetW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&mut self) -> CrPdsWbMemStbyW<'_, PdsCtl4Spec> {
        CrPdsWbMemStbyW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&mut self) -> CrPdsWbGateClkW<'_, PdsCtl4Spec> {
        CrPdsWbGateClkW::new(self, 15)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_usb_pwr_off(&mut self) -> CrPdsUsbPwrOffW<'_, PdsCtl4Spec> {
        CrPdsUsbPwrOffW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_usb_reset(&mut self) -> CrPdsUsbResetW<'_, PdsCtl4Spec> {
        CrPdsUsbResetW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_usb_mem_stby(&mut self) -> CrPdsUsbMemStbyW<'_, PdsCtl4Spec> {
        CrPdsUsbMemStbyW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_pds_usb_gate_clk(&mut self) -> CrPdsUsbGateClkW<'_, PdsCtl4Spec> {
        CrPdsUsbGateClkW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&mut self) -> CrPdsMiscPwrOffW<'_, PdsCtl4Spec> {
        CrPdsMiscPwrOffW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&mut self) -> CrPdsMiscResetW<'_, PdsCtl4Spec> {
        CrPdsMiscResetW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&mut self) -> CrPdsMiscMemStbyW<'_, PdsCtl4Spec> {
        CrPdsMiscMemStbyW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&mut self) -> CrPdsMiscGateClkW<'_, PdsCtl4Spec> {
        CrPdsMiscGateClkW::new(self, 27)
    }
}
#[doc = "PDS Individual Domain Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtl4Spec;
impl crate::RegisterSpec for PdsCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl4::R`](R) reader structure"]
impl crate::Readable for PdsCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl4::W`](W) writer structure"]
impl crate::Writable for PdsCtl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL4 to value 0x0ff0_ef0e"]
impl crate::Resettable for PdsCtl4Spec {
    const RESET_VALUE: u32 = 0x0ff0_ef0e;
}
