#[doc = "Register `GPADC_REG_ISR` reader"]
pub type R = crate::R<GpadcRegIsrSpec>;
#[doc = "Register `GPADC_REG_ISR` writer"]
pub type W = crate::W<GpadcRegIsrSpec>;
#[doc = "Field `gpadc_neg_satur` reader - "]
pub type GpadcNegSaturR = crate::BitReader;
#[doc = "Field `gpadc_pos_satur` reader - "]
pub type GpadcPosSaturR = crate::BitReader;
#[doc = "Field `gpadc_neg_satur_clr` reader - "]
pub type GpadcNegSaturClrR = crate::BitReader;
#[doc = "Field `gpadc_neg_satur_clr` writer - "]
pub type GpadcNegSaturClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pos_satur_clr` reader - "]
pub type GpadcPosSaturClrR = crate::BitReader;
#[doc = "Field `gpadc_pos_satur_clr` writer - "]
pub type GpadcPosSaturClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_neg_satur_mask` reader - "]
pub type GpadcNegSaturMaskR = crate::BitReader;
#[doc = "Field `gpadc_neg_satur_mask` writer - "]
pub type GpadcNegSaturMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pos_satur_mask` reader - "]
pub type GpadcPosSaturMaskR = crate::BitReader;
#[doc = "Field `gpadc_pos_satur_mask` writer - "]
pub type GpadcPosSaturMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_neg_satur(&self) -> GpadcNegSaturR {
        GpadcNegSaturR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_pos_satur(&self) -> GpadcPosSaturR {
        GpadcPosSaturR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&self) -> GpadcNegSaturClrR {
        GpadcNegSaturClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&self) -> GpadcPosSaturClrR {
        GpadcPosSaturClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&self) -> GpadcNegSaturMaskR {
        GpadcNegSaturMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&self) -> GpadcPosSaturMaskR {
        GpadcPosSaturMaskR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&mut self) -> GpadcNegSaturClrW<'_, GpadcRegIsrSpec> {
        GpadcNegSaturClrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&mut self) -> GpadcPosSaturClrW<'_, GpadcRegIsrSpec> {
        GpadcPosSaturClrW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&mut self) -> GpadcNegSaturMaskW<'_, GpadcRegIsrSpec> {
        GpadcNegSaturMaskW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&mut self) -> GpadcPosSaturMaskW<'_, GpadcRegIsrSpec> {
        GpadcPosSaturMaskW::new(self, 9)
    }
}
#[doc = "GPADC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegIsrSpec;
impl crate::RegisterSpec for GpadcRegIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_isr::R`](R) reader structure"]
impl crate::Readable for GpadcRegIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_isr::W`](W) writer structure"]
impl crate::Writable for GpadcRegIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_ISR to value 0"]
impl crate::Resettable for GpadcRegIsrSpec {}
