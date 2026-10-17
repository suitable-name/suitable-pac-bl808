#[doc = "Register `HBN_PAD_CTRL_1` reader"]
pub type R = crate::R<HbnPadCtrl1Spec>;
#[doc = "Register `HBN_PAD_CTRL_1` writer"]
pub type W = crate::W<HbnPadCtrl1Spec>;
#[doc = "Field `reg_aon_pad_oe` reader - "]
pub type RegAonPadOeR = crate::FieldReader<u16>;
#[doc = "Field `reg_aon_pad_oe` writer - "]
pub type RegAonPadOeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_aon_pad_pd` reader - "]
pub type RegAonPadPdR = crate::FieldReader<u16>;
#[doc = "Field `reg_aon_pad_pd` writer - "]
pub type RegAonPadPdW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_aon_pad_pu` reader - "]
pub type RegAonPadPuR = crate::FieldReader<u16>;
#[doc = "Field `reg_aon_pad_pu` writer - "]
pub type RegAonPadPuW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reg_aon_pad_oe(&self) -> RegAonPadOeR {
        RegAonPadOeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_pad_pd(&self) -> RegAonPadPdR {
        RegAonPadPdR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn reg_aon_pad_pu(&self) -> RegAonPadPuR {
        RegAonPadPuR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reg_aon_pad_oe(&mut self) -> RegAonPadOeW<'_, HbnPadCtrl1Spec> {
        RegAonPadOeW::new(self, 0)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_pad_pd(&mut self) -> RegAonPadPdW<'_, HbnPadCtrl1Spec> {
        RegAonPadPdW::new(self, 10)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn reg_aon_pad_pu(&mut self) -> RegAonPadPuW<'_, HbnPadCtrl1Spec> {
        RegAonPadPuW::new(self, 20)
    }
}
#[doc = "AON GPIO Pad Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pad_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pad_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnPadCtrl1Spec;
impl crate::RegisterSpec for HbnPadCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pad_ctrl_1::R`](R) reader structure"]
impl crate::Readable for HbnPadCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`hbn_pad_ctrl_1::W`](W) writer structure"]
impl crate::Writable for HbnPadCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_PAD_CTRL_1 to value 0"]
impl crate::Resettable for HbnPadCtrl1Spec {}
