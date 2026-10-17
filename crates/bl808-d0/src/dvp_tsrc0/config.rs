#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `cr_enable` reader - Master enable for DVP Timing Source"]
pub type CrEnableR = crate::BitReader;
#[doc = "Field `cr_enable` writer - Master enable for DVP Timing Source"]
pub type CrEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_axi_en` reader - AXI to DVP streaming path enable (Memory read mode)"]
pub type CrAxiEnR = crate::BitReader;
#[doc = "Field `cr_axi_en` writer - AXI to DVP streaming path enable (Memory read mode)"]
pub type CrAxiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mode_cea_861` reader - CEA-861 timing mode enable"]
pub type CrModeCea861R = crate::BitReader;
#[doc = "Field `cr_mode_cea_861` writer - CEA-861 timing mode enable"]
pub type CrModeCea861W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_snsr_en` reader - Sensor data input enable (Direct camera mode)"]
pub type CrSnsrEnR = crate::BitReader;
#[doc = "Field `cr_snsr_en` writer - Sensor data input enable (Direct camera mode)"]
pub type CrSnsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_snsr_hsync_inv` reader - Sensor HSYNC signal inversion"]
pub type CrSnsrHsyncInvR = crate::BitReader;
#[doc = "Field `cr_snsr_hsync_inv` writer - Sensor HSYNC signal inversion"]
pub type CrSnsrHsyncInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_snsr_vsync_inv` reader - Sensor VSYNC signal inversion"]
pub type CrSnsrVsyncInvR = crate::BitReader;
#[doc = "Field `cr_snsr_vsync_inv` writer - Sensor VSYNC signal inversion"]
pub type CrSnsrVsyncInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_axi_swap_mode` reader - AXI to DVP swap buffer mode enable"]
pub type CrAxiSwapModeR = crate::BitReader;
#[doc = "Field `cr_axi_swap_mode` writer - AXI to DVP swap buffer mode enable"]
pub type CrAxiSwapModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_axi_swap_idx_sel` reader - AXI to DVP swap index selection (4-bit field)"]
pub type CrAxiSwapIdxSelR = crate::FieldReader;
#[doc = "Field `cr_axi_swap_idx_sel` writer - AXI to DVP swap index selection (4-bit field)"]
pub type CrAxiSwapIdxSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_axi_swap_idx_swm` reader - AXI swap index software mode enable"]
pub type CrAxiSwapIdxSwmR = crate::BitReader;
#[doc = "Field `cr_axi_swap_idx_swm` writer - AXI swap index software mode enable"]
pub type CrAxiSwapIdxSwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_axi_swap_idx_swv` reader - AXI swap index software value"]
pub type CrAxiSwapIdxSwvR = crate::BitReader;
#[doc = "Field `cr_axi_swap_idx_swv` writer - AXI swap index software value"]
pub type CrAxiSwapIdxSwvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_axi_dvp_data_mode` reader - AXI to DVP data format mode: 0=16-bit RGB565/YUV422, 1=24-bit, 2=32-bit, etc."]
pub type CrAxiDvpDataModeR = crate::FieldReader;
#[doc = "Field `cr_axi_dvp_data_mode` writer - AXI to DVP data format mode: 0=16-bit RGB565/YUV422, 1=24-bit, 2=32-bit, etc."]
pub type CrAxiDvpDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_axi_b0_sel` reader - Byte 0 lane selection for AXI data reordering"]
pub type CrAxiB0SelR = crate::FieldReader;
#[doc = "Field `cr_axi_b0_sel` writer - Byte 0 lane selection for AXI data reordering"]
pub type CrAxiB0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_axi_b1_sel` reader - Byte 1 lane selection for AXI data reordering"]
pub type CrAxiB1SelR = crate::FieldReader;
#[doc = "Field `cr_axi_b1_sel` writer - Byte 1 lane selection for AXI data reordering"]
pub type CrAxiB1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_axi_b2_sel` reader - Byte 2 lane selection for AXI data reordering"]
pub type CrAxiB2SelR = crate::FieldReader;
#[doc = "Field `cr_axi_b2_sel` writer - Byte 2 lane selection for AXI data reordering"]
pub type CrAxiB2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Master enable for DVP Timing Source"]
    #[inline(always)]
    pub fn cr_enable(&self) -> CrEnableR {
        CrEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI to DVP streaming path enable (Memory read mode)"]
    #[inline(always)]
    pub fn cr_axi_en(&self) -> CrAxiEnR {
        CrAxiEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CEA-861 timing mode enable"]
    #[inline(always)]
    pub fn cr_mode_cea_861(&self) -> CrModeCea861R {
        CrModeCea861R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sensor data input enable (Direct camera mode)"]
    #[inline(always)]
    pub fn cr_snsr_en(&self) -> CrSnsrEnR {
        CrSnsrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sensor HSYNC signal inversion"]
    #[inline(always)]
    pub fn cr_snsr_hsync_inv(&self) -> CrSnsrHsyncInvR {
        CrSnsrHsyncInvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sensor VSYNC signal inversion"]
    #[inline(always)]
    pub fn cr_snsr_vsync_inv(&self) -> CrSnsrVsyncInvR {
        CrSnsrVsyncInvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - AXI to DVP swap buffer mode enable"]
    #[inline(always)]
    pub fn cr_axi_swap_mode(&self) -> CrAxiSwapModeR {
        CrAxiSwapModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - AXI to DVP swap index selection (4-bit field)"]
    #[inline(always)]
    pub fn cr_axi_swap_idx_sel(&self) -> CrAxiSwapIdxSelR {
        CrAxiSwapIdxSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - AXI swap index software mode enable"]
    #[inline(always)]
    pub fn cr_axi_swap_idx_swm(&self) -> CrAxiSwapIdxSwmR {
        CrAxiSwapIdxSwmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AXI swap index software value"]
    #[inline(always)]
    pub fn cr_axi_swap_idx_swv(&self) -> CrAxiSwapIdxSwvR {
        CrAxiSwapIdxSwvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - AXI to DVP data format mode: 0=16-bit RGB565/YUV422, 1=24-bit, 2=32-bit, etc."]
    #[inline(always)]
    pub fn cr_axi_dvp_data_mode(&self) -> CrAxiDvpDataModeR {
        CrAxiDvpDataModeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Byte 0 lane selection for AXI data reordering"]
    #[inline(always)]
    pub fn cr_axi_b0_sel(&self) -> CrAxiB0SelR {
        CrAxiB0SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Byte 1 lane selection for AXI data reordering"]
    #[inline(always)]
    pub fn cr_axi_b1_sel(&self) -> CrAxiB1SelR {
        CrAxiB1SelR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Byte 2 lane selection for AXI data reordering"]
    #[inline(always)]
    pub fn cr_axi_b2_sel(&self) -> CrAxiB2SelR {
        CrAxiB2SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master enable for DVP Timing Source"]
    #[inline(always)]
    pub fn cr_enable(&mut self) -> CrEnableW<'_, ConfigSpec> {
        CrEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - AXI to DVP streaming path enable (Memory read mode)"]
    #[inline(always)]
    pub fn cr_axi_en(&mut self) -> CrAxiEnW<'_, ConfigSpec> {
        CrAxiEnW::new(self, 1)
    }
    #[doc = "Bit 2 - CEA-861 timing mode enable"]
    #[inline(always)]
    pub fn cr_mode_cea_861(&mut self) -> CrModeCea861W<'_, ConfigSpec> {
        CrModeCea861W::new(self, 2)
    }
    #[doc = "Bit 3 - Sensor data input enable (Direct camera mode)"]
    #[inline(always)]
    pub fn cr_snsr_en(&mut self) -> CrSnsrEnW<'_, ConfigSpec> {
        CrSnsrEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Sensor HSYNC signal inversion"]
    #[inline(always)]
    pub fn cr_snsr_hsync_inv(&mut self) -> CrSnsrHsyncInvW<'_, ConfigSpec> {
        CrSnsrHsyncInvW::new(self, 4)
    }
    #[doc = "Bit 5 - Sensor VSYNC signal inversion"]
    #[inline(always)]
    pub fn cr_snsr_vsync_inv(&mut self) -> CrSnsrVsyncInvW<'_, ConfigSpec> {
        CrSnsrVsyncInvW::new(self, 5)
    }
    #[doc = "Bit 7 - AXI to DVP swap buffer mode enable"]
    #[inline(always)]
    pub fn cr_axi_swap_mode(&mut self) -> CrAxiSwapModeW<'_, ConfigSpec> {
        CrAxiSwapModeW::new(self, 7)
    }
    #[doc = "Bits 8:11 - AXI to DVP swap index selection (4-bit field)"]
    #[inline(always)]
    pub fn cr_axi_swap_idx_sel(&mut self) -> CrAxiSwapIdxSelW<'_, ConfigSpec> {
        CrAxiSwapIdxSelW::new(self, 8)
    }
    #[doc = "Bit 12 - AXI swap index software mode enable"]
    #[inline(always)]
    pub fn cr_axi_swap_idx_swm(&mut self) -> CrAxiSwapIdxSwmW<'_, ConfigSpec> {
        CrAxiSwapIdxSwmW::new(self, 12)
    }
    #[doc = "Bit 13 - AXI swap index software value"]
    #[inline(always)]
    pub fn cr_axi_swap_idx_swv(&mut self) -> CrAxiSwapIdxSwvW<'_, ConfigSpec> {
        CrAxiSwapIdxSwvW::new(self, 13)
    }
    #[doc = "Bits 16:18 - AXI to DVP data format mode: 0=16-bit RGB565/YUV422, 1=24-bit, 2=32-bit, etc."]
    #[inline(always)]
    pub fn cr_axi_dvp_data_mode(&mut self) -> CrAxiDvpDataModeW<'_, ConfigSpec> {
        CrAxiDvpDataModeW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Byte 0 lane selection for AXI data reordering"]
    #[inline(always)]
    pub fn cr_axi_b0_sel(&mut self) -> CrAxiB0SelW<'_, ConfigSpec> {
        CrAxiB0SelW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Byte 1 lane selection for AXI data reordering"]
    #[inline(always)]
    pub fn cr_axi_b1_sel(&mut self) -> CrAxiB1SelW<'_, ConfigSpec> {
        CrAxiB1SelW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Byte 2 lane selection for AXI data reordering"]
    #[inline(always)]
    pub fn cr_axi_b2_sel(&mut self) -> CrAxiB2SelW<'_, ConfigSpec> {
        CrAxiB2SelW::new(self, 24)
    }
}
#[doc = "DVP Timing Source Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x0240_0000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0240_0000;
}
