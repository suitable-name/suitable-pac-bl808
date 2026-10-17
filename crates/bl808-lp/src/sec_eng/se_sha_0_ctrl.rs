#[doc = "Register `SE_SHA_0_CTRL` reader"]
pub type R = crate::R<SeSha0CtrlSpec>;
#[doc = "Register `SE_SHA_0_CTRL` writer"]
pub type W = crate::W<SeSha0CtrlSpec>;
#[doc = "Field `se_sha_0_busy` reader - SHA engine busy status"]
pub type SeSha0BusyR = crate::BitReader;
#[doc = "Field `se_sha_0_trig_1t` writer - Trigger SHA calculation (Pulse)"]
pub type SeSha0Trig1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_mode` reader - SHA mode: 0:SHA-256, 1:SHA-224, 2:SHA-1, 4:SHA-512, 5:SHA-384"]
pub type SeSha0ModeR = crate::FieldReader;
#[doc = "Field `se_sha_0_mode` writer - SHA mode: 0:SHA-256, 1:SHA-224, 2:SHA-1, 4:SHA-512, 5:SHA-384"]
pub type SeSha0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `se_sha_0_en` reader - Enable SHA engine"]
pub type SeSha0EnR = crate::BitReader;
#[doc = "Field `se_sha_0_en` writer - Enable SHA engine"]
pub type SeSha0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_hash_sel` reader - 0:New hash, 1:Accumulate from previous result"]
pub type SeSha0HashSelR = crate::BitReader;
#[doc = "Field `se_sha_0_hash_sel` writer - 0:New hash, 1:Accumulate from previous result"]
pub type SeSha0HashSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_int` reader - SHA interrupt status"]
pub type SeSha0IntR = crate::BitReader;
#[doc = "Field `se_sha_0_int_clr_1t` writer - Clear SHA interrupt (Pulse)"]
pub type SeSha0IntClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_int_set_1t` writer - Set SHA interrupt for debug (Pulse)"]
pub type SeSha0IntSet1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_int_mask` reader - Mask SHA interrupt"]
pub type SeSha0IntMaskR = crate::BitReader;
#[doc = "Field `se_sha_0_int_mask` writer - Mask SHA interrupt"]
pub type SeSha0IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_mode_ext` reader - Extended mode: 0:SHA, 1:MD5, 2:CRC16, 3:CRC32"]
pub type SeSha0ModeExtR = crate::FieldReader;
#[doc = "Field `se_sha_0_mode_ext` writer - Extended mode: 0:SHA, 1:MD5, 2:CRC16, 3:CRC32"]
pub type SeSha0ModeExtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `se_sha_0_link_mode` reader - Enable link list mode"]
pub type SeSha0LinkModeR = crate::BitReader;
#[doc = "Field `se_sha_0_link_mode` writer - Enable link list mode"]
pub type SeSha0LinkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_0_msg_len` reader - Message length in 512-bit/1024-bit blocks"]
pub type SeSha0MsgLenR = crate::FieldReader<u16>;
#[doc = "Field `se_sha_0_msg_len` writer - Message length in 512-bit/1024-bit blocks"]
pub type SeSha0MsgLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - SHA engine busy status"]
    #[inline(always)]
    pub fn se_sha_0_busy(&self) -> SeSha0BusyR {
        SeSha0BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - SHA mode: 0:SHA-256, 1:SHA-224, 2:SHA-1, 4:SHA-512, 5:SHA-384"]
    #[inline(always)]
    pub fn se_sha_0_mode(&self) -> SeSha0ModeR {
        SeSha0ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Enable SHA engine"]
    #[inline(always)]
    pub fn se_sha_0_en(&self) -> SeSha0EnR {
        SeSha0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0:New hash, 1:Accumulate from previous result"]
    #[inline(always)]
    pub fn se_sha_0_hash_sel(&self) -> SeSha0HashSelR {
        SeSha0HashSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SHA interrupt status"]
    #[inline(always)]
    pub fn se_sha_0_int(&self) -> SeSha0IntR {
        SeSha0IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask SHA interrupt"]
    #[inline(always)]
    pub fn se_sha_0_int_mask(&self) -> SeSha0IntMaskR {
        SeSha0IntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Extended mode: 0:SHA, 1:MD5, 2:CRC16, 3:CRC32"]
    #[inline(always)]
    pub fn se_sha_0_mode_ext(&self) -> SeSha0ModeExtR {
        SeSha0ModeExtR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Enable link list mode"]
    #[inline(always)]
    pub fn se_sha_0_link_mode(&self) -> SeSha0LinkModeR {
        SeSha0LinkModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Message length in 512-bit/1024-bit blocks"]
    #[inline(always)]
    pub fn se_sha_0_msg_len(&self) -> SeSha0MsgLenR {
        SeSha0MsgLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Trigger SHA calculation (Pulse)"]
    #[inline(always)]
    pub fn se_sha_0_trig_1t(&mut self) -> SeSha0Trig1tW<'_, SeSha0CtrlSpec> {
        SeSha0Trig1tW::new(self, 1)
    }
    #[doc = "Bits 2:4 - SHA mode: 0:SHA-256, 1:SHA-224, 2:SHA-1, 4:SHA-512, 5:SHA-384"]
    #[inline(always)]
    pub fn se_sha_0_mode(&mut self) -> SeSha0ModeW<'_, SeSha0CtrlSpec> {
        SeSha0ModeW::new(self, 2)
    }
    #[doc = "Bit 5 - Enable SHA engine"]
    #[inline(always)]
    pub fn se_sha_0_en(&mut self) -> SeSha0EnW<'_, SeSha0CtrlSpec> {
        SeSha0EnW::new(self, 5)
    }
    #[doc = "Bit 6 - 0:New hash, 1:Accumulate from previous result"]
    #[inline(always)]
    pub fn se_sha_0_hash_sel(&mut self) -> SeSha0HashSelW<'_, SeSha0CtrlSpec> {
        SeSha0HashSelW::new(self, 6)
    }
    #[doc = "Bit 9 - Clear SHA interrupt (Pulse)"]
    #[inline(always)]
    pub fn se_sha_0_int_clr_1t(&mut self) -> SeSha0IntClr1tW<'_, SeSha0CtrlSpec> {
        SeSha0IntClr1tW::new(self, 9)
    }
    #[doc = "Bit 10 - Set SHA interrupt for debug (Pulse)"]
    #[inline(always)]
    pub fn se_sha_0_int_set_1t(&mut self) -> SeSha0IntSet1tW<'_, SeSha0CtrlSpec> {
        SeSha0IntSet1tW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask SHA interrupt"]
    #[inline(always)]
    pub fn se_sha_0_int_mask(&mut self) -> SeSha0IntMaskW<'_, SeSha0CtrlSpec> {
        SeSha0IntMaskW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Extended mode: 0:SHA, 1:MD5, 2:CRC16, 3:CRC32"]
    #[inline(always)]
    pub fn se_sha_0_mode_ext(&mut self) -> SeSha0ModeExtW<'_, SeSha0CtrlSpec> {
        SeSha0ModeExtW::new(self, 12)
    }
    #[doc = "Bit 15 - Enable link list mode"]
    #[inline(always)]
    pub fn se_sha_0_link_mode(&mut self) -> SeSha0LinkModeW<'_, SeSha0CtrlSpec> {
        SeSha0LinkModeW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Message length in 512-bit/1024-bit blocks"]
    #[inline(always)]
    pub fn se_sha_0_msg_len(&mut self) -> SeSha0MsgLenW<'_, SeSha0CtrlSpec> {
        SeSha0MsgLenW::new(self, 16)
    }
}
#[doc = "SHA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0CtrlSpec;
impl crate::RegisterSpec for SeSha0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_ctrl::R`](R) reader structure"]
impl crate::Readable for SeSha0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_ctrl::W`](W) writer structure"]
impl crate::Writable for SeSha0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_SHA_0_CTRL to value 0"]
impl crate::Resettable for SeSha0CtrlSpec {}
