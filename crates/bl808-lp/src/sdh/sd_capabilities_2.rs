#[doc = "Register `SD_CAPABILITIES_2` reader"]
pub type R = crate::R<SdCapabilities2Spec>;
#[doc = "Register `SD_CAPABILITIES_2` writer"]
pub type W = crate::W<SdCapabilities2Spec>;
#[doc = "Field `MAX_BLK_LEN` reader - "]
pub type MaxBlkLenR = crate::FieldReader;
#[doc = "Field `EX_DATA_WIDTH_SUPPORT` reader - "]
pub type ExDataWidthSupportR = crate::BitReader;
#[doc = "Field `ADMA2_SUPPORT` reader - "]
pub type Adma2SupportR = crate::BitReader;
#[doc = "Field `ADMA1_SUPPORT` reader - "]
pub type Adma1SupportR = crate::BitReader;
#[doc = "Field `HI_SPEED_SUPPORT` reader - "]
pub type HiSpeedSupportR = crate::BitReader;
#[doc = "Field `SDMA_SUPPORT` reader - "]
pub type SdmaSupportR = crate::BitReader;
#[doc = "Field `SUS_RES_SUPPORT` reader - "]
pub type SusResSupportR = crate::BitReader;
#[doc = "Field `VLG_33_SUPPORT` reader - "]
pub type Vlg33SupportR = crate::BitReader;
#[doc = "Field `VLG_30_SUPPORT` reader - "]
pub type Vlg30SupportR = crate::BitReader;
#[doc = "Field `VLG_18_SUPPORT` reader - "]
pub type Vlg18SupportR = crate::BitReader;
#[doc = "Field `SYS_BUS_64_SUPPORT` reader - "]
pub type SysBus64SupportR = crate::BitReader;
#[doc = "Field `ASYNC_INT_SUPPORT` reader - "]
pub type AsyncIntSupportR = crate::BitReader;
#[doc = "Field `CFG_SLOT_TYPE` reader - "]
pub type CfgSlotTypeR = crate::FieldReader;
#[doc = "Field `CFG_SLOT_TYPE` writer - "]
pub type CfgSlotTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn max_blk_len(&self) -> MaxBlkLenR {
        MaxBlkLenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ex_data_width_support(&self) -> ExDataWidthSupportR {
        ExDataWidthSupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adma2_support(&self) -> Adma2SupportR {
        Adma2SupportR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adma1_support(&self) -> Adma1SupportR {
        Adma1SupportR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hi_speed_support(&self) -> HiSpeedSupportR {
        HiSpeedSupportR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdma_support(&self) -> SdmaSupportR {
        SdmaSupportR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sus_res_support(&self) -> SusResSupportR {
        SusResSupportR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vlg_33_support(&self) -> Vlg33SupportR {
        Vlg33SupportR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vlg_30_support(&self) -> Vlg30SupportR {
        Vlg30SupportR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vlg_18_support(&self) -> Vlg18SupportR {
        Vlg18SupportR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sys_bus_64_support(&self) -> SysBus64SupportR {
        SysBus64SupportR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn async_int_support(&self) -> AsyncIntSupportR {
        AsyncIntSupportR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cfg_slot_type(&self) -> CfgSlotTypeR {
        CfgSlotTypeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cfg_slot_type(&mut self) -> CfgSlotTypeW<'_, SdCapabilities2Spec> {
        CfgSlotTypeW::new(self, 14)
    }
}
#[doc = "Host Controller Capabilities Register 2 (Voltage/DMA)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_capabilities_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCapabilities2Spec;
impl crate::RegisterSpec for SdCapabilities2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_capabilities_2::R`](R) reader structure"]
impl crate::Readable for SdCapabilities2Spec {}
#[doc = "`write(|w| ..)` method takes [`sd_capabilities_2::W`](W) writer structure"]
impl crate::Writable for SdCapabilities2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_CAPABILITIES_2 to value 0"]
impl crate::Resettable for SdCapabilities2Spec {}
