#[doc = "Register `IRTX_INT_STS` reader"]
pub type R = crate::R<IrtxIntStsSpec>;
#[doc = "Register `IRTX_INT_STS` writer"]
pub type W = crate::W<IrtxIntStsSpec>;
#[doc = "Field `irtx_end_int` reader - IRTX transfer end interrupt status"]
pub type IrtxEndIntR = crate::BitReader;
#[doc = "Field `irtx_frdy_int` reader - IRTX FIFO ready interrupt status"]
pub type IrtxFrdyIntR = crate::BitReader;
#[doc = "Field `irtx_fer_int` reader - IRTX FIFO error interrupt status"]
pub type IrtxFerIntR = crate::BitReader;
#[doc = "Field `cr_irtx_end_mask` reader - Mask IRTX transfer end interrupt"]
pub type CrIrtxEndMaskR = crate::BitReader;
#[doc = "Field `cr_irtx_end_mask` writer - Mask IRTX transfer end interrupt"]
pub type CrIrtxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_frdy_mask` reader - Mask IRTX FIFO ready interrupt"]
pub type CrIrtxFrdyMaskR = crate::BitReader;
#[doc = "Field `cr_irtx_frdy_mask` writer - Mask IRTX FIFO ready interrupt"]
pub type CrIrtxFrdyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_fer_mask` reader - Mask IRTX FIFO error interrupt"]
pub type CrIrtxFerMaskR = crate::BitReader;
#[doc = "Field `cr_irtx_fer_mask` writer - Mask IRTX FIFO error interrupt"]
pub type CrIrtxFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_end_clr` reader - Clear IRTX transfer end interrupt"]
pub type CrIrtxEndClrR = crate::BitReader;
#[doc = "Field `cr_irtx_end_clr` writer - Clear IRTX transfer end interrupt"]
pub type CrIrtxEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_irtx_end_en` reader - Enable IRTX transfer end interrupt"]
pub type CrIrtxEndEnR = crate::BitReader;
#[doc = "Field `cr_irtx_end_en` writer - Enable IRTX transfer end interrupt"]
pub type CrIrtxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_frdy_en` reader - Enable IRTX FIFO ready interrupt"]
pub type CrIrtxFrdyEnR = crate::BitReader;
#[doc = "Field `cr_irtx_frdy_en` writer - Enable IRTX FIFO ready interrupt"]
pub type CrIrtxFrdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irtx_fer_en` reader - Enable IRTX FIFO error interrupt"]
pub type CrIrtxFerEnR = crate::BitReader;
#[doc = "Field `cr_irtx_fer_en` writer - Enable IRTX FIFO error interrupt"]
pub type CrIrtxFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRTX transfer end interrupt status"]
    #[inline(always)]
    pub fn irtx_end_int(&self) -> IrtxEndIntR {
        IrtxEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRTX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn irtx_frdy_int(&self) -> IrtxFrdyIntR {
        IrtxFrdyIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRTX FIFO error interrupt status"]
    #[inline(always)]
    pub fn irtx_fer_int(&self) -> IrtxFerIntR {
        IrtxFerIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask IRTX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&self) -> CrIrtxEndMaskR {
        CrIrtxEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask IRTX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irtx_frdy_mask(&self) -> CrIrtxFrdyMaskR {
        CrIrtxFrdyMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask IRTX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irtx_fer_mask(&self) -> CrIrtxFerMaskR {
        CrIrtxFerMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Clear IRTX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&self) -> CrIrtxEndClrR {
        CrIrtxEndClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable IRTX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&self) -> CrIrtxEndEnR {
        CrIrtxEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable IRTX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irtx_frdy_en(&self) -> CrIrtxFrdyEnR {
        CrIrtxFrdyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable IRTX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irtx_fer_en(&self) -> CrIrtxFerEnR {
        CrIrtxFerEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Mask IRTX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&mut self) -> CrIrtxEndMaskW<'_, IrtxIntStsSpec> {
        CrIrtxEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask IRTX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irtx_frdy_mask(&mut self) -> CrIrtxFrdyMaskW<'_, IrtxIntStsSpec> {
        CrIrtxFrdyMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask IRTX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irtx_fer_mask(&mut self) -> CrIrtxFerMaskW<'_, IrtxIntStsSpec> {
        CrIrtxFerMaskW::new(self, 10)
    }
    #[doc = "Bit 16 - Clear IRTX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&mut self) -> CrIrtxEndClrW<'_, IrtxIntStsSpec> {
        CrIrtxEndClrW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable IRTX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&mut self) -> CrIrtxEndEnW<'_, IrtxIntStsSpec> {
        CrIrtxEndEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable IRTX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irtx_frdy_en(&mut self) -> CrIrtxFrdyEnW<'_, IrtxIntStsSpec> {
        CrIrtxFrdyEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable IRTX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irtx_fer_en(&mut self) -> CrIrtxFerEnW<'_, IrtxIntStsSpec> {
        CrIrtxFerEnW::new(self, 26)
    }
}
#[doc = "IRTX Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxIntStsSpec;
impl crate::RegisterSpec for IrtxIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_int_sts::R`](R) reader structure"]
impl crate::Readable for IrtxIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`irtx_int_sts::W`](W) writer structure"]
impl crate::Writable for IrtxIntStsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets IRTX_INT_STS to value 0x0700_0702"]
impl crate::Resettable for IrtxIntStsSpec {
    const RESET_VALUE: u32 = 0x0700_0702;
}
