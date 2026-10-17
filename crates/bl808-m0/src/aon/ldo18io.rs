#[doc = "Register `LDO18IO` reader"]
pub type R = crate::R<Ldo18ioSpec>;
#[doc = "Register `LDO18IO` writer"]
pub type W = crate::W<Ldo18ioSpec>;
#[doc = "Field `ldo18io_bypass_iso_aon` reader - "]
pub type Ldo18ioBypassIsoAonR = crate::BitReader;
#[doc = "Field `ldo18io_bypass_iso_aon` writer - "]
pub type Ldo18ioBypassIsoAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_pulldown_aon` reader - "]
pub type Ldo18ioPulldownAonR = crate::BitReader;
#[doc = "Field `ldo18io_pulldown_aon` writer - "]
pub type Ldo18ioPulldownAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_pulldown_sel_aon` reader - "]
pub type Ldo18ioPulldownSelAonR = crate::BitReader;
#[doc = "Field `ldo18io_pulldown_sel_aon` writer - "]
pub type Ldo18ioPulldownSelAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_bm_aon` reader - "]
pub type Ldo18ioBmAonR = crate::FieldReader;
#[doc = "Field `ldo18io_bm_aon` writer - "]
pub type Ldo18ioBmAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_cc_aon` reader - "]
pub type Ldo18ioCcAonR = crate::FieldReader;
#[doc = "Field `ldo18io_cc_aon` writer - "]
pub type Ldo18ioCcAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_ocp_out_aon` reader - "]
pub type Ldo18ioOcpOutAonR = crate::BitReader;
#[doc = "Field `ldo18io_ocp_th_aon` reader - "]
pub type Ldo18ioOcpThAonR = crate::FieldReader;
#[doc = "Field `ldo18io_ocp_th_aon` writer - "]
pub type Ldo18ioOcpThAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_ocp_en_aon` reader - "]
pub type Ldo18ioOcpEnAonR = crate::BitReader;
#[doc = "Field `ldo18io_ocp_en_aon` writer - "]
pub type Ldo18ioOcpEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_sstart_delay_aon` reader - "]
pub type Ldo18ioSstartDelayAonR = crate::FieldReader;
#[doc = "Field `ldo18io_sstart_delay_aon` writer - "]
pub type Ldo18ioSstartDelayAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_sstart_en_aon` reader - "]
pub type Ldo18ioSstartEnAonR = crate::BitReader;
#[doc = "Field `ldo18io_sstart_en_aon` writer - "]
pub type Ldo18ioSstartEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_vout_sel_aon` reader - "]
pub type Ldo18ioVoutSelAonR = crate::FieldReader;
#[doc = "Field `ldo18io_vout_sel_aon` writer - "]
pub type Ldo18ioVoutSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo18io_vout_trim_aon` reader - "]
pub type Ldo18ioVoutTrimAonR = crate::FieldReader;
#[doc = "Field `ldo18io_vout_trim_aon` writer - "]
pub type Ldo18ioVoutTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18io_bypass_iso_aon(&self) -> Ldo18ioBypassIsoAonR {
        Ldo18ioBypassIsoAonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18io_pulldown_aon(&self) -> Ldo18ioPulldownAonR {
        Ldo18ioPulldownAonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18io_pulldown_sel_aon(&self) -> Ldo18ioPulldownSelAonR {
        Ldo18ioPulldownSelAonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18io_bm_aon(&self) -> Ldo18ioBmAonR {
        Ldo18ioBmAonR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18io_cc_aon(&self) -> Ldo18ioCcAonR {
        Ldo18ioCcAonR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo18io_ocp_out_aon(&self) -> Ldo18ioOcpOutAonR {
        Ldo18ioOcpOutAonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18io_ocp_th_aon(&self) -> Ldo18ioOcpThAonR {
        Ldo18ioOcpThAonR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18io_ocp_en_aon(&self) -> Ldo18ioOcpEnAonR {
        Ldo18ioOcpEnAonR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo18io_sstart_delay_aon(&self) -> Ldo18ioSstartDelayAonR {
        Ldo18ioSstartDelayAonR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo18io_sstart_en_aon(&self) -> Ldo18ioSstartEnAonR {
        Ldo18ioSstartEnAonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo18io_vout_sel_aon(&self) -> Ldo18ioVoutSelAonR {
        Ldo18ioVoutSelAonR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18io_vout_trim_aon(&self) -> Ldo18ioVoutTrimAonR {
        Ldo18ioVoutTrimAonR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18io_bypass_iso_aon(&mut self) -> Ldo18ioBypassIsoAonW<'_, Ldo18ioSpec> {
        Ldo18ioBypassIsoAonW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18io_pulldown_aon(&mut self) -> Ldo18ioPulldownAonW<'_, Ldo18ioSpec> {
        Ldo18ioPulldownAonW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18io_pulldown_sel_aon(&mut self) -> Ldo18ioPulldownSelAonW<'_, Ldo18ioSpec> {
        Ldo18ioPulldownSelAonW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18io_bm_aon(&mut self) -> Ldo18ioBmAonW<'_, Ldo18ioSpec> {
        Ldo18ioBmAonW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18io_cc_aon(&mut self) -> Ldo18ioCcAonW<'_, Ldo18ioSpec> {
        Ldo18ioCcAonW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18io_ocp_th_aon(&mut self) -> Ldo18ioOcpThAonW<'_, Ldo18ioSpec> {
        Ldo18ioOcpThAonW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18io_ocp_en_aon(&mut self) -> Ldo18ioOcpEnAonW<'_, Ldo18ioSpec> {
        Ldo18ioOcpEnAonW::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo18io_sstart_delay_aon(&mut self) -> Ldo18ioSstartDelayAonW<'_, Ldo18ioSpec> {
        Ldo18ioSstartDelayAonW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo18io_sstart_en_aon(&mut self) -> Ldo18ioSstartEnAonW<'_, Ldo18ioSpec> {
        Ldo18ioSstartEnAonW::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo18io_vout_sel_aon(&mut self) -> Ldo18ioVoutSelAonW<'_, Ldo18ioSpec> {
        Ldo18ioVoutSelAonW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18io_vout_trim_aon(&mut self) -> Ldo18ioVoutTrimAonW<'_, Ldo18ioSpec> {
        Ldo18ioVoutTrimAonW::new(self, 24)
    }
}
#[doc = "LDO18 IO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo18io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo18io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo18ioSpec;
impl crate::RegisterSpec for Ldo18ioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo18io::R`](R) reader structure"]
impl crate::Readable for Ldo18ioSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo18io::W`](W) writer structure"]
impl crate::Writable for Ldo18ioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO18IO to value 0"]
impl crate::Resettable for Ldo18ioSpec {}
