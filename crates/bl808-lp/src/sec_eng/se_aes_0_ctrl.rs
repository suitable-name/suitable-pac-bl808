#[doc = "Register `SE_AES_0_CTRL` reader"]
pub type R = crate::R<SeAes0CtrlSpec>;
#[doc = "Register `SE_AES_0_CTRL` writer"]
pub type W = crate::W<SeAes0CtrlSpec>;
#[doc = "Field `se_aes_0_busy` reader - "]
pub type SeAes0BusyR = crate::BitReader;
#[doc = "Field `se_aes_0_trig_1t` writer - "]
pub type SeAes0Trig1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_en` reader - "]
pub type SeAes0EnR = crate::BitReader;
#[doc = "Field `se_aes_0_en` writer - "]
pub type SeAes0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_mode` reader - 0:128-bit, 1:256-bit, 2:192-bit"]
pub type SeAes0ModeR = crate::FieldReader;
#[doc = "Field `se_aes_0_mode` writer - 0:128-bit, 1:256-bit, 2:192-bit"]
pub type SeAes0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `se_aes_0_dec_en` reader - 0:Encrypt, 1:Decrypt"]
pub type SeAes0DecEnR = crate::BitReader;
#[doc = "Field `se_aes_0_dec_en` writer - 0:Encrypt, 1:Decrypt"]
pub type SeAes0DecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_dec_key_sel` reader - "]
pub type SeAes0DecKeySelR = crate::BitReader;
#[doc = "Field `se_aes_0_dec_key_sel` writer - "]
pub type SeAes0DecKeySelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_hw_key_en` reader - "]
pub type SeAes0HwKeyEnR = crate::BitReader;
#[doc = "Field `se_aes_0_hw_key_en` writer - "]
pub type SeAes0HwKeyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_int` reader - "]
pub type SeAes0IntR = crate::BitReader;
#[doc = "Field `se_aes_0_int_clr_1t` writer - "]
pub type SeAes0IntClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_int_set_1t` writer - "]
pub type SeAes0IntSet1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_int_mask` reader - "]
pub type SeAes0IntMaskR = crate::BitReader;
#[doc = "Field `se_aes_0_int_mask` writer - "]
pub type SeAes0IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_block_mode` reader - 0:ECB, 1:CTR, 2:CBC, 3:XTS"]
pub type SeAes0BlockModeR = crate::FieldReader;
#[doc = "Field `se_aes_0_block_mode` writer - 0:ECB, 1:CTR, 2:CBC, 3:XTS"]
pub type SeAes0BlockModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `se_aes_0_iv_sel` reader - "]
pub type SeAes0IvSelR = crate::BitReader;
#[doc = "Field `se_aes_0_iv_sel` writer - "]
pub type SeAes0IvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_link_mode` reader - "]
pub type SeAes0LinkModeR = crate::BitReader;
#[doc = "Field `se_aes_0_link_mode` writer - "]
pub type SeAes0LinkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_0_msg_len` reader - "]
pub type SeAes0MsgLenR = crate::FieldReader<u16>;
#[doc = "Field `se_aes_0_msg_len` writer - "]
pub type SeAes0MsgLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_busy(&self) -> SeAes0BusyR {
        SeAes0BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_en(&self) -> SeAes0EnR {
        SeAes0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0:128-bit, 1:256-bit, 2:192-bit"]
    #[inline(always)]
    pub fn se_aes_0_mode(&self) -> SeAes0ModeR {
        SeAes0ModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 0:Encrypt, 1:Decrypt"]
    #[inline(always)]
    pub fn se_aes_0_dec_en(&self) -> SeAes0DecEnR {
        SeAes0DecEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_0_dec_key_sel(&self) -> SeAes0DecKeySelR {
        SeAes0DecKeySelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn se_aes_0_hw_key_en(&self) -> SeAes0HwKeyEnR {
        SeAes0HwKeyEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_aes_0_int(&self) -> SeAes0IntR {
        SeAes0IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_aes_0_int_mask(&self) -> SeAes0IntMaskR {
        SeAes0IntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 0:ECB, 1:CTR, 2:CBC, 3:XTS"]
    #[inline(always)]
    pub fn se_aes_0_block_mode(&self) -> SeAes0BlockModeR {
        SeAes0BlockModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_aes_0_iv_sel(&self) -> SeAes0IvSelR {
        SeAes0IvSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_link_mode(&self) -> SeAes0LinkModeR {
        SeAes0LinkModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_msg_len(&self) -> SeAes0MsgLenR {
        SeAes0MsgLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_trig_1t(&mut self) -> SeAes0Trig1tW<'_, SeAes0CtrlSpec> {
        SeAes0Trig1tW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_en(&mut self) -> SeAes0EnW<'_, SeAes0CtrlSpec> {
        SeAes0EnW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 0:128-bit, 1:256-bit, 2:192-bit"]
    #[inline(always)]
    pub fn se_aes_0_mode(&mut self) -> SeAes0ModeW<'_, SeAes0CtrlSpec> {
        SeAes0ModeW::new(self, 3)
    }
    #[doc = "Bit 5 - 0:Encrypt, 1:Decrypt"]
    #[inline(always)]
    pub fn se_aes_0_dec_en(&mut self) -> SeAes0DecEnW<'_, SeAes0CtrlSpec> {
        SeAes0DecEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_0_dec_key_sel(&mut self) -> SeAes0DecKeySelW<'_, SeAes0CtrlSpec> {
        SeAes0DecKeySelW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn se_aes_0_hw_key_en(&mut self) -> SeAes0HwKeyEnW<'_, SeAes0CtrlSpec> {
        SeAes0HwKeyEnW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_aes_0_int_clr_1t(&mut self) -> SeAes0IntClr1tW<'_, SeAes0CtrlSpec> {
        SeAes0IntClr1tW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_aes_0_int_set_1t(&mut self) -> SeAes0IntSet1tW<'_, SeAes0CtrlSpec> {
        SeAes0IntSet1tW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_aes_0_int_mask(&mut self) -> SeAes0IntMaskW<'_, SeAes0CtrlSpec> {
        SeAes0IntMaskW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 0:ECB, 1:CTR, 2:CBC, 3:XTS"]
    #[inline(always)]
    pub fn se_aes_0_block_mode(&mut self) -> SeAes0BlockModeW<'_, SeAes0CtrlSpec> {
        SeAes0BlockModeW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_aes_0_iv_sel(&mut self) -> SeAes0IvSelW<'_, SeAes0CtrlSpec> {
        SeAes0IvSelW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_link_mode(&mut self) -> SeAes0LinkModeW<'_, SeAes0CtrlSpec> {
        SeAes0LinkModeW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_msg_len(&mut self) -> SeAes0MsgLenW<'_, SeAes0CtrlSpec> {
        SeAes0MsgLenW::new(self, 16)
    }
}
#[doc = "AES Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0CtrlSpec;
impl crate::RegisterSpec for SeAes0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_ctrl::R`](R) reader structure"]
impl crate::Readable for SeAes0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_ctrl::W`](W) writer structure"]
impl crate::Writable for SeAes0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_CTRL to value 0"]
impl crate::Resettable for SeAes0CtrlSpec {}
