#[doc = "Register `SF_CTRL_1` reader"]
pub type R = crate::R<SfCtrl1Spec>;
#[doc = "Register `SF_CTRL_1` writer"]
pub type W = crate::W<SfCtrl1Spec>;
#[doc = "Field `sf_if_sr_pat_mask` reader - "]
pub type SfIfSrPatMaskR = crate::FieldReader;
#[doc = "Field `sf_if_sr_pat_mask` writer - "]
pub type SfIfSrPatMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sf_if_sr_pat` reader - "]
pub type SfIfSrPatR = crate::FieldReader;
#[doc = "Field `sf_if_sr_pat` writer - "]
pub type SfIfSrPatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sf_if_sr_int` reader - "]
pub type SfIfSrIntR = crate::BitReader;
#[doc = "Field `sf_if_sr_int_en` reader - "]
pub type SfIfSrIntEnR = crate::BitReader;
#[doc = "Field `sf_if_sr_int_en` writer - "]
pub type SfIfSrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_sr_int_set` reader - "]
pub type SfIfSrIntSetR = crate::BitReader;
#[doc = "Field `sf_if_sr_int_set` writer - "]
pub type SfIfSrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_ack_lat` reader - "]
pub type SfIf0AckLatR = crate::FieldReader;
#[doc = "Field `sf_if_0_ack_lat` writer - "]
pub type SfIf0AckLatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_ahb2sif_diswrap` reader - "]
pub type SfAhb2sifDiswrapR = crate::BitReader;
#[doc = "Field `sf_ahb2sif_diswrap` writer - "]
pub type SfAhb2sifDiswrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_reg_hold` reader - "]
pub type SfIfRegHoldR = crate::BitReader;
#[doc = "Field `sf_if_reg_hold` writer - "]
pub type SfIfRegHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_reg_wp` reader - "]
pub type SfIfRegWpR = crate::BitReader;
#[doc = "Field `sf_if_reg_wp` writer - "]
pub type SfIfRegWpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ahb2sif_stopped` reader - "]
pub type SfAhb2sifStoppedR = crate::BitReader;
#[doc = "Field `sf_ahb2sif_stop` reader - "]
pub type SfAhb2sifStopR = crate::BitReader;
#[doc = "Field `sf_ahb2sif_stop` writer - "]
pub type SfAhb2sifStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_fn_sel` reader - "]
pub type SfIfFnSelR = crate::BitReader;
#[doc = "Field `sf_if_fn_sel` writer - "]
pub type SfIfFnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_en` reader - "]
pub type SfIfEnR = crate::BitReader;
#[doc = "Field `sf_if_en` writer - "]
pub type SfIfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ahb2sif_en` reader - "]
pub type SfAhb2sifEnR = crate::BitReader;
#[doc = "Field `sf_ahb2sif_en` writer - "]
pub type SfAhb2sifEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ahb2sram_en` reader - "]
pub type SfAhb2sramEnR = crate::BitReader;
#[doc = "Field `sf_ahb2sram_en` writer - "]
pub type SfAhb2sramEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&self) -> SfIfSrPatMaskR {
        SfIfSrPatMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&self) -> SfIfSrPatR {
        SfIfSrPatR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_sr_int(&self) -> SfIfSrIntR {
        SfIfSrIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&self) -> SfIfSrIntEnR {
        SfIfSrIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&self) -> SfIfSrIntSetR {
        SfIfSrIntSetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&self) -> SfIf0AckLatR {
        SfIf0AckLatR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_ahb2sif_diswrap(&self) -> SfAhb2sifDiswrapR {
        SfAhb2sifDiswrapR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&self) -> SfIfRegHoldR {
        SfIfRegHoldR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&self) -> SfIfRegWpR {
        SfIfRegWpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_ahb2sif_stopped(&self) -> SfAhb2sifStoppedR {
        SfAhb2sifStoppedR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&self) -> SfAhb2sifStopR {
        SfAhb2sifStopR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&self) -> SfIfFnSelR {
        SfIfFnSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&self) -> SfIfEnR {
        SfIfEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&self) -> SfAhb2sifEnR {
        SfAhb2sifEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&self) -> SfAhb2sramEnR {
        SfAhb2sramEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&mut self) -> SfIfSrPatMaskW<'_, SfCtrl1Spec> {
        SfIfSrPatMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&mut self) -> SfIfSrPatW<'_, SfCtrl1Spec> {
        SfIfSrPatW::new(self, 8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&mut self) -> SfIfSrIntEnW<'_, SfCtrl1Spec> {
        SfIfSrIntEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&mut self) -> SfIfSrIntSetW<'_, SfCtrl1Spec> {
        SfIfSrIntSetW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&mut self) -> SfIf0AckLatW<'_, SfCtrl1Spec> {
        SfIf0AckLatW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_ahb2sif_diswrap(&mut self) -> SfAhb2sifDiswrapW<'_, SfCtrl1Spec> {
        SfAhb2sifDiswrapW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&mut self) -> SfIfRegHoldW<'_, SfCtrl1Spec> {
        SfIfRegHoldW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&mut self) -> SfIfRegWpW<'_, SfCtrl1Spec> {
        SfIfRegWpW::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&mut self) -> SfAhb2sifStopW<'_, SfCtrl1Spec> {
        SfAhb2sifStopW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&mut self) -> SfIfFnSelW<'_, SfCtrl1Spec> {
        SfIfFnSelW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&mut self) -> SfIfEnW<'_, SfCtrl1Spec> {
        SfIfEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&mut self) -> SfAhb2sifEnW<'_, SfCtrl1Spec> {
        SfAhb2sifEnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&mut self) -> SfAhb2sramEnW<'_, SfCtrl1Spec> {
        SfAhb2sramEnW::new(self, 31)
    }
}
#[doc = "Serial Flash Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrl1Spec;
impl crate::RegisterSpec for SfCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SfCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SfCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_CTRL_1 to value 0"]
impl crate::Resettable for SfCtrl1Spec {}
