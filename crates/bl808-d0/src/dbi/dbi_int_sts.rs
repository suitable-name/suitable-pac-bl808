#[doc = "Register `DBI_INT_STS` reader"]
pub type R = crate::R<DbiIntStsSpec>;
#[doc = "Register `DBI_INT_STS` writer"]
pub type W = crate::W<DbiIntStsSpec>;
#[doc = "Field `dbi_end_int` reader - Transfer end interrupt status"]
pub type DbiEndIntR = crate::BitReader;
#[doc = "Field `dbi_txf_int` reader - TX FIFO ready interrupt status"]
pub type DbiTxfIntR = crate::BitReader;
#[doc = "Field `dbi_fer_int` reader - FIFO error interrupt status"]
pub type DbiFerIntR = crate::BitReader;
#[doc = "Field `cr_dbi_end_mask` reader - Mask transfer end interrupt"]
pub type CrDbiEndMaskR = crate::BitReader;
#[doc = "Field `cr_dbi_end_mask` writer - Mask transfer end interrupt"]
pub type CrDbiEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_txf_mask` reader - Mask TX FIFO ready interrupt"]
pub type CrDbiTxfMaskR = crate::BitReader;
#[doc = "Field `cr_dbi_txf_mask` writer - Mask TX FIFO ready interrupt"]
pub type CrDbiTxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_fer_mask` reader - Mask FIFO error interrupt"]
pub type CrDbiFerMaskR = crate::BitReader;
#[doc = "Field `cr_dbi_fer_mask` writer - Mask FIFO error interrupt"]
pub type CrDbiFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_end_clr` writer - Clear transfer end interrupt"]
pub type CrDbiEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_dbi_end_en` reader - Enable transfer end interrupt"]
pub type CrDbiEndEnR = crate::BitReader;
#[doc = "Field `cr_dbi_end_en` writer - Enable transfer end interrupt"]
pub type CrDbiEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_txf_en` reader - Enable TX FIFO ready interrupt"]
pub type CrDbiTxfEnR = crate::BitReader;
#[doc = "Field `cr_dbi_txf_en` writer - Enable TX FIFO ready interrupt"]
pub type CrDbiTxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_fer_en` reader - Enable FIFO error interrupt"]
pub type CrDbiFerEnR = crate::BitReader;
#[doc = "Field `cr_dbi_fer_en` writer - Enable FIFO error interrupt"]
pub type CrDbiFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer end interrupt status"]
    #[inline(always)]
    pub fn dbi_end_int(&self) -> DbiEndIntR {
        DbiEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn dbi_txf_int(&self) -> DbiTxfIntR {
        DbiTxfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO error interrupt status"]
    #[inline(always)]
    pub fn dbi_fer_int(&self) -> DbiFerIntR {
        DbiFerIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask transfer end interrupt"]
    #[inline(always)]
    pub fn cr_dbi_end_mask(&self) -> CrDbiEndMaskR {
        CrDbiEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_dbi_txf_mask(&self) -> CrDbiTxfMaskR {
        CrDbiTxfMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_dbi_fer_mask(&self) -> CrDbiFerMaskR {
        CrDbiFerMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable transfer end interrupt"]
    #[inline(always)]
    pub fn cr_dbi_end_en(&self) -> CrDbiEndEnR {
        CrDbiEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_dbi_txf_en(&self) -> CrDbiTxfEnR {
        CrDbiTxfEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_dbi_fer_en(&self) -> CrDbiFerEnR {
        CrDbiFerEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Mask transfer end interrupt"]
    #[inline(always)]
    pub fn cr_dbi_end_mask(&mut self) -> CrDbiEndMaskW<'_, DbiIntStsSpec> {
        CrDbiEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_dbi_txf_mask(&mut self) -> CrDbiTxfMaskW<'_, DbiIntStsSpec> {
        CrDbiTxfMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_dbi_fer_mask(&mut self) -> CrDbiFerMaskW<'_, DbiIntStsSpec> {
        CrDbiFerMaskW::new(self, 10)
    }
    #[doc = "Bit 16 - Clear transfer end interrupt"]
    #[inline(always)]
    pub fn cr_dbi_end_clr(&mut self) -> CrDbiEndClrW<'_, DbiIntStsSpec> {
        CrDbiEndClrW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable transfer end interrupt"]
    #[inline(always)]
    pub fn cr_dbi_end_en(&mut self) -> CrDbiEndEnW<'_, DbiIntStsSpec> {
        CrDbiEndEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_dbi_txf_en(&mut self) -> CrDbiTxfEnW<'_, DbiIntStsSpec> {
        CrDbiTxfEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_dbi_fer_en(&mut self) -> CrDbiFerEnW<'_, DbiIntStsSpec> {
        CrDbiFerEnW::new(self, 26)
    }
}
#[doc = "DBI Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiIntStsSpec;
impl crate::RegisterSpec for DbiIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_int_sts::R`](R) reader structure"]
impl crate::Readable for DbiIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_int_sts::W`](W) writer structure"]
impl crate::Writable for DbiIntStsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets DBI_INT_STS to value 0x0700_0702"]
impl crate::Resettable for DbiIntStsSpec {
    const RESET_VALUE: u32 = 0x0700_0702;
}
