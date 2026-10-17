#[doc = "Register `MIPI_CONFIG` reader"]
pub type R = crate::R<MipiConfigSpec>;
#[doc = "Register `MIPI_CONFIG` writer"]
pub type W = crate::W<MipiConfigSpec>;
#[doc = "Field `cr_csi_en` reader - Master enable for MIPI CSI module"]
pub type CrCsiEnR = crate::BitReader;
#[doc = "Field `cr_csi_en` writer - Master enable for MIPI CSI module"]
pub type CrCsiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_lane_num` reader - Number of data lanes (0:1-lane, 1:2-lane)"]
pub type CrLaneNumR = crate::BitReader;
#[doc = "Field `cr_lane_num` writer - Number of data lanes (0:1-lane, 1:2-lane)"]
pub type CrLaneNumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_lane_inv` reader - Invert lane order"]
pub type CrLaneInvR = crate::BitReader;
#[doc = "Field `cr_lane_inv` writer - Invert lane order"]
pub type CrLaneInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_data_bit_inv` reader - Invert data bit order"]
pub type CrDataBitInvR = crate::BitReader;
#[doc = "Field `cr_data_bit_inv` writer - Invert data bit order"]
pub type CrDataBitInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_sync_sp_en` reader - Enable sync short packet mode"]
pub type CrSyncSpEnR = crate::BitReader;
#[doc = "Field `cr_sync_sp_en` writer - Enable sync short packet mode"]
pub type CrSyncSpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_unpack_en` reader - Unpack pixel data enable (e.g., for 10-bit packing)"]
pub type CrUnpackEnR = crate::BitReader;
#[doc = "Field `cr_unpack_en` writer - Unpack pixel data enable (e.g., for 10-bit packing)"]
pub type CrUnpackEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_vc_dvp0` reader - Virtual Channel (VC) selection for DVP0 output"]
pub type CrVcDvp0R = crate::FieldReader;
#[doc = "Field `cr_vc_dvp0` writer - Virtual Channel (VC) selection for DVP0 output"]
pub type CrVcDvp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_vc_dvp1` reader - Virtual Channel (VC) selection for DVP1 output"]
pub type CrVcDvp1R = crate::FieldReader;
#[doc = "Field `cr_vc_dvp1` writer - Virtual Channel (VC) selection for DVP1 output"]
pub type CrVcDvp1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Master enable for MIPI CSI module"]
    #[inline(always)]
    pub fn cr_csi_en(&self) -> CrCsiEnR {
        CrCsiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Number of data lanes (0:1-lane, 1:2-lane)"]
    #[inline(always)]
    pub fn cr_lane_num(&self) -> CrLaneNumR {
        CrLaneNumR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert lane order"]
    #[inline(always)]
    pub fn cr_lane_inv(&self) -> CrLaneInvR {
        CrLaneInvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invert data bit order"]
    #[inline(always)]
    pub fn cr_data_bit_inv(&self) -> CrDataBitInvR {
        CrDataBitInvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable sync short packet mode"]
    #[inline(always)]
    pub fn cr_sync_sp_en(&self) -> CrSyncSpEnR {
        CrSyncSpEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Unpack pixel data enable (e.g., for 10-bit packing)"]
    #[inline(always)]
    pub fn cr_unpack_en(&self) -> CrUnpackEnR {
        CrUnpackEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Virtual Channel (VC) selection for DVP0 output"]
    #[inline(always)]
    pub fn cr_vc_dvp0(&self) -> CrVcDvp0R {
        CrVcDvp0R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Virtual Channel (VC) selection for DVP1 output"]
    #[inline(always)]
    pub fn cr_vc_dvp1(&self) -> CrVcDvp1R {
        CrVcDvp1R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master enable for MIPI CSI module"]
    #[inline(always)]
    pub fn cr_csi_en(&mut self) -> CrCsiEnW<'_, MipiConfigSpec> {
        CrCsiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Number of data lanes (0:1-lane, 1:2-lane)"]
    #[inline(always)]
    pub fn cr_lane_num(&mut self) -> CrLaneNumW<'_, MipiConfigSpec> {
        CrLaneNumW::new(self, 1)
    }
    #[doc = "Bit 3 - Invert lane order"]
    #[inline(always)]
    pub fn cr_lane_inv(&mut self) -> CrLaneInvW<'_, MipiConfigSpec> {
        CrLaneInvW::new(self, 3)
    }
    #[doc = "Bit 4 - Invert data bit order"]
    #[inline(always)]
    pub fn cr_data_bit_inv(&mut self) -> CrDataBitInvW<'_, MipiConfigSpec> {
        CrDataBitInvW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable sync short packet mode"]
    #[inline(always)]
    pub fn cr_sync_sp_en(&mut self) -> CrSyncSpEnW<'_, MipiConfigSpec> {
        CrSyncSpEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Unpack pixel data enable (e.g., for 10-bit packing)"]
    #[inline(always)]
    pub fn cr_unpack_en(&mut self) -> CrUnpackEnW<'_, MipiConfigSpec> {
        CrUnpackEnW::new(self, 6)
    }
    #[doc = "Bits 12:13 - Virtual Channel (VC) selection for DVP0 output"]
    #[inline(always)]
    pub fn cr_vc_dvp0(&mut self) -> CrVcDvp0W<'_, MipiConfigSpec> {
        CrVcDvp0W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Virtual Channel (VC) selection for DVP1 output"]
    #[inline(always)]
    pub fn cr_vc_dvp1(&mut self) -> CrVcDvp1W<'_, MipiConfigSpec> {
        CrVcDvp1W::new(self, 14)
    }
}
#[doc = "MIPI CSI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiConfigSpec;
impl crate::RegisterSpec for MipiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_config::R`](R) reader structure"]
impl crate::Readable for MipiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_config::W`](W) writer structure"]
impl crate::Writable for MipiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_CONFIG to value 0x4042"]
impl crate::Resettable for MipiConfigSpec {
    const RESET_VALUE: u32 = 0x4042;
}
