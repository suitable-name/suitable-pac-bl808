#[doc = "Register `AON_COMMON` reader"]
pub type R = crate::R<AonCommonSpec>;
#[doc = "Register `AON_COMMON` writer"]
pub type W = crate::W<AonCommonSpec>;
#[doc = "Field `tmux_aon` reader - "]
pub type TmuxAonR = crate::FieldReader;
#[doc = "Field `tmux_aon` writer - "]
pub type TmuxAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pmip_dc_tp_out_en_aon` reader - "]
pub type PmipDcTpOutEnAonR = crate::BitReader;
#[doc = "Field `pmip_dc_tp_out_en_aon` writer - "]
pub type PmipDcTpOutEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_bg_sys_aon` reader - "]
pub type TenBgSysAonR = crate::BitReader;
#[doc = "Field `ten_bg_sys_aon` writer - "]
pub type TenBgSysAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc11_0_aon` reader - "]
pub type TenDcdc11_0AonR = crate::BitReader;
#[doc = "Field `ten_dcdc11_0_aon` writer - "]
pub type TenDcdc11_0AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc11_1_aon` reader - "]
pub type TenDcdc11_1AonR = crate::BitReader;
#[doc = "Field `ten_dcdc11_1_aon` writer - "]
pub type TenDcdc11_1AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc18_0_aon` reader - "]
pub type TenDcdc18_0AonR = crate::BitReader;
#[doc = "Field `ten_dcdc18_0_aon` writer - "]
pub type TenDcdc18_0AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc18_1_aon` reader - "]
pub type TenDcdc18_1AonR = crate::BitReader;
#[doc = "Field `ten_dcdc18_1_aon` writer - "]
pub type TenDcdc18_1AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo12uhs` reader - "]
pub type TenLdo12uhsR = crate::BitReader;
#[doc = "Field `ten_ldo12uhs` writer - "]
pub type TenLdo12uhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo18flash` reader - "]
pub type TenLdo18flashR = crate::BitReader;
#[doc = "Field `ten_ldo18flash` writer - "]
pub type TenLdo18flashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo15cis` reader - "]
pub type TenLdo15cisR = crate::BitReader;
#[doc = "Field `ten_ldo15cis` writer - "]
pub type TenLdo15cisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo18io_aon` reader - "]
pub type TenLdo18ioAonR = crate::BitReader;
#[doc = "Field `ten_ldo18io_aon` writer - "]
pub type TenLdo18ioAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo28cis` reader - "]
pub type TenLdo28cisR = crate::BitReader;
#[doc = "Field `ten_ldo28cis` writer - "]
pub type TenLdo28cisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_rc32m` reader - "]
pub type TenRc32mR = crate::BitReader;
#[doc = "Field `ten_rc32m` writer - "]
pub type TenRc32mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo15rf_aon` reader - "]
pub type TenLdo15rfAonR = crate::BitReader;
#[doc = "Field `ten_ldo15rf_aon` writer - "]
pub type TenLdo15rfAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_xtal_aon` reader - "]
pub type TenXtalAonR = crate::BitReader;
#[doc = "Field `ten_xtal_aon` writer - "]
pub type TenXtalAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_xtal_aon` reader - "]
pub type DtenXtalAonR = crate::BitReader;
#[doc = "Field `dten_xtal_aon` writer - "]
pub type DtenXtalAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_mbg_aon` reader - "]
pub type TenMbgAonR = crate::BitReader;
#[doc = "Field `ten_mbg_aon` writer - "]
pub type TenMbgAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_cip_misc_aon` reader - "]
pub type TenCipMiscAonR = crate::BitReader;
#[doc = "Field `ten_cip_misc_aon` writer - "]
pub type TenCipMiscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_aon` reader - "]
pub type TenAonR = crate::BitReader;
#[doc = "Field `ten_aon` writer - "]
pub type TenAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TmuxAonR {
        TmuxAonR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PmipDcTpOutEnAonR {
        PmipDcTpOutEnAonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TenBgSysAonR {
        TenBgSysAonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ten_dcdc11_0_aon(&self) -> TenDcdc11_0AonR {
        TenDcdc11_0AonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_dcdc11_1_aon(&self) -> TenDcdc11_1AonR {
        TenDcdc11_1AonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&self) -> TenDcdc18_0AonR {
        TenDcdc18_0AonR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&self) -> TenDcdc18_1AonR {
        TenDcdc18_1AonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo12uhs(&self) -> TenLdo12uhsR {
        TenLdo12uhsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_ldo18flash(&self) -> TenLdo18flashR {
        TenLdo18flashR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_ldo15cis(&self) -> TenLdo15cisR {
        TenLdo15cisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_ldo18io_aon(&self) -> TenLdo18ioAonR {
        TenLdo18ioAonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_ldo28cis(&self) -> TenLdo28cisR {
        TenLdo28cisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_rc32m(&self) -> TenRc32mR {
        TenRc32mR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TenLdo15rfAonR {
        TenLdo15rfAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TenXtalAonR {
        TenXtalAonR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DtenXtalAonR {
        DtenXtalAonR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TenMbgAonR {
        TenMbgAonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TenCipMiscAonR {
        TenCipMiscAonR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TenAonR {
        TenAonR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&mut self) -> TmuxAonW<'_, AonCommonSpec> {
        TmuxAonW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PmipDcTpOutEnAonW<'_, AonCommonSpec> {
        PmipDcTpOutEnAonW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&mut self) -> TenBgSysAonW<'_, AonCommonSpec> {
        TenBgSysAonW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ten_dcdc11_0_aon(&mut self) -> TenDcdc11_0AonW<'_, AonCommonSpec> {
        TenDcdc11_0AonW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_dcdc11_1_aon(&mut self) -> TenDcdc11_1AonW<'_, AonCommonSpec> {
        TenDcdc11_1AonW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&mut self) -> TenDcdc18_0AonW<'_, AonCommonSpec> {
        TenDcdc18_0AonW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&mut self) -> TenDcdc18_1AonW<'_, AonCommonSpec> {
        TenDcdc18_1AonW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo12uhs(&mut self) -> TenLdo12uhsW<'_, AonCommonSpec> {
        TenLdo12uhsW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_ldo18flash(&mut self) -> TenLdo18flashW<'_, AonCommonSpec> {
        TenLdo18flashW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_ldo15cis(&mut self) -> TenLdo15cisW<'_, AonCommonSpec> {
        TenLdo15cisW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_ldo18io_aon(&mut self) -> TenLdo18ioAonW<'_, AonCommonSpec> {
        TenLdo18ioAonW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_ldo28cis(&mut self) -> TenLdo28cisW<'_, AonCommonSpec> {
        TenLdo28cisW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_rc32m(&mut self) -> TenRc32mW<'_, AonCommonSpec> {
        TenRc32mW::new(self, 14)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&mut self) -> TenLdo15rfAonW<'_, AonCommonSpec> {
        TenLdo15rfAonW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&mut self) -> TenXtalAonW<'_, AonCommonSpec> {
        TenXtalAonW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&mut self) -> DtenXtalAonW<'_, AonCommonSpec> {
        DtenXtalAonW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&mut self) -> TenMbgAonW<'_, AonCommonSpec> {
        TenMbgAonW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&mut self) -> TenCipMiscAonW<'_, AonCommonSpec> {
        TenCipMiscAonW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_aon(&mut self) -> TenAonW<'_, AonCommonSpec> {
        TenAonW::new(self, 21)
    }
}
#[doc = "AON Common Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_common::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_common::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonCommonSpec;
impl crate::RegisterSpec for AonCommonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_common::R`](R) reader structure"]
impl crate::Readable for AonCommonSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_common::W`](W) writer structure"]
impl crate::Writable for AonCommonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_COMMON to value 0"]
impl crate::Resettable for AonCommonSpec {}
