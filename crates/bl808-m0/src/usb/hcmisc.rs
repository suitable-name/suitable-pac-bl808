#[doc = "Register `HCMISC` reader"]
pub type R = crate::R<HcmiscSpec>;
#[doc = "Register `HCMISC` writer"]
pub type W = crate::W<HcmiscSpec>;
#[doc = "Field `ASYN_SCH_SLPT` reader - Asynchronous Schedule Skip Threshold"]
pub type AsynSchSlptR = crate::FieldReader;
#[doc = "Field `ASYN_SCH_SLPT` writer - Asynchronous Schedule Skip Threshold"]
pub type AsynSchSlptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOF1_TIME` reader - End Of Frame 1 Time"]
pub type Eof1TimeR = crate::FieldReader;
#[doc = "Field `EOF1_TIME` writer - End Of Frame 1 Time"]
pub type Eof1TimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOF2_TIME` reader - End Of Frame 2 Time"]
pub type Eof2TimeR = crate::FieldReader;
#[doc = "Field `EOF2_TIME` writer - End Of Frame 2 Time"]
pub type Eof2TimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HOSTPHY_SUSPEND` reader - Host PHY Suspend"]
pub type HostphySuspendR = crate::BitReader;
#[doc = "Field `HOSTPHY_SUSPEND` writer - Host PHY Suspend"]
pub type HostphySuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HC_WKP_DET_EN` reader - Host Controller Wakeup Detect Enable"]
pub type HcWkpDetEnR = crate::BitReader;
#[doc = "Field `HC_WKP_DET_EN` writer - Host Controller Wakeup Detect Enable"]
pub type HcWkpDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HC_CONN_DET_EN` reader - Host Controller Connection Detect Enable"]
pub type HcConnDetEnR = crate::BitReader;
#[doc = "Field `HC_CONN_DET_EN` writer - Host Controller Connection Detect Enable"]
pub type HcConnDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Asynchronous Schedule Skip Threshold"]
    #[inline(always)]
    pub fn asyn_sch_slpt(&self) -> AsynSchSlptR {
        AsynSchSlptR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - End Of Frame 1 Time"]
    #[inline(always)]
    pub fn eof1_time(&self) -> Eof1TimeR {
        Eof1TimeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - End Of Frame 2 Time"]
    #[inline(always)]
    pub fn eof2_time(&self) -> Eof2TimeR {
        Eof2TimeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Host PHY Suspend"]
    #[inline(always)]
    pub fn hostphy_suspend(&self) -> HostphySuspendR {
        HostphySuspendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Host Controller Wakeup Detect Enable"]
    #[inline(always)]
    pub fn hc_wkp_det_en(&self) -> HcWkpDetEnR {
        HcWkpDetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host Controller Connection Detect Enable"]
    #[inline(always)]
    pub fn hc_conn_det_en(&self) -> HcConnDetEnR {
        HcConnDetEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Asynchronous Schedule Skip Threshold"]
    #[inline(always)]
    pub fn asyn_sch_slpt(&mut self) -> AsynSchSlptW<'_, HcmiscSpec> {
        AsynSchSlptW::new(self, 0)
    }
    #[doc = "Bits 2:3 - End Of Frame 1 Time"]
    #[inline(always)]
    pub fn eof1_time(&mut self) -> Eof1TimeW<'_, HcmiscSpec> {
        Eof1TimeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - End Of Frame 2 Time"]
    #[inline(always)]
    pub fn eof2_time(&mut self) -> Eof2TimeW<'_, HcmiscSpec> {
        Eof2TimeW::new(self, 4)
    }
    #[doc = "Bit 6 - Host PHY Suspend"]
    #[inline(always)]
    pub fn hostphy_suspend(&mut self) -> HostphySuspendW<'_, HcmiscSpec> {
        HostphySuspendW::new(self, 6)
    }
    #[doc = "Bit 8 - Host Controller Wakeup Detect Enable"]
    #[inline(always)]
    pub fn hc_wkp_det_en(&mut self) -> HcWkpDetEnW<'_, HcmiscSpec> {
        HcWkpDetEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Host Controller Connection Detect Enable"]
    #[inline(always)]
    pub fn hc_conn_det_en(&mut self) -> HcConnDetEnW<'_, HcmiscSpec> {
        HcConnDetEnW::new(self, 9)
    }
}
#[doc = "Host Controller Miscellaneous Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hcmisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcmisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcmiscSpec;
impl crate::RegisterSpec for HcmiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcmisc::R`](R) reader structure"]
impl crate::Readable for HcmiscSpec {}
#[doc = "`write(|w| ..)` method takes [`hcmisc::W`](W) writer structure"]
impl crate::Writable for HcmiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCMISC to value 0"]
impl crate::Resettable for HcmiscSpec {}
