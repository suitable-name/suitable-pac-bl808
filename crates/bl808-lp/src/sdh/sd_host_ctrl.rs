#[doc = "Register `SD_HOST_CTRL` reader"]
pub type R = crate::R<SdHostCtrlSpec>;
#[doc = "Register `SD_HOST_CTRL` writer"]
pub type W = crate::W<SdHostCtrlSpec>;
#[doc = "Field `LED_CTRL` reader - "]
pub type LedCtrlR = crate::BitReader;
#[doc = "Field `LED_CTRL` writer - "]
pub type LedCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_WIDTH` reader - "]
pub type DataWidthR = crate::BitReader;
#[doc = "Field `DATA_WIDTH` writer - "]
pub type DataWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_SPEED_EN` reader - "]
pub type HiSpeedEnR = crate::BitReader;
#[doc = "Field `HI_SPEED_EN` writer - "]
pub type HiSpeedEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SEL` reader - "]
pub type DmaSelR = crate::FieldReader;
#[doc = "Field `DMA_SEL` writer - "]
pub type DmaSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EX_DATA_WIDTH` reader - "]
pub type ExDataWidthR = crate::BitReader;
#[doc = "Field `EX_DATA_WIDTH` writer - "]
pub type ExDataWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DET_L` reader - "]
pub type CardDetLR = crate::BitReader;
#[doc = "Field `CARD_DET_L` writer - "]
pub type CardDetLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DET_S` reader - "]
pub type CardDetSR = crate::BitReader;
#[doc = "Field `CARD_DET_S` writer - "]
pub type CardDetSW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_BUS_POWER` reader - "]
pub type SdBusPowerR = crate::BitReader;
#[doc = "Field `SD_BUS_POWER` writer - "]
pub type SdBusPowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_BUS_VLT` reader - "]
pub type SdBusVltR = crate::FieldReader;
#[doc = "Field `SD_BUS_VLT` writer - "]
pub type SdBusVltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LedCtrlR {
        LedCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hi_speed_en(&self) -> HiSpeedEnR {
        HiSpeedEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DmaSelR {
        DmaSelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ex_data_width(&self) -> ExDataWidthR {
        ExDataWidthR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_det_l(&self) -> CardDetLR {
        CardDetLR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_det_s(&self) -> CardDetSR {
        CardDetSR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sd_bus_power(&self) -> SdBusPowerR {
        SdBusPowerR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sd_bus_vlt(&self) -> SdBusVltR {
        SdBusVltR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_ctrl(&mut self) -> LedCtrlW<'_, SdHostCtrlSpec> {
        LedCtrlW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DataWidthW<'_, SdHostCtrlSpec> {
        DataWidthW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hi_speed_en(&mut self) -> HiSpeedEnW<'_, SdHostCtrlSpec> {
        HiSpeedEnW::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn dma_sel(&mut self) -> DmaSelW<'_, SdHostCtrlSpec> {
        DmaSelW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ex_data_width(&mut self) -> ExDataWidthW<'_, SdHostCtrlSpec> {
        ExDataWidthW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_det_l(&mut self) -> CardDetLW<'_, SdHostCtrlSpec> {
        CardDetLW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_det_s(&mut self) -> CardDetSW<'_, SdHostCtrlSpec> {
        CardDetSW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sd_bus_power(&mut self) -> SdBusPowerW<'_, SdHostCtrlSpec> {
        SdBusPowerW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sd_bus_vlt(&mut self) -> SdBusVltW<'_, SdHostCtrlSpec> {
        SdBusVltW::new(self, 9)
    }
}
#[doc = "Host Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_host_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_host_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdHostCtrlSpec;
impl crate::RegisterSpec for SdHostCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_host_ctrl::R`](R) reader structure"]
impl crate::Readable for SdHostCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_host_ctrl::W`](W) writer structure"]
impl crate::Writable for SdHostCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_HOST_CTRL to value 0"]
impl crate::Resettable for SdHostCtrlSpec {}
