#[doc = "Register `VDO_SRC_STA_CLR` writer"]
pub type W = crate::W<VdoSrcStaClrSpec>;
#[doc = "Field `src_uv_wovr_clr` writer - Clear source UV write overflow flag"]
pub type SrcUvWovrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `src_y_wovr_clr` writer - Clear source Y write overflow flag"]
pub type SrcYWovrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_src_uv_wovr_clr` writer - Clear secondary source UV write overflow flag"]
pub type SSrcUvWovrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_src_y_wovr_clr` writer - Clear secondary source Y write overflow flag"]
pub type SSrcYWovrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear source UV write overflow flag"]
    #[inline(always)]
    pub fn src_uv_wovr_clr(&mut self) -> SrcUvWovrClrW<'_, VdoSrcStaClrSpec> {
        SrcUvWovrClrW::new(self, 0)
    }
    #[doc = "Bit 4 - Clear source Y write overflow flag"]
    #[inline(always)]
    pub fn src_y_wovr_clr(&mut self) -> SrcYWovrClrW<'_, VdoSrcStaClrSpec> {
        SrcYWovrClrW::new(self, 4)
    }
    #[doc = "Bit 8 - Clear secondary source UV write overflow flag"]
    #[inline(always)]
    pub fn s_src_uv_wovr_clr(&mut self) -> SSrcUvWovrClrW<'_, VdoSrcStaClrSpec> {
        SSrcUvWovrClrW::new(self, 8)
    }
    #[doc = "Bit 12 - Clear secondary source Y write overflow flag"]
    #[inline(always)]
    pub fn s_src_y_wovr_clr(&mut self) -> SSrcYWovrClrW<'_, VdoSrcStaClrSpec> {
        SSrcYWovrClrW::new(self, 12)
    }
}
#[doc = "Source buffer overflow status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_src_sta_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoSrcStaClrSpec;
impl crate::RegisterSpec for VdoSrcStaClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`vdo_src_sta_clr::W`](W) writer structure"]
impl crate::Writable for VdoSrcStaClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_SRC_STA_CLR to value 0"]
impl crate::Resettable for VdoSrcStaClrSpec {}
