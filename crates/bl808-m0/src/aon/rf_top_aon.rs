#[doc = "Register `RF_TOP_AON` reader"]
pub type R = crate::R<RfTopAonSpec>;
#[doc = "Register `RF_TOP_AON` writer"]
pub type W = crate::W<RfTopAonSpec>;
#[doc = "Field `pu_mbg_aon` reader - "]
pub type PuMbgAonR = crate::BitReader;
#[doc = "Field `pu_mbg_aon` writer - "]
pub type PuMbgAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_ldo15rf_aon` reader - "]
pub type PuLdo15rfAonR = crate::BitReader;
#[doc = "Field `pu_ldo15rf_aon` writer - "]
pub type PuLdo15rfAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_sfreg_aon` reader - "]
pub type PuSfregAonR = crate::BitReader;
#[doc = "Field `pu_sfreg_aon` writer - "]
pub type PuSfregAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal_buf_aon` reader - "]
pub type PuXtalBufAonR = crate::BitReader;
#[doc = "Field `pu_xtal_buf_aon` writer - "]
pub type PuXtalBufAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal_aon` reader - "]
pub type PuXtalAonR = crate::BitReader;
#[doc = "Field `pu_xtal_aon` writer - "]
pub type PuXtalAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_sstart_sel_aon` reader - "]
pub type Ldo15rfSstartSelAonR = crate::BitReader;
#[doc = "Field `ldo15rf_sstart_sel_aon` writer - "]
pub type Ldo15rfSstartSelAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_sstart_delay_aon` reader - "]
pub type Ldo15rfSstartDelayAonR = crate::FieldReader;
#[doc = "Field `ldo15rf_sstart_delay_aon` writer - "]
pub type Ldo15rfSstartDelayAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo15rf_pulldown_aon` reader - "]
pub type Ldo15rfPulldownAonR = crate::BitReader;
#[doc = "Field `ldo15rf_pulldown_aon` writer - "]
pub type Ldo15rfPulldownAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_pulldown_sel_aon` reader - "]
pub type Ldo15rfPulldownSelAonR = crate::BitReader;
#[doc = "Field `ldo15rf_pulldown_sel_aon` writer - "]
pub type Ldo15rfPulldownSelAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_vout_sel_aon` reader - "]
pub type Ldo15rfVoutSelAonR = crate::FieldReader;
#[doc = "Field `ldo15rf_vout_sel_aon` writer - "]
pub type Ldo15rfVoutSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo15rf_cc_aon` reader - "]
pub type Ldo15rfCcAonR = crate::FieldReader;
#[doc = "Field `ldo15rf_cc_aon` writer - "]
pub type Ldo15rfCcAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo15rf_bypass_aon` reader - "]
pub type Ldo15rfBypassAonR = crate::BitReader;
#[doc = "Field `ldo15rf_bypass_aon` writer - "]
pub type Ldo15rfBypassAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&self) -> PuMbgAonR {
        PuMbgAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&self) -> PuLdo15rfAonR {
        PuLdo15rfAonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&self) -> PuSfregAonR {
        PuSfregAonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&self) -> PuXtalBufAonR {
        PuXtalBufAonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&self) -> PuXtalAonR {
        PuXtalAonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&self) -> Ldo15rfSstartSelAonR {
        Ldo15rfSstartSelAonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&self) -> Ldo15rfSstartDelayAonR {
        Ldo15rfSstartDelayAonR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&self) -> Ldo15rfPulldownAonR {
        Ldo15rfPulldownAonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&self) -> Ldo15rfPulldownSelAonR {
        Ldo15rfPulldownSelAonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&self) -> Ldo15rfVoutSelAonR {
        Ldo15rfVoutSelAonR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&self) -> Ldo15rfCcAonR {
        Ldo15rfCcAonR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&self) -> Ldo15rfBypassAonR {
        Ldo15rfBypassAonR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&mut self) -> PuMbgAonW<'_, RfTopAonSpec> {
        PuMbgAonW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&mut self) -> PuLdo15rfAonW<'_, RfTopAonSpec> {
        PuLdo15rfAonW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&mut self) -> PuSfregAonW<'_, RfTopAonSpec> {
        PuSfregAonW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&mut self) -> PuXtalBufAonW<'_, RfTopAonSpec> {
        PuXtalBufAonW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&mut self) -> PuXtalAonW<'_, RfTopAonSpec> {
        PuXtalAonW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&mut self) -> Ldo15rfSstartSelAonW<'_, RfTopAonSpec> {
        Ldo15rfSstartSelAonW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&mut self) -> Ldo15rfSstartDelayAonW<'_, RfTopAonSpec> {
        Ldo15rfSstartDelayAonW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&mut self) -> Ldo15rfPulldownAonW<'_, RfTopAonSpec> {
        Ldo15rfPulldownAonW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&mut self) -> Ldo15rfPulldownSelAonW<'_, RfTopAonSpec> {
        Ldo15rfPulldownSelAonW::new(self, 13)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&mut self) -> Ldo15rfVoutSelAonW<'_, RfTopAonSpec> {
        Ldo15rfVoutSelAonW::new(self, 16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&mut self) -> Ldo15rfCcAonW<'_, RfTopAonSpec> {
        Ldo15rfCcAonW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&mut self) -> Ldo15rfBypassAonW<'_, RfTopAonSpec> {
        Ldo15rfBypassAonW::new(self, 28)
    }
}
#[doc = "RF LDO15 and XTAL Top Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_top_aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_top_aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfTopAonSpec;
impl crate::RegisterSpec for RfTopAonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_top_aon::R`](R) reader structure"]
impl crate::Readable for RfTopAonSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_top_aon::W`](W) writer structure"]
impl crate::Writable for RfTopAonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_TOP_AON to value 0"]
impl crate::Resettable for RfTopAonSpec {}
