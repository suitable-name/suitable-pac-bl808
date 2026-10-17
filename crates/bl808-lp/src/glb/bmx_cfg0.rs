#[doc = "Register `BMX_CFG0` reader"]
pub type R = crate::R<BmxCfg0Spec>;
#[doc = "Register `BMX_CFG0` writer"]
pub type W = crate::W<BmxCfg0Spec>;
#[doc = "Field `reg_bmx_timeout_en` reader - "]
pub type RegBmxTimeoutEnR = crate::FieldReader;
#[doc = "Field `reg_bmx_timeout_en` writer - "]
pub type RegBmxTimeoutEnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_bmx_arb_mode` reader - "]
pub type RegBmxArbModeR = crate::BitReader;
#[doc = "Field `reg_bmx_arb_mode` writer - "]
pub type RegBmxArbModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_timeout_clr` reader - "]
pub type RegBmxTimeoutClrR = crate::BitReader;
#[doc = "Field `reg_bmx_timeout_clr` writer - "]
pub type RegBmxTimeoutClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_h_wthre_hw2ext` reader - "]
pub type RegHWthreHw2extR = crate::FieldReader;
#[doc = "Field `reg_h_wthre_hw2ext` writer - "]
pub type RegHWthreHw2extW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bmx_busy_option_dis` reader - "]
pub type BmxBusyOptionDisR = crate::BitReader;
#[doc = "Field `bmx_busy_option_dis` writer - "]
pub type BmxBusyOptionDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bmx_gating_dis` reader - "]
pub type BmxGatingDisR = crate::BitReader;
#[doc = "Field `bmx_gating_dis` writer - "]
pub type BmxGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bmx_timeout_sts` reader - "]
pub type StsBmxTimeoutStsR = crate::FieldReader;
#[doc = "Field `pds_apb_cfg` reader - "]
pub type PdsApbCfgR = crate::FieldReader;
#[doc = "Field `pds_apb_cfg` writer - "]
pub type PdsApbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hbn_apb_cfg` reader - "]
pub type HbnApbCfgR = crate::FieldReader;
#[doc = "Field `hbn_apb_cfg` writer - "]
pub type HbnApbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_bmx_timeout_en(&self) -> RegBmxTimeoutEnR {
        RegBmxTimeoutEnR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_bmx_arb_mode(&self) -> RegBmxArbModeR {
        RegBmxArbModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_bmx_timeout_clr(&self) -> RegBmxTimeoutClrR {
        RegBmxTimeoutClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn reg_h_wthre_hw2ext(&self) -> RegHWthreHw2extR {
        RegHWthreHw2extR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&self) -> BmxBusyOptionDisR {
        BmxBusyOptionDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&self) -> BmxGatingDisR {
        BmxGatingDisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn sts_bmx_timeout_sts(&self) -> StsBmxTimeoutStsR {
        StsBmxTimeoutStsR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PdsApbCfgR {
        PdsApbCfgR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HbnApbCfgR {
        HbnApbCfgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_bmx_timeout_en(&mut self) -> RegBmxTimeoutEnW<'_, BmxCfg0Spec> {
        RegBmxTimeoutEnW::new(self, 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_bmx_arb_mode(&mut self) -> RegBmxArbModeW<'_, BmxCfg0Spec> {
        RegBmxArbModeW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_bmx_timeout_clr(&mut self) -> RegBmxTimeoutClrW<'_, BmxCfg0Spec> {
        RegBmxTimeoutClrW::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn reg_h_wthre_hw2ext(&mut self) -> RegHWthreHw2extW<'_, BmxCfg0Spec> {
        RegHWthreHw2extW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&mut self) -> BmxBusyOptionDisW<'_, BmxCfg0Spec> {
        BmxBusyOptionDisW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&mut self) -> BmxGatingDisW<'_, BmxCfg0Spec> {
        BmxGatingDisW::new(self, 10)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&mut self) -> PdsApbCfgW<'_, BmxCfg0Spec> {
        PdsApbCfgW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&mut self) -> HbnApbCfgW<'_, BmxCfg0Spec> {
        HbnApbCfgW::new(self, 24)
    }
}
#[doc = "BMX Bus Matrix Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg0Spec;
impl crate::RegisterSpec for BmxCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg0::R`](R) reader structure"]
impl crate::Readable for BmxCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg0::W`](W) writer structure"]
impl crate::Writable for BmxCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMX_CFG0 to value 0"]
impl crate::Resettable for BmxCfg0Spec {}
