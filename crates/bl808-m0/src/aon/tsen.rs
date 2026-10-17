#[doc = "Register `TSEN` reader"]
pub type R = crate::R<TsenSpec>;
#[doc = "Register `TSEN` writer"]
pub type W = crate::W<TsenSpec>;
#[doc = "Field `tsen_refcode_corner` reader - "]
pub type TsenRefcodeCornerR = crate::FieldReader<u16>;
#[doc = "Field `tsen_refcode_corner` writer - "]
pub type TsenRefcodeCornerW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `tsen_refcode_rfcal` reader - "]
pub type TsenRefcodeRfcalR = crate::FieldReader<u16>;
#[doc = "Field `tsen_refcode_rfcal` writer - "]
pub type TsenRefcodeRfcalW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `xtal_rdy` reader - "]
pub type XtalRdyR = crate::BitReader;
#[doc = "Field `xtal_inn_cfg_en_aon` reader - "]
pub type XtalInnCfgEnAonR = crate::BitReader;
#[doc = "Field `xtal_inn_cfg_en_aon` writer - "]
pub type XtalInnCfgEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_rdy_int_sel_aon` reader - "]
pub type XtalRdyIntSelAonR = crate::FieldReader;
#[doc = "Field `xtal_rdy_int_sel_aon` writer - "]
pub type XtalRdyIntSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&self) -> TsenRefcodeCornerR {
        TsenRefcodeCornerR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&self) -> TsenRefcodeRfcalR {
        TsenRefcodeRfcalR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_rdy(&self) -> XtalRdyR {
        XtalRdyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&self) -> XtalInnCfgEnAonR {
        XtalInnCfgEnAonR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&self) -> XtalRdyIntSelAonR {
        XtalRdyIntSelAonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&mut self) -> TsenRefcodeCornerW<'_, TsenSpec> {
        TsenRefcodeCornerW::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&mut self) -> TsenRefcodeRfcalW<'_, TsenSpec> {
        TsenRefcodeRfcalW::new(self, 16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&mut self) -> XtalInnCfgEnAonW<'_, TsenSpec> {
        XtalInnCfgEnAonW::new(self, 29)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&mut self) -> XtalRdyIntSelAonW<'_, TsenSpec> {
        XtalRdyIntSelAonW::new(self, 30)
    }
}
#[doc = "Temperature Sensor and XTAL Ready\n\nYou can [`read`](crate::Reg::read) this register and get [`tsen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsenSpec;
impl crate::RegisterSpec for TsenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsen::R`](R) reader structure"]
impl crate::Readable for TsenSpec {}
#[doc = "`write(|w| ..)` method takes [`tsen::W`](W) writer structure"]
impl crate::Writable for TsenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSEN to value 0"]
impl crate::Resettable for TsenSpec {}
