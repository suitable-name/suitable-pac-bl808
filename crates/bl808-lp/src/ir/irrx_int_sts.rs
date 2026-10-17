#[doc = "Register `IRRX_INT_STS` reader"]
pub type R = crate::R<IrrxIntStsSpec>;
#[doc = "Register `IRRX_INT_STS` writer"]
pub type W = crate::W<IrrxIntStsSpec>;
#[doc = "Field `irrx_end_int` reader - IRRX transfer end interrupt status"]
pub type IrrxEndIntR = crate::BitReader;
#[doc = "Field `irrx_frdy_int` reader - IRRX FIFO ready interrupt status"]
pub type IrrxFrdyIntR = crate::BitReader;
#[doc = "Field `irrx_fer_int` reader - IRRX FIFO error interrupt status"]
pub type IrrxFerIntR = crate::BitReader;
#[doc = "Field `cr_irrx_end_mask` reader - Mask IRRX transfer end interrupt"]
pub type CrIrrxEndMaskR = crate::BitReader;
#[doc = "Field `cr_irrx_end_mask` writer - Mask IRRX transfer end interrupt"]
pub type CrIrrxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_frdy_mask` reader - Mask IRRX FIFO ready interrupt"]
pub type CrIrrxFrdyMaskR = crate::BitReader;
#[doc = "Field `cr_irrx_frdy_mask` writer - Mask IRRX FIFO ready interrupt"]
pub type CrIrrxFrdyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_fer_mask` reader - Mask IRRX FIFO error interrupt"]
pub type CrIrrxFerMaskR = crate::BitReader;
#[doc = "Field `cr_irrx_fer_mask` writer - Mask IRRX FIFO error interrupt"]
pub type CrIrrxFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_end_clr` reader - Clear IRRX transfer end interrupt"]
pub type CrIrrxEndClrR = crate::BitReader;
#[doc = "Field `cr_irrx_end_clr` writer - Clear IRRX transfer end interrupt"]
pub type CrIrrxEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_irrx_end_en` reader - Enable IRRX transfer end interrupt"]
pub type CrIrrxEndEnR = crate::BitReader;
#[doc = "Field `cr_irrx_end_en` writer - Enable IRRX transfer end interrupt"]
pub type CrIrrxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_frdy_en` reader - Enable IRRX FIFO ready interrupt"]
pub type CrIrrxFrdyEnR = crate::BitReader;
#[doc = "Field `cr_irrx_frdy_en` writer - Enable IRRX FIFO ready interrupt"]
pub type CrIrrxFrdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_fer_en` reader - Enable IRRX FIFO error interrupt"]
pub type CrIrrxFerEnR = crate::BitReader;
#[doc = "Field `cr_irrx_fer_en` writer - Enable IRRX FIFO error interrupt"]
pub type CrIrrxFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRRX transfer end interrupt status"]
    #[inline(always)]
    pub fn irrx_end_int(&self) -> IrrxEndIntR {
        IrrxEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRRX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn irrx_frdy_int(&self) -> IrrxFrdyIntR {
        IrrxFrdyIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRRX FIFO error interrupt status"]
    #[inline(always)]
    pub fn irrx_fer_int(&self) -> IrrxFerIntR {
        IrrxFerIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask IRRX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irrx_end_mask(&self) -> CrIrrxEndMaskR {
        CrIrrxEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask IRRX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irrx_frdy_mask(&self) -> CrIrrxFrdyMaskR {
        CrIrrxFrdyMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask IRRX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irrx_fer_mask(&self) -> CrIrrxFerMaskR {
        CrIrrxFerMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Clear IRRX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irrx_end_clr(&self) -> CrIrrxEndClrR {
        CrIrrxEndClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable IRRX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irrx_end_en(&self) -> CrIrrxEndEnR {
        CrIrrxEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable IRRX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irrx_frdy_en(&self) -> CrIrrxFrdyEnR {
        CrIrrxFrdyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable IRRX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irrx_fer_en(&self) -> CrIrrxFerEnR {
        CrIrrxFerEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Mask IRRX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irrx_end_mask(&mut self) -> CrIrrxEndMaskW<'_, IrrxIntStsSpec> {
        CrIrrxEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask IRRX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irrx_frdy_mask(&mut self) -> CrIrrxFrdyMaskW<'_, IrrxIntStsSpec> {
        CrIrrxFrdyMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask IRRX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irrx_fer_mask(&mut self) -> CrIrrxFerMaskW<'_, IrrxIntStsSpec> {
        CrIrrxFerMaskW::new(self, 10)
    }
    #[doc = "Bit 16 - Clear IRRX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irrx_end_clr(&mut self) -> CrIrrxEndClrW<'_, IrrxIntStsSpec> {
        CrIrrxEndClrW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable IRRX transfer end interrupt"]
    #[inline(always)]
    pub fn cr_irrx_end_en(&mut self) -> CrIrrxEndEnW<'_, IrrxIntStsSpec> {
        CrIrrxEndEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable IRRX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_irrx_frdy_en(&mut self) -> CrIrrxFrdyEnW<'_, IrrxIntStsSpec> {
        CrIrrxFrdyEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable IRRX FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_irrx_fer_en(&mut self) -> CrIrrxFerEnW<'_, IrrxIntStsSpec> {
        CrIrrxFerEnW::new(self, 26)
    }
}
#[doc = "IRRX Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxIntStsSpec;
impl crate::RegisterSpec for IrrxIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_int_sts::R`](R) reader structure"]
impl crate::Readable for IrrxIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`irrx_int_sts::W`](W) writer structure"]
impl crate::Writable for IrrxIntStsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets IRRX_INT_STS to value 0x0700_0700"]
impl crate::Resettable for IrrxIntStsSpec {
    const RESET_VALUE: u32 = 0x0700_0700;
}
