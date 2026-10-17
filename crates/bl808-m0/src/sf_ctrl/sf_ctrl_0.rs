#[doc = "Register `SF_CTRL_0` reader"]
pub type R = crate::R<SfCtrl0Spec>;
#[doc = "Register `SF_CTRL_0` writer"]
pub type W = crate::W<SfCtrl0Spec>;
#[doc = "Field `sf_clk_sf_rx_inv_sel` reader - "]
pub type SfClkSfRxInvSelR = crate::BitReader;
#[doc = "Field `sf_clk_sf_rx_inv_sel` writer - "]
pub type SfClkSfRxInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_clk_out_gate_en` reader - "]
pub type SfClkOutGateEnR = crate::BitReader;
#[doc = "Field `sf_clk_out_gate_en` writer - "]
pub type SfClkOutGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_clk_out_inv_sel` reader - "]
pub type SfClkOutInvSelR = crate::BitReader;
#[doc = "Field `sf_clk_out_inv_sel` writer - "]
pub type SfClkOutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_read_dly_n` reader - "]
pub type SfIfReadDlyNR = crate::FieldReader;
#[doc = "Field `sf_if_read_dly_n` writer - "]
pub type SfIfReadDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_read_dly_en` reader - "]
pub type SfIfReadDlyEnR = crate::BitReader;
#[doc = "Field `sf_if_read_dly_en` writer - "]
pub type SfIfReadDlyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_int` reader - "]
pub type SfIfIntR = crate::BitReader;
#[doc = "Field `sf_if_int_clr` reader - "]
pub type SfIfIntClrR = crate::BitReader;
#[doc = "Field `sf_if_int_clr` writer - "]
pub type SfIfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_int_set` reader - "]
pub type SfIfIntSetR = crate::BitReader;
#[doc = "Field `sf_if_int_set` writer - "]
pub type SfIfIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_32b_adr_en` reader - "]
pub type SfIf32bAdrEnR = crate::BitReader;
#[doc = "Field `sf_if_32b_adr_en` writer - "]
pub type SfIf32bAdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_dout_endian` reader - "]
pub type SfAesDoutEndianR = crate::BitReader;
#[doc = "Field `sf_aes_dout_endian` writer - "]
pub type SfAesDoutEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_din_endian` reader - "]
pub type SfAesDinEndianR = crate::BitReader;
#[doc = "Field `sf_aes_din_endian` writer - "]
pub type SfAesDinEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_key_endian` reader - "]
pub type SfAesKeyEndianR = crate::BitReader;
#[doc = "Field `sf_aes_key_endian` writer - "]
pub type SfAesKeyEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_aes_iv_endian` reader - "]
pub type SfAesIvEndianR = crate::BitReader;
#[doc = "Field `sf_aes_iv_endian` writer - "]
pub type SfAesIvEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_id` reader - "]
pub type SfIdR = crate::FieldReader;
#[doc = "Field `sf_id` writer - "]
pub type SfIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&self) -> SfClkSfRxInvSelR {
        SfClkSfRxInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&self) -> SfClkOutGateEnR {
        SfClkOutGateEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&self) -> SfClkOutInvSelR {
        SfClkOutInvSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&self) -> SfIfReadDlyNR {
        SfIfReadDlyNR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&self) -> SfIfReadDlyEnR {
        SfIfReadDlyEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_int(&self) -> SfIfIntR {
        SfIfIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&self) -> SfIfIntClrR {
        SfIfIntClrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&self) -> SfIfIntSetR {
        SfIfIntSetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_if_32b_adr_en(&self) -> SfIf32bAdrEnR {
        SfIf32bAdrEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&self) -> SfAesDoutEndianR {
        SfAesDoutEndianR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_din_endian(&self) -> SfAesDinEndianR {
        SfAesDinEndianR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&self) -> SfAesKeyEndianR {
        SfAesKeyEndianR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&self) -> SfAesIvEndianR {
        SfAesIvEndianR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&self) -> SfIdR {
        SfIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&mut self) -> SfClkSfRxInvSelW<'_, SfCtrl0Spec> {
        SfClkSfRxInvSelW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&mut self) -> SfClkOutGateEnW<'_, SfCtrl0Spec> {
        SfClkOutGateEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&mut self) -> SfClkOutInvSelW<'_, SfCtrl0Spec> {
        SfClkOutInvSelW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&mut self) -> SfIfReadDlyNW<'_, SfCtrl0Spec> {
        SfIfReadDlyNW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&mut self) -> SfIfReadDlyEnW<'_, SfCtrl0Spec> {
        SfIfReadDlyEnW::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&mut self) -> SfIfIntClrW<'_, SfCtrl0Spec> {
        SfIfIntClrW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&mut self) -> SfIfIntSetW<'_, SfCtrl0Spec> {
        SfIfIntSetW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_if_32b_adr_en(&mut self) -> SfIf32bAdrEnW<'_, SfCtrl0Spec> {
        SfIf32bAdrEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&mut self) -> SfAesDoutEndianW<'_, SfCtrl0Spec> {
        SfAesDoutEndianW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_din_endian(&mut self) -> SfAesDinEndianW<'_, SfCtrl0Spec> {
        SfAesDinEndianW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&mut self) -> SfAesKeyEndianW<'_, SfCtrl0Spec> {
        SfAesKeyEndianW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&mut self) -> SfAesIvEndianW<'_, SfCtrl0Spec> {
        SfAesIvEndianW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&mut self) -> SfIdW<'_, SfCtrl0Spec> {
        SfIdW::new(self, 24)
    }
}
#[doc = "Serial Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrl0Spec;
impl crate::RegisterSpec for SfCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SfCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SfCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_CTRL_0 to value 0"]
impl crate::Resettable for SfCtrl0Spec {}
