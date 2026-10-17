#[doc = "Register `PDS_RAM1` reader"]
pub type R = crate::R<PdsRam1Spec>;
#[doc = "Register `PDS_RAM1` writer"]
pub type W = crate::W<PdsRam1Spec>;
#[doc = "Field `cr_ocram_slp` reader - "]
pub type CrOcramSlpR = crate::FieldReader;
#[doc = "Field `cr_ocram_slp` writer - "]
pub type CrOcramSlpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_ocram_ret` reader - "]
pub type CrOcramRetR = crate::FieldReader;
#[doc = "Field `cr_ocram_ret` writer - "]
pub type CrOcramRetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_pds_ram_clk_cnt` reader - "]
pub type CrPdsRamClkCntR = crate::FieldReader;
#[doc = "Field `cr_pds_ram_clk_cnt` writer - "]
pub type CrPdsRamClkCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cr_pds_ram_clk2_cnt` reader - "]
pub type CrPdsRamClk2CntR = crate::FieldReader;
#[doc = "Field `cr_pds_ram_clk2_cnt` writer - "]
pub type CrPdsRamClk2CntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cr_pds_ctrl_np_ram_clk` reader - "]
pub type CrPdsCtrlNpRamClkR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_np_ram_clk` writer - "]
pub type CrPdsCtrlNpRamClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_mm_ram_clk` reader - "]
pub type CrPdsCtrlMmRamClkR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_mm_ram_clk` writer - "]
pub type CrPdsCtrlMmRamClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_wb_ram_clk` reader - "]
pub type CrPdsCtrlWbRamClkR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_wb_ram_clk` writer - "]
pub type CrPdsCtrlWbRamClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_usb_ram_clk` reader - "]
pub type CrPdsCtrlUsbRamClkR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_usb_ram_clk` writer - "]
pub type CrPdsCtrlUsbRamClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_misc_ram_clk` reader - "]
pub type CrPdsCtrlMiscRamClkR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_misc_ram_clk` writer - "]
pub type CrPdsCtrlMiscRamClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_ram_clk2` reader - "]
pub type CrPdsCtrlRamClk2R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_ram_clk2` writer - "]
pub type CrPdsCtrlRamClk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_ram_clk` reader - "]
pub type CrPdsCtrlRamClkR = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_ram_clk` writer - "]
pub type CrPdsCtrlRamClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_ocram_slp(&self) -> CrOcramSlpR {
        CrOcramSlpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_ocram_ret(&self) -> CrOcramRetR {
        CrOcramRetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cr_pds_ram_clk_cnt(&self) -> CrPdsRamClkCntR {
        CrPdsRamClkCntR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn cr_pds_ram_clk2_cnt(&self) -> CrPdsRamClk2CntR {
        CrPdsRamClk2CntR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_ctrl_np_ram_clk(&self) -> CrPdsCtrlNpRamClkR {
        CrPdsCtrlNpRamClkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_ctrl_mm_ram_clk(&self) -> CrPdsCtrlMmRamClkR {
        CrPdsCtrlMmRamClkR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wb_ram_clk(&self) -> CrPdsCtrlWbRamClkR {
        CrPdsCtrlWbRamClkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb_ram_clk(&self) -> CrPdsCtrlUsbRamClkR {
        CrPdsCtrlUsbRamClkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_pds_ctrl_misc_ram_clk(&self) -> CrPdsCtrlMiscRamClkR {
        CrPdsCtrlMiscRamClkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_ctrl_ram_clk2(&self) -> CrPdsCtrlRamClk2R {
        CrPdsCtrlRamClk2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_ram_clk(&self) -> CrPdsCtrlRamClkR {
        CrPdsCtrlRamClkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_ocram_slp(&mut self) -> CrOcramSlpW<'_, PdsRam1Spec> {
        CrOcramSlpW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_ocram_ret(&mut self) -> CrOcramRetW<'_, PdsRam1Spec> {
        CrOcramRetW::new(self, 4)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cr_pds_ram_clk_cnt(&mut self) -> CrPdsRamClkCntW<'_, PdsRam1Spec> {
        CrPdsRamClkCntW::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn cr_pds_ram_clk2_cnt(&mut self) -> CrPdsRamClk2CntW<'_, PdsRam1Spec> {
        CrPdsRamClk2CntW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_ctrl_np_ram_clk(&mut self) -> CrPdsCtrlNpRamClkW<'_, PdsRam1Spec> {
        CrPdsCtrlNpRamClkW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_ctrl_mm_ram_clk(&mut self) -> CrPdsCtrlMmRamClkW<'_, PdsRam1Spec> {
        CrPdsCtrlMmRamClkW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wb_ram_clk(&mut self) -> CrPdsCtrlWbRamClkW<'_, PdsRam1Spec> {
        CrPdsCtrlWbRamClkW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb_ram_clk(&mut self) -> CrPdsCtrlUsbRamClkW<'_, PdsRam1Spec> {
        CrPdsCtrlUsbRamClkW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_pds_ctrl_misc_ram_clk(&mut self) -> CrPdsCtrlMiscRamClkW<'_, PdsRam1Spec> {
        CrPdsCtrlMiscRamClkW::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_ctrl_ram_clk2(&mut self) -> CrPdsCtrlRamClk2W<'_, PdsRam1Spec> {
        CrPdsCtrlRamClk2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_ram_clk(&mut self) -> CrPdsCtrlRamClkW<'_, PdsRam1Spec> {
        CrPdsCtrlRamClkW::new(self, 31)
    }
}
#[doc = "PDS RAM Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ram1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ram1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsRam1Spec;
impl crate::RegisterSpec for PdsRam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ram1::R`](R) reader structure"]
impl crate::Readable for PdsRam1Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ram1::W`](W) writer structure"]
impl crate::Writable for PdsRam1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_RAM1 to value 0x0018_0800"]
impl crate::Resettable for PdsRam1Spec {
    const RESET_VALUE: u32 = 0x0018_0800;
}
