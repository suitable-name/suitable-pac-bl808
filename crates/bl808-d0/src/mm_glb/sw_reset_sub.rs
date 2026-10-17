#[doc = "Register `SW_RESET_SUB` reader"]
pub type R = crate::R<SwResetSubSpec>;
#[doc = "Register `SW_RESET_SUB` writer"]
pub type W = crate::W<SwResetSubSpec>;
#[doc = "Field `swrst_misc` reader - "]
pub type SwrstMiscR = crate::BitReader;
#[doc = "Field `swrst_misc` writer - "]
pub type SwrstMiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_main` reader - "]
pub type SwrstMainR = crate::BitReader;
#[doc = "Field `swrst_main` writer - "]
pub type SwrstMainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_tsrc` reader - "]
pub type SwrstTsrcR = crate::BitReader;
#[doc = "Field `swrst_tsrc` writer - "]
pub type SwrstTsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dp_tsrc` reader - "]
pub type SwrstDpTsrcR = crate::BitReader;
#[doc = "Field `swrst_dp_tsrc` writer - "]
pub type SwrstDpTsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_nr3d_ctrl` reader - "]
pub type SwrstNr3dCtrlR = crate::BitReader;
#[doc = "Field `swrst_nr3d_ctrl` writer - "]
pub type SwrstNr3dCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busA` reader - "]
pub type SwrstDvp2busAR = crate::BitReader;
#[doc = "Field `swrst_dvp2busA` writer - "]
pub type SwrstDvp2busAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busB` reader - "]
pub type SwrstDvp2busBR = crate::BitReader;
#[doc = "Field `swrst_dvp2busB` writer - "]
pub type SwrstDvp2busBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busC` reader - "]
pub type SwrstDvp2busCR = crate::BitReader;
#[doc = "Field `swrst_dvp2busC` writer - "]
pub type SwrstDvp2busCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busD` reader - "]
pub type SwrstDvp2busDR = crate::BitReader;
#[doc = "Field `swrst_dvp2busD` writer - "]
pub type SwrstDvp2busDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_mipi` reader - "]
pub type SwrstMipiR = crate::BitReader;
#[doc = "Field `swrst_mipi` writer - "]
pub type SwrstMipiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_reg` reader - "]
pub type SwrstRegR = crate::BitReader;
#[doc = "Field `swrst_reg` writer - "]
pub type SwrstRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busE` reader - "]
pub type SwrstDvp2busER = crate::BitReader;
#[doc = "Field `swrst_dvp2busE` writer - "]
pub type SwrstDvp2busEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busF` reader - "]
pub type SwrstDvp2busFR = crate::BitReader;
#[doc = "Field `swrst_dvp2busF` writer - "]
pub type SwrstDvp2busFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busG` reader - "]
pub type SwrstDvp2busGR = crate::BitReader;
#[doc = "Field `swrst_dvp2busG` writer - "]
pub type SwrstDvp2busGW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dvp2busH` reader - "]
pub type SwrstDvp2busHR = crate::BitReader;
#[doc = "Field `swrst_dvp2busH` writer - "]
pub type SwrstDvp2busHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_misc(&self) -> SwrstMiscR {
        SwrstMiscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_main(&self) -> SwrstMainR {
        SwrstMainR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_tsrc(&self) -> SwrstTsrcR {
        SwrstTsrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_dp_tsrc(&self) -> SwrstDpTsrcR {
        SwrstDpTsrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_nr3d_ctrl(&self) -> SwrstNr3dCtrlR {
        SwrstNr3dCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_dvp2bus_a(&self) -> SwrstDvp2busAR {
        SwrstDvp2busAR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_dvp2bus_b(&self) -> SwrstDvp2busBR {
        SwrstDvp2busBR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_dvp2bus_c(&self) -> SwrstDvp2busCR {
        SwrstDvp2busCR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_dvp2bus_d(&self) -> SwrstDvp2busDR {
        SwrstDvp2busDR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_mipi(&self) -> SwrstMipiR {
        SwrstMipiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_reg(&self) -> SwrstRegR {
        SwrstRegR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_dvp2bus_e(&self) -> SwrstDvp2busER {
        SwrstDvp2busER::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_dvp2bus_f(&self) -> SwrstDvp2busFR {
        SwrstDvp2busFR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_dvp2bus_g(&self) -> SwrstDvp2busGR {
        SwrstDvp2busGR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_dvp2bus_h(&self) -> SwrstDvp2busHR {
        SwrstDvp2busHR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_misc(&mut self) -> SwrstMiscW<'_, SwResetSubSpec> {
        SwrstMiscW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_main(&mut self) -> SwrstMainW<'_, SwResetSubSpec> {
        SwrstMainW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_tsrc(&mut self) -> SwrstTsrcW<'_, SwResetSubSpec> {
        SwrstTsrcW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_dp_tsrc(&mut self) -> SwrstDpTsrcW<'_, SwResetSubSpec> {
        SwrstDpTsrcW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_nr3d_ctrl(&mut self) -> SwrstNr3dCtrlW<'_, SwResetSubSpec> {
        SwrstNr3dCtrlW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_dvp2bus_a(&mut self) -> SwrstDvp2busAW<'_, SwResetSubSpec> {
        SwrstDvp2busAW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_dvp2bus_b(&mut self) -> SwrstDvp2busBW<'_, SwResetSubSpec> {
        SwrstDvp2busBW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_dvp2bus_c(&mut self) -> SwrstDvp2busCW<'_, SwResetSubSpec> {
        SwrstDvp2busCW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_dvp2bus_d(&mut self) -> SwrstDvp2busDW<'_, SwResetSubSpec> {
        SwrstDvp2busDW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_mipi(&mut self) -> SwrstMipiW<'_, SwResetSubSpec> {
        SwrstMipiW::new(self, 9)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_reg(&mut self) -> SwrstRegW<'_, SwResetSubSpec> {
        SwrstRegW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_dvp2bus_e(&mut self) -> SwrstDvp2busEW<'_, SwResetSubSpec> {
        SwrstDvp2busEW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_dvp2bus_f(&mut self) -> SwrstDvp2busFW<'_, SwResetSubSpec> {
        SwrstDvp2busFW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_dvp2bus_g(&mut self) -> SwrstDvp2busGW<'_, SwResetSubSpec> {
        SwrstDvp2busGW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_dvp2bus_h(&mut self) -> SwrstDvp2busHW<'_, SwResetSubSpec> {
        SwrstDvp2busHW::new(self, 20)
    }
}
#[doc = "MM Subsystem Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset_sub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset_sub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResetSubSpec;
impl crate::RegisterSpec for SwResetSubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_reset_sub::R`](R) reader structure"]
impl crate::Readable for SwResetSubSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_reset_sub::W`](W) writer structure"]
impl crate::Writable for SwResetSubSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_RESET_SUB to value 0"]
impl crate::Resettable for SwResetSubSpec {}
