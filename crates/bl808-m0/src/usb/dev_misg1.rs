#[doc = "Register `DEV_MISG1` reader"]
pub type R = crate::R<DevMisg1Spec>;
#[doc = "Field `MF0_OUT_INT` reader - "]
pub type Mf0OutIntR = crate::BitReader;
#[doc = "Field `MF0_SPK_INT` reader - "]
pub type Mf0SpkIntR = crate::BitReader;
#[doc = "Field `MF1_OUT_INT` reader - "]
pub type Mf1OutIntR = crate::BitReader;
#[doc = "Field `MF1_SPK_INT` reader - "]
pub type Mf1SpkIntR = crate::BitReader;
#[doc = "Field `MF2_OUT_INT` reader - "]
pub type Mf2OutIntR = crate::BitReader;
#[doc = "Field `MF2_SPK_INT` reader - "]
pub type Mf2SpkIntR = crate::BitReader;
#[doc = "Field `MF3_OUT_INT` reader - "]
pub type Mf3OutIntR = crate::BitReader;
#[doc = "Field `MF3_SPK_INT` reader - "]
pub type Mf3SpkIntR = crate::BitReader;
#[doc = "Field `MF4_OUT_INT` reader - "]
pub type Mf4OutIntR = crate::BitReader;
#[doc = "Field `MF4_SPK_INT` reader - "]
pub type Mf4SpkIntR = crate::BitReader;
#[doc = "Field `MF5_OUT_INT` reader - "]
pub type Mf5OutIntR = crate::BitReader;
#[doc = "Field `MF5_SPK_INT` reader - "]
pub type Mf5SpkIntR = crate::BitReader;
#[doc = "Field `MF6_OUT_INT` reader - "]
pub type Mf6OutIntR = crate::BitReader;
#[doc = "Field `MF6_SPK_INT` reader - "]
pub type Mf6SpkIntR = crate::BitReader;
#[doc = "Field `MF7_OUT_INT` reader - "]
pub type Mf7OutIntR = crate::BitReader;
#[doc = "Field `MF7_SPK_INT` reader - "]
pub type Mf7SpkIntR = crate::BitReader;
#[doc = "Field `MF0_IN_INT` reader - "]
pub type Mf0InIntR = crate::BitReader;
#[doc = "Field `MF1_IN_INT` reader - "]
pub type Mf1InIntR = crate::BitReader;
#[doc = "Field `MF2_IN_INT` reader - "]
pub type Mf2InIntR = crate::BitReader;
#[doc = "Field `MF3_IN_INT` reader - "]
pub type Mf3InIntR = crate::BitReader;
#[doc = "Field `MF4_IN_INT` reader - "]
pub type Mf4InIntR = crate::BitReader;
#[doc = "Field `MF5_IN_INT` reader - "]
pub type Mf5InIntR = crate::BitReader;
#[doc = "Field `MF6_IN_INT` reader - "]
pub type Mf6InIntR = crate::BitReader;
#[doc = "Field `MF7_IN_INT` reader - "]
pub type Mf7InIntR = crate::BitReader;
#[doc = "Field `MF8_IN_INT` reader - "]
pub type Mf8InIntR = crate::BitReader;
#[doc = "Field `MF9_IN_INT` reader - "]
pub type Mf9InIntR = crate::BitReader;
#[doc = "Field `MF10_IN_INT` reader - "]
pub type Mf10InIntR = crate::BitReader;
#[doc = "Field `MF11_IN_INT` reader - "]
pub type Mf11InIntR = crate::BitReader;
#[doc = "Field `MF12_IN_INT` reader - "]
pub type Mf12InIntR = crate::BitReader;
#[doc = "Field `MF13_IN_INT` reader - "]
pub type Mf13InIntR = crate::BitReader;
#[doc = "Field `MF14_IN_INT` reader - "]
pub type Mf14InIntR = crate::BitReader;
#[doc = "Field `MF15_IN_INT` reader - "]
pub type Mf15InIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mf0_out_int(&self) -> Mf0OutIntR {
        Mf0OutIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mf0_spk_int(&self) -> Mf0SpkIntR {
        Mf0SpkIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mf1_out_int(&self) -> Mf1OutIntR {
        Mf1OutIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mf1_spk_int(&self) -> Mf1SpkIntR {
        Mf1SpkIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mf2_out_int(&self) -> Mf2OutIntR {
        Mf2OutIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mf2_spk_int(&self) -> Mf2SpkIntR {
        Mf2SpkIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mf3_out_int(&self) -> Mf3OutIntR {
        Mf3OutIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mf3_spk_int(&self) -> Mf3SpkIntR {
        Mf3SpkIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mf4_out_int(&self) -> Mf4OutIntR {
        Mf4OutIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mf4_spk_int(&self) -> Mf4SpkIntR {
        Mf4SpkIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mf5_out_int(&self) -> Mf5OutIntR {
        Mf5OutIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mf5_spk_int(&self) -> Mf5SpkIntR {
        Mf5SpkIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mf6_out_int(&self) -> Mf6OutIntR {
        Mf6OutIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mf6_spk_int(&self) -> Mf6SpkIntR {
        Mf6SpkIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn mf7_out_int(&self) -> Mf7OutIntR {
        Mf7OutIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mf7_spk_int(&self) -> Mf7SpkIntR {
        Mf7SpkIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mf0_in_int(&self) -> Mf0InIntR {
        Mf0InIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mf1_in_int(&self) -> Mf1InIntR {
        Mf1InIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mf2_in_int(&self) -> Mf2InIntR {
        Mf2InIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mf3_in_int(&self) -> Mf3InIntR {
        Mf3InIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn mf4_in_int(&self) -> Mf4InIntR {
        Mf4InIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn mf5_in_int(&self) -> Mf5InIntR {
        Mf5InIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn mf6_in_int(&self) -> Mf6InIntR {
        Mf6InIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mf7_in_int(&self) -> Mf7InIntR {
        Mf7InIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn mf8_in_int(&self) -> Mf8InIntR {
        Mf8InIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mf9_in_int(&self) -> Mf9InIntR {
        Mf9InIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mf10_in_int(&self) -> Mf10InIntR {
        Mf10InIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mf11_in_int(&self) -> Mf11InIntR {
        Mf11InIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mf12_in_int(&self) -> Mf12InIntR {
        Mf12InIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mf13_in_int(&self) -> Mf13InIntR {
        Mf13InIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn mf14_in_int(&self) -> Mf14InIntR {
        Mf14InIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mf15_in_int(&self) -> Mf15InIntR {
        Mf15InIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Masked Interrupt Status Group 1 (FIFO 0-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevMisg1Spec;
impl crate::RegisterSpec for DevMisg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_misg1::R`](R) reader structure"]
impl crate::Readable for DevMisg1Spec {}
#[doc = "`reset()` method sets DEV_MISG1 to value 0"]
impl crate::Resettable for DevMisg1Spec {}
