#[doc = "Register `ACOMP_CTRL` reader"]
pub type R = crate::R<AcompCtrlSpec>;
#[doc = "Register `ACOMP_CTRL` writer"]
pub type W = crate::W<AcompCtrlSpec>;
#[doc = "Field `acomp1_rstn_ana` reader - "]
pub type Acomp1RstnAnaR = crate::BitReader;
#[doc = "Field `acomp1_rstn_ana` writer - "]
pub type Acomp1RstnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp0_rstn_ana` reader - "]
pub type Acomp0RstnAnaR = crate::BitReader;
#[doc = "Field `acomp0_rstn_ana` writer - "]
pub type Acomp0RstnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp1_test_en` reader - "]
pub type Acomp1TestEnR = crate::BitReader;
#[doc = "Field `acomp1_test_en` writer - "]
pub type Acomp1TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp0_test_en` reader - "]
pub type Acomp0TestEnR = crate::BitReader;
#[doc = "Field `acomp0_test_en` writer - "]
pub type Acomp0TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp1_test_sel` reader - "]
pub type Acomp1TestSelR = crate::FieldReader;
#[doc = "Field `acomp1_test_sel` writer - "]
pub type Acomp1TestSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp0_test_sel` reader - "]
pub type Acomp0TestSelR = crate::FieldReader;
#[doc = "Field `acomp0_test_sel` writer - "]
pub type Acomp0TestSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp1_out_raw` reader - "]
pub type Acomp1OutRawR = crate::BitReader;
#[doc = "Field `acomp0_out_raw` reader - "]
pub type Acomp0OutRawR = crate::BitReader;
#[doc = "Field `acomp_vref_sel` reader - "]
pub type AcompVrefSelR = crate::FieldReader;
#[doc = "Field `acomp_vref_sel` writer - "]
pub type AcompVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `acomp_reserved` reader - "]
pub type AcompReservedR = crate::FieldReader;
#[doc = "Field `acomp_reserved` writer - "]
pub type AcompReservedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&self) -> Acomp1RstnAnaR {
        Acomp1RstnAnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&self) -> Acomp0RstnAnaR {
        Acomp0RstnAnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&self) -> Acomp1TestEnR {
        Acomp1TestEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&self) -> Acomp0TestEnR {
        Acomp0TestEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&self) -> Acomp1TestSelR {
        Acomp1TestSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&self) -> Acomp0TestSelR {
        Acomp0TestSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&self) -> Acomp1OutRawR {
        Acomp1OutRawR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&self) -> Acomp0OutRawR {
        Acomp0OutRawR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn acomp_vref_sel(&self) -> AcompVrefSelR {
        AcompVrefSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn acomp_reserved(&self) -> AcompReservedR {
        AcompReservedR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&mut self) -> Acomp1RstnAnaW<'_, AcompCtrlSpec> {
        Acomp1RstnAnaW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&mut self) -> Acomp0RstnAnaW<'_, AcompCtrlSpec> {
        Acomp0RstnAnaW::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&mut self) -> Acomp1TestEnW<'_, AcompCtrlSpec> {
        Acomp1TestEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&mut self) -> Acomp0TestEnW<'_, AcompCtrlSpec> {
        Acomp0TestEnW::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&mut self) -> Acomp1TestSelW<'_, AcompCtrlSpec> {
        Acomp1TestSelW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&mut self) -> Acomp0TestSelW<'_, AcompCtrlSpec> {
        Acomp0TestSelW::new(self, 12)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn acomp_vref_sel(&mut self) -> AcompVrefSelW<'_, AcompCtrlSpec> {
        AcompVrefSelW::new(self, 24)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn acomp_reserved(&mut self) -> AcompReservedW<'_, AcompCtrlSpec> {
        AcompReservedW::new(self, 30)
    }
}
#[doc = "Analog Comparator Shared Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcompCtrlSpec;
impl crate::RegisterSpec for AcompCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acomp_ctrl::R`](R) reader structure"]
impl crate::Readable for AcompCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`acomp_ctrl::W`](W) writer structure"]
impl crate::Writable for AcompCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACOMP_CTRL to value 0"]
impl crate::Resettable for AcompCtrlSpec {}
