#[doc = "Register `IRTX_CONFIG` reader"]
pub type R = crate::R<IrtxConfigSpec>;
#[doc = "Register `IRTX_CONFIG` writer"]
pub type W = crate::W<IrtxConfigSpec>;
#[doc = "Field `cr_irtx_en` reader - Enable IRTX module"]
pub type CrIrtxEnR = crate::BitReader;
#[doc = "Field `cr_irtx_en` writer - Enable IRTX module"]
pub type CrIrtxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_out_inv` reader - Invert IRTX output signal"]
pub type CrIrtxOutInvR = crate::BitReader;
#[doc = "Field `cr_irtx_out_inv` writer - Invert IRTX output signal"]
pub type CrIrtxOutInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_mod_en` reader - Enable modulation for IRTX"]
pub type CrIrtxModEnR = crate::BitReader;
#[doc = "Field `cr_irtx_mod_en` writer - Enable modulation for IRTX"]
pub type CrIrtxModEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_swm_en` reader - Enable software mode for IRTX"]
pub type CrIrtxSwmEnR = crate::BitReader;
#[doc = "Field `cr_irtx_swm_en` writer - Enable software mode for IRTX"]
pub type CrIrtxSwmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_data_en` reader - Enable data phase for IRTX"]
pub type CrIrtxDataEnR = crate::BitReader;
#[doc = "Field `cr_irtx_data_en` writer - Enable data phase for IRTX"]
pub type CrIrtxDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_logic0_hl_inv` reader - Invert logic 0 high/low phase"]
pub type CrIrtxLogic0HlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_logic0_hl_inv` writer - Invert logic 0 high/low phase"]
pub type CrIrtxLogic0HlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_logic1_hl_inv` reader - Invert logic 1 high/low phase"]
pub type CrIrtxLogic1HlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_logic1_hl_inv` writer - Invert logic 1 high/low phase"]
pub type CrIrtxLogic1HlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_head_en` reader - Enable head phase"]
pub type CrIrtxHeadEnR = crate::BitReader;
#[doc = "Field `cr_irtx_head_en` writer - Enable head phase"]
pub type CrIrtxHeadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_head_hl_inv` reader - Invert head high/low phase"]
pub type CrIrtxHeadHlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_head_hl_inv` writer - Invert head high/low phase"]
pub type CrIrtxHeadHlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_tail_en` reader - Enable tail phase"]
pub type CrIrtxTailEnR = crate::BitReader;
#[doc = "Field `cr_irtx_tail_en` writer - Enable tail phase"]
pub type CrIrtxTailEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_tail_hl_inv` reader - Invert tail high/low phase"]
pub type CrIrtxTailHlInvR = crate::BitReader;
#[doc = "Field `cr_irtx_tail_hl_inv` writer - Invert tail high/low phase"]
pub type CrIrtxTailHlInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_frm_en` reader - Enable frame mode"]
pub type CrIrtxFrmEnR = crate::BitReader;
#[doc = "Field `cr_irtx_frm_en` writer - Enable frame mode"]
pub type CrIrtxFrmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_frm_cont_en` reader - Enable continuous frame mode"]
pub type CrIrtxFrmContEnR = crate::BitReader;
#[doc = "Field `cr_irtx_frm_cont_en` writer - Enable continuous frame mode"]
pub type CrIrtxFrmContEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_frm_frame_size` reader - Frame size selection"]
pub type CrIrtxFrmFrameSizeR = crate::FieldReader;
#[doc = "Field `cr_irtx_frm_frame_size` writer - Frame size selection"]
pub type CrIrtxFrmFrameSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_irtx_data_num` reader - Number of data bits to transmit"]
pub type CrIrtxDataNumR = crate::FieldReader;
#[doc = "Field `cr_irtx_data_num` writer - Number of data bits to transmit"]
pub type CrIrtxDataNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable IRTX module"]
    #[inline(always)]
    pub fn cr_irtx_en(&self) -> CrIrtxEnR {
        CrIrtxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert IRTX output signal"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&self) -> CrIrtxOutInvR {
        CrIrtxOutInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable modulation for IRTX"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&self) -> CrIrtxModEnR {
        CrIrtxModEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable software mode for IRTX"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&self) -> CrIrtxSwmEnR {
        CrIrtxSwmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable data phase for IRTX"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&self) -> CrIrtxDataEnR {
        CrIrtxDataEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert logic 0 high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&self) -> CrIrtxLogic0HlInvR {
        CrIrtxLogic0HlInvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Invert logic 1 high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&self) -> CrIrtxLogic1HlInvR {
        CrIrtxLogic1HlInvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable head phase"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&self) -> CrIrtxHeadEnR {
        CrIrtxHeadEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Invert head high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&self) -> CrIrtxHeadHlInvR {
        CrIrtxHeadHlInvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable tail phase"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&self) -> CrIrtxTailEnR {
        CrIrtxTailEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Invert tail high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&self) -> CrIrtxTailHlInvR {
        CrIrtxTailHlInvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable frame mode"]
    #[inline(always)]
    pub fn cr_irtx_frm_en(&self) -> CrIrtxFrmEnR {
        CrIrtxFrmEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable continuous frame mode"]
    #[inline(always)]
    pub fn cr_irtx_frm_cont_en(&self) -> CrIrtxFrmContEnR {
        CrIrtxFrmContEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Frame size selection"]
    #[inline(always)]
    pub fn cr_irtx_frm_frame_size(&self) -> CrIrtxFrmFrameSizeR {
        CrIrtxFrmFrameSizeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Number of data bits to transmit"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&self) -> CrIrtxDataNumR {
        CrIrtxDataNumR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IRTX module"]
    #[inline(always)]
    pub fn cr_irtx_en(&mut self) -> CrIrtxEnW<'_, IrtxConfigSpec> {
        CrIrtxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Invert IRTX output signal"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&mut self) -> CrIrtxOutInvW<'_, IrtxConfigSpec> {
        CrIrtxOutInvW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable modulation for IRTX"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&mut self) -> CrIrtxModEnW<'_, IrtxConfigSpec> {
        CrIrtxModEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable software mode for IRTX"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&mut self) -> CrIrtxSwmEnW<'_, IrtxConfigSpec> {
        CrIrtxSwmEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable data phase for IRTX"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&mut self) -> CrIrtxDataEnW<'_, IrtxConfigSpec> {
        CrIrtxDataEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Invert logic 0 high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&mut self) -> CrIrtxLogic0HlInvW<'_, IrtxConfigSpec> {
        CrIrtxLogic0HlInvW::new(self, 5)
    }
    #[doc = "Bit 6 - Invert logic 1 high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&mut self) -> CrIrtxLogic1HlInvW<'_, IrtxConfigSpec> {
        CrIrtxLogic1HlInvW::new(self, 6)
    }
    #[doc = "Bit 8 - Enable head phase"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&mut self) -> CrIrtxHeadEnW<'_, IrtxConfigSpec> {
        CrIrtxHeadEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Invert head high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&mut self) -> CrIrtxHeadHlInvW<'_, IrtxConfigSpec> {
        CrIrtxHeadHlInvW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable tail phase"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&mut self) -> CrIrtxTailEnW<'_, IrtxConfigSpec> {
        CrIrtxTailEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Invert tail high/low phase"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&mut self) -> CrIrtxTailHlInvW<'_, IrtxConfigSpec> {
        CrIrtxTailHlInvW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable frame mode"]
    #[inline(always)]
    pub fn cr_irtx_frm_en(&mut self) -> CrIrtxFrmEnW<'_, IrtxConfigSpec> {
        CrIrtxFrmEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable continuous frame mode"]
    #[inline(always)]
    pub fn cr_irtx_frm_cont_en(&mut self) -> CrIrtxFrmContEnW<'_, IrtxConfigSpec> {
        CrIrtxFrmContEnW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Frame size selection"]
    #[inline(always)]
    pub fn cr_irtx_frm_frame_size(&mut self) -> CrIrtxFrmFrameSizeW<'_, IrtxConfigSpec> {
        CrIrtxFrmFrameSizeW::new(self, 14)
    }
    #[doc = "Bits 16:22 - Number of data bits to transmit"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&mut self) -> CrIrtxDataNumW<'_, IrtxConfigSpec> {
        CrIrtxDataNumW::new(self, 16)
    }
}
#[doc = "IRTX Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxConfigSpec;
impl crate::RegisterSpec for IrtxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_config::R`](R) reader structure"]
impl crate::Readable for IrtxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`irtx_config::W`](W) writer structure"]
impl crate::Writable for IrtxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRTX_CONFIG to value 0x001f_0510"]
impl crate::Resettable for IrtxConfigSpec {
    const RESET_VALUE: u32 = 0x001f_0510;
}
