#[doc = "Register `SF_IF2_CTRL_1` reader"]
pub type R = crate::R<SfIf2Ctrl1Spec>;
#[doc = "Register `SF_IF2_CTRL_1` writer"]
pub type W = crate::W<SfIf2Ctrl1Spec>;
#[doc = "Field `sf_if2_sr_pat_mask` reader - "]
pub type SfIf2SrPatMaskR = crate::FieldReader;
#[doc = "Field `sf_if2_sr_pat_mask` writer - "]
pub type SfIf2SrPatMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sf_if2_sr_pat` reader - "]
pub type SfIf2SrPatR = crate::FieldReader;
#[doc = "Field `sf_if2_sr_pat` writer - "]
pub type SfIf2SrPatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sf_if2_sr_int` reader - "]
pub type SfIf2SrIntR = crate::BitReader;
#[doc = "Field `sf_if2_sr_int_en` reader - "]
pub type SfIf2SrIntEnR = crate::BitReader;
#[doc = "Field `sf_if2_sr_int_en` writer - "]
pub type SfIf2SrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_sr_int_set` reader - "]
pub type SfIf2SrIntSetR = crate::BitReader;
#[doc = "Field `sf_if2_sr_int_set` writer - "]
pub type SfIf2SrIntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_ack_lat` reader - "]
pub type SfIf2AckLatR = crate::FieldReader;
#[doc = "Field `sf_if2_ack_lat` writer - "]
pub type SfIf2AckLatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if2_reg_hold` reader - "]
pub type SfIf2RegHoldR = crate::BitReader;
#[doc = "Field `sf_if2_reg_hold` writer - "]
pub type SfIf2RegHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_reg_wp` reader - "]
pub type SfIf2RegWpR = crate::BitReader;
#[doc = "Field `sf_if2_reg_wp` writer - "]
pub type SfIf2RegWpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_fn_sel` reader - "]
pub type SfIf2FnSelR = crate::BitReader;
#[doc = "Field `sf_if2_fn_sel` writer - "]
pub type SfIf2FnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_en` reader - "]
pub type SfIf2EnR = crate::BitReader;
#[doc = "Field `sf_if2_en` writer - "]
pub type SfIf2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if2_sr_pat_mask(&self) -> SfIf2SrPatMaskR {
        SfIf2SrPatMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if2_sr_pat(&self) -> SfIf2SrPatR {
        SfIf2SrPatR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if2_sr_int(&self) -> SfIf2SrIntR {
        SfIf2SrIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if2_sr_int_en(&self) -> SfIf2SrIntEnR {
        SfIf2SrIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if2_sr_int_set(&self) -> SfIf2SrIntSetR {
        SfIf2SrIntSetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if2_ack_lat(&self) -> SfIf2AckLatR {
        SfIf2AckLatR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_reg_hold(&self) -> SfIf2RegHoldR {
        SfIf2RegHoldR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_reg_wp(&self) -> SfIf2RegWpR {
        SfIf2RegWpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if2_fn_sel(&self) -> SfIf2FnSelR {
        SfIf2FnSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if2_en(&self) -> SfIf2EnR {
        SfIf2EnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if2_sr_pat_mask(&mut self) -> SfIf2SrPatMaskW<'_, SfIf2Ctrl1Spec> {
        SfIf2SrPatMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if2_sr_pat(&mut self) -> SfIf2SrPatW<'_, SfIf2Ctrl1Spec> {
        SfIf2SrPatW::new(self, 8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if2_sr_int_en(&mut self) -> SfIf2SrIntEnW<'_, SfIf2Ctrl1Spec> {
        SfIf2SrIntEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if2_sr_int_set(&mut self) -> SfIf2SrIntSetW<'_, SfIf2Ctrl1Spec> {
        SfIf2SrIntSetW::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if2_ack_lat(&mut self) -> SfIf2AckLatW<'_, SfIf2Ctrl1Spec> {
        SfIf2AckLatW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_reg_hold(&mut self) -> SfIf2RegHoldW<'_, SfIf2Ctrl1Spec> {
        SfIf2RegHoldW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_reg_wp(&mut self) -> SfIf2RegWpW<'_, SfIf2Ctrl1Spec> {
        SfIf2RegWpW::new(self, 25)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if2_fn_sel(&mut self) -> SfIf2FnSelW<'_, SfIf2Ctrl1Spec> {
        SfIf2FnSelW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if2_en(&mut self) -> SfIf2EnW<'_, SfIf2Ctrl1Spec> {
        SfIf2EnW::new(self, 29)
    }
}
#[doc = "Serial Flash 2 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIf2Ctrl1Spec;
impl crate::RegisterSpec for SfIf2Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if2_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SfIf2Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if2_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SfIf2Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF2_CTRL_1 to value 0"]
impl crate::Resettable for SfIf2Ctrl1Spec {}
