#[doc = "Register `PROC_MON` reader"]
pub type R = crate::R<ProcMonSpec>;
#[doc = "Register `PROC_MON` writer"]
pub type W = crate::W<ProcMonSpec>;
#[doc = "Field `pu_proc_mon` reader - "]
pub type PuProcMonR = crate::BitReader;
#[doc = "Field `pu_proc_mon` writer - "]
pub type PuProcMonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc_en_rvt` reader - "]
pub type OscEnRvtR = crate::BitReader;
#[doc = "Field `osc_en_rvt` writer - "]
pub type OscEnRvtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc_en_lvt` reader - "]
pub type OscEnLvtR = crate::BitReader;
#[doc = "Field `osc_en_lvt` writer - "]
pub type OscEnLvtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc_sel` reader - "]
pub type OscSelR = crate::BitReader;
#[doc = "Field `osc_sel` writer - "]
pub type OscSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_ringcount` reader - "]
pub type RstnRingcountR = crate::BitReader;
#[doc = "Field `rstn_ringcount` writer - "]
pub type RstnRingcountW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_refcount` reader - "]
pub type RstnRefcountR = crate::BitReader;
#[doc = "Field `rstn_refcount` writer - "]
pub type RstnRefcountW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refcount_div_onehot` reader - "]
pub type RefcountDivOnehotR = crate::FieldReader;
#[doc = "Field `refcount_div_onehot` writer - "]
pub type RefcountDivOnehotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ring_freq` reader - "]
pub type RingFreqR = crate::FieldReader<u16>;
#[doc = "Field `ring_freq_rdy` reader - "]
pub type RingFreqRdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_proc_mon(&self) -> PuProcMonR {
        PuProcMonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn osc_en_rvt(&self) -> OscEnRvtR {
        OscEnRvtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn osc_en_lvt(&self) -> OscEnLvtR {
        OscEnLvtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn osc_sel(&self) -> OscSelR {
        OscSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rstn_ringcount(&self) -> RstnRingcountR {
        RstnRingcountR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rstn_refcount(&self) -> RstnRefcountR {
        RstnRefcountR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn refcount_div_onehot(&self) -> RefcountDivOnehotR {
        RefcountDivOnehotR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27"]
    #[inline(always)]
    pub fn ring_freq(&self) -> RingFreqR {
        RingFreqR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ring_freq_rdy(&self) -> RingFreqRdyR {
        RingFreqRdyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_proc_mon(&mut self) -> PuProcMonW<'_, ProcMonSpec> {
        PuProcMonW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn osc_en_rvt(&mut self) -> OscEnRvtW<'_, ProcMonSpec> {
        OscEnRvtW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn osc_en_lvt(&mut self) -> OscEnLvtW<'_, ProcMonSpec> {
        OscEnLvtW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn osc_sel(&mut self) -> OscSelW<'_, ProcMonSpec> {
        OscSelW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rstn_ringcount(&mut self) -> RstnRingcountW<'_, ProcMonSpec> {
        RstnRingcountW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rstn_refcount(&mut self) -> RstnRefcountW<'_, ProcMonSpec> {
        RstnRefcountW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn refcount_div_onehot(&mut self) -> RefcountDivOnehotW<'_, ProcMonSpec> {
        RefcountDivOnehotW::new(self, 8)
    }
}
#[doc = "Process Monitor Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`proc_mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc_mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProcMonSpec;
impl crate::RegisterSpec for ProcMonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc_mon::R`](R) reader structure"]
impl crate::Readable for ProcMonSpec {}
#[doc = "`write(|w| ..)` method takes [`proc_mon::W`](W) writer structure"]
impl crate::Writable for ProcMonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROC_MON to value 0"]
impl crate::Resettable for ProcMonSpec {}
