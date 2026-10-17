#[doc = "Register `SF_IF_IAHB_12` reader"]
pub type R = crate::R<SfIfIahb12Spec>;
#[doc = "Register `SF_IF_IAHB_12` writer"]
pub type W = crate::W<SfIfIahb12Spec>;
#[doc = "Field `sf2_clk_sf_rx_inv_sel` reader - "]
pub type Sf2ClkSfRxInvSelR = crate::BitReader;
#[doc = "Field `sf2_clk_sf_rx_inv_sel` writer - "]
pub type Sf2ClkSfRxInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf2_clk_sf_rx_inv_src` reader - "]
pub type Sf2ClkSfRxInvSrcR = crate::BitReader;
#[doc = "Field `sf2_clk_sf_rx_inv_src` writer - "]
pub type Sf2ClkSfRxInvSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf2_clk_out_inv_sel` reader - "]
pub type Sf2ClkOutInvSelR = crate::BitReader;
#[doc = "Field `sf2_clk_out_inv_sel` writer - "]
pub type Sf2ClkOutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf3_clk_out_inv_sel` reader - "]
pub type Sf3ClkOutInvSelR = crate::BitReader;
#[doc = "Field `sf3_clk_out_inv_sel` writer - "]
pub type Sf3ClkOutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf2_if_read_dly_n` reader - "]
pub type Sf2IfReadDlyNR = crate::FieldReader;
#[doc = "Field `sf2_if_read_dly_n` writer - "]
pub type Sf2IfReadDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf2_if_read_dly_en` reader - "]
pub type Sf2IfReadDlyEnR = crate::BitReader;
#[doc = "Field `sf2_if_read_dly_en` writer - "]
pub type Sf2IfReadDlyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf2_if_read_dly_src` reader - "]
pub type Sf2IfReadDlySrcR = crate::BitReader;
#[doc = "Field `sf2_if_read_dly_src` writer - "]
pub type Sf2IfReadDlySrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_sel(&self) -> Sf2ClkSfRxInvSelR {
        Sf2ClkSfRxInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_src(&self) -> Sf2ClkSfRxInvSrcR {
        Sf2ClkSfRxInvSrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf2_clk_out_inv_sel(&self) -> Sf2ClkOutInvSelR {
        Sf2ClkOutInvSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf3_clk_out_inv_sel(&self) -> Sf3ClkOutInvSelR {
        Sf3ClkOutInvSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf2_if_read_dly_n(&self) -> Sf2IfReadDlyNR {
        Sf2IfReadDlyNR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf2_if_read_dly_en(&self) -> Sf2IfReadDlyEnR {
        Sf2IfReadDlyEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sf2_if_read_dly_src(&self) -> Sf2IfReadDlySrcR {
        Sf2IfReadDlySrcR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_sel(&mut self) -> Sf2ClkSfRxInvSelW<'_, SfIfIahb12Spec> {
        Sf2ClkSfRxInvSelW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_src(&mut self) -> Sf2ClkSfRxInvSrcW<'_, SfIfIahb12Spec> {
        Sf2ClkSfRxInvSrcW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf2_clk_out_inv_sel(&mut self) -> Sf2ClkOutInvSelW<'_, SfIfIahb12Spec> {
        Sf2ClkOutInvSelW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf3_clk_out_inv_sel(&mut self) -> Sf3ClkOutInvSelW<'_, SfIfIahb12Spec> {
        Sf3ClkOutInvSelW::new(self, 5)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf2_if_read_dly_n(&mut self) -> Sf2IfReadDlyNW<'_, SfIfIahb12Spec> {
        Sf2IfReadDlyNW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf2_if_read_dly_en(&mut self) -> Sf2IfReadDlyEnW<'_, SfIfIahb12Spec> {
        Sf2IfReadDlyEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sf2_if_read_dly_src(&mut self) -> Sf2IfReadDlySrcW<'_, SfIfIahb12Spec> {
        Sf2IfReadDlySrcW::new(self, 12)
    }
}
#[doc = "SF2/SF3 Clock and Read Delay Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb12Spec;
impl crate::RegisterSpec for SfIfIahb12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_12::R`](R) reader structure"]
impl crate::Readable for SfIfIahb12Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_12::W`](W) writer structure"]
impl crate::Writable for SfIfIahb12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_12 to value 0"]
impl crate::Resettable for SfIfIahb12Spec {}
