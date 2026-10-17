#[doc = "Register `URX_CONFIG` reader"]
pub type R = crate::R<UrxConfigSpec>;
#[doc = "Register `URX_CONFIG` writer"]
pub type W = crate::W<UrxConfigSpec>;
#[doc = "Field `cr_urx_en` reader - Enable UART RX"]
pub type CrUrxEnR = crate::BitReader;
#[doc = "Field `cr_urx_en` writer - Enable UART RX"]
pub type CrUrxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_abr_en` reader - Enable RX auto baud rate detection"]
pub type CrUrxAbrEnR = crate::BitReader;
#[doc = "Field `cr_urx_abr_en` writer - Enable RX auto baud rate detection"]
pub type CrUrxAbrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_lin_en` reader - Enable RX LIN mode"]
pub type CrUrxLinEnR = crate::BitReader;
#[doc = "Field `cr_urx_lin_en` writer - Enable RX LIN mode"]
pub type CrUrxLinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_prt_en` reader - Enable RX parity"]
pub type CrUrxPrtEnR = crate::BitReader;
#[doc = "Field `cr_urx_prt_en` writer - Enable RX parity"]
pub type CrUrxPrtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_prt_sel` reader - RX Parity selection: 0:Even, 1:Odd"]
pub type CrUrxPrtSelR = crate::BitReader;
#[doc = "Field `cr_urx_prt_sel` writer - RX Parity selection: 0:Even, 1:Odd"]
pub type CrUrxPrtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ir_en` reader - Enable RX IR mode"]
pub type CrUrxIrEnR = crate::BitReader;
#[doc = "Field `cr_urx_ir_en` writer - Enable RX IR mode"]
pub type CrUrxIrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ir_inv` reader - Invert RX IR signal"]
pub type CrUrxIrInvR = crate::BitReader;
#[doc = "Field `cr_urx_ir_inv` writer - Invert RX IR signal"]
pub type CrUrxIrInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_bit_cnt_d` reader - RX data bit count: 0-3 maps to 5-8 bits"]
pub type CrUrxBitCntDR = crate::FieldReader;
#[doc = "Field `cr_urx_bit_cnt_d` writer - RX data bit count: 0-3 maps to 5-8 bits"]
pub type CrUrxBitCntDW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_urx_deg_en` reader - Enable RX deglitch"]
pub type CrUrxDegEnR = crate::BitReader;
#[doc = "Field `cr_urx_deg_en` writer - Enable RX deglitch"]
pub type CrUrxDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_deg_cnt` reader - RX deglitch cycle count"]
pub type CrUrxDegCntR = crate::FieldReader;
#[doc = "Field `cr_urx_deg_cnt` writer - RX deglitch cycle count"]
pub type CrUrxDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_urx_len` reader - RX transfer length"]
pub type CrUrxLenR = crate::FieldReader<u16>;
#[doc = "Field `cr_urx_len` writer - RX transfer length"]
pub type CrUrxLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable UART RX"]
    #[inline(always)]
    pub fn cr_urx_en(&self) -> CrUrxEnR {
        CrUrxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RX auto baud rate detection"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&self) -> CrUrxAbrEnR {
        CrUrxAbrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable RX LIN mode"]
    #[inline(always)]
    pub fn cr_urx_lin_en(&self) -> CrUrxLinEnR {
        CrUrxLinEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable RX parity"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&self) -> CrUrxPrtEnR {
        CrUrxPrtEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Parity selection: 0:Even, 1:Odd"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&self) -> CrUrxPrtSelR {
        CrUrxPrtSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable RX IR mode"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&self) -> CrUrxIrEnR {
        CrUrxIrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invert RX IR signal"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&self) -> CrUrxIrInvR {
        CrUrxIrInvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RX data bit count: 0-3 maps to 5-8 bits"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&self) -> CrUrxBitCntDR {
        CrUrxBitCntDR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable RX deglitch"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&self) -> CrUrxDegEnR {
        CrUrxDegEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RX deglitch cycle count"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&self) -> CrUrxDegCntR {
        CrUrxDegCntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - RX transfer length"]
    #[inline(always)]
    pub fn cr_urx_len(&self) -> CrUrxLenR {
        CrUrxLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable UART RX"]
    #[inline(always)]
    pub fn cr_urx_en(&mut self) -> CrUrxEnW<'_, UrxConfigSpec> {
        CrUrxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable RX auto baud rate detection"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&mut self) -> CrUrxAbrEnW<'_, UrxConfigSpec> {
        CrUrxAbrEnW::new(self, 1)
    }
    #[doc = "Bit 3 - Enable RX LIN mode"]
    #[inline(always)]
    pub fn cr_urx_lin_en(&mut self) -> CrUrxLinEnW<'_, UrxConfigSpec> {
        CrUrxLinEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable RX parity"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&mut self) -> CrUrxPrtEnW<'_, UrxConfigSpec> {
        CrUrxPrtEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RX Parity selection: 0:Even, 1:Odd"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&mut self) -> CrUrxPrtSelW<'_, UrxConfigSpec> {
        CrUrxPrtSelW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable RX IR mode"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&mut self) -> CrUrxIrEnW<'_, UrxConfigSpec> {
        CrUrxIrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Invert RX IR signal"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&mut self) -> CrUrxIrInvW<'_, UrxConfigSpec> {
        CrUrxIrInvW::new(self, 7)
    }
    #[doc = "Bits 8:10 - RX data bit count: 0-3 maps to 5-8 bits"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&mut self) -> CrUrxBitCntDW<'_, UrxConfigSpec> {
        CrUrxBitCntDW::new(self, 8)
    }
    #[doc = "Bit 11 - Enable RX deglitch"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&mut self) -> CrUrxDegEnW<'_, UrxConfigSpec> {
        CrUrxDegEnW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RX deglitch cycle count"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&mut self) -> CrUrxDegCntW<'_, UrxConfigSpec> {
        CrUrxDegCntW::new(self, 12)
    }
    #[doc = "Bits 16:31 - RX transfer length"]
    #[inline(always)]
    pub fn cr_urx_len(&mut self) -> CrUrxLenW<'_, UrxConfigSpec> {
        CrUrxLenW::new(self, 16)
    }
}
#[doc = "UART RX Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxConfigSpec;
impl crate::RegisterSpec for UrxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_config::R`](R) reader structure"]
impl crate::Readable for UrxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`urx_config::W`](W) writer structure"]
impl crate::Writable for UrxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets URX_CONFIG to value 0x0700"]
impl crate::Resettable for UrxConfigSpec {
    const RESET_VALUE: u32 = 0x0700;
}
