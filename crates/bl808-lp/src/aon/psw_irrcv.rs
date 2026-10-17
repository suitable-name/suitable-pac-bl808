#[doc = "Register `PSW_IRRCV` reader"]
pub type R = crate::R<PswIrrcvSpec>;
#[doc = "Register `PSW_IRRCV` writer"]
pub type W = crate::W<PswIrrcvSpec>;
#[doc = "Field `pu_psw_irrcv_aon` reader - "]
pub type PuPswIrrcvAonR = crate::BitReader;
#[doc = "Field `pu_psw_irrcv_aon` writer - "]
pub type PuPswIrrcvAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb20_rref_ext_en_aon` reader - "]
pub type Usb20RrefExtEnAonR = crate::BitReader;
#[doc = "Field `usb20_rref_ext_en_aon` writer - "]
pub type Usb20RrefExtEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_por33_aon` reader - "]
pub type EnPor33AonR = crate::BitReader;
#[doc = "Field `en_por33_aon` writer - "]
pub type EnPor33AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb20_rref_hiz_aon` reader - "]
pub type Usb20RrefHizAonR = crate::BitReader;
#[doc = "Field `usb20_rref_hiz_aon` writer - "]
pub type Usb20RrefHizAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb20_rcal_code_aon` reader - "]
pub type Usb20RcalCodeAonR = crate::FieldReader;
#[doc = "Field `usb20_rcal_code_aon` writer - "]
pub type Usb20RcalCodeAonW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_psw_irrcv_aon(&self) -> PuPswIrrcvAonR {
        PuPswIrrcvAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn usb20_rref_ext_en_aon(&self) -> Usb20RrefExtEnAonR {
        Usb20RrefExtEnAonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn en_por33_aon(&self) -> EnPor33AonR {
        EnPor33AonR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb20_rref_hiz_aon(&self) -> Usb20RrefHizAonR {
        Usb20RrefHizAonR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn usb20_rcal_code_aon(&self) -> Usb20RcalCodeAonR {
        Usb20RcalCodeAonR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_psw_irrcv_aon(&mut self) -> PuPswIrrcvAonW<'_, PswIrrcvSpec> {
        PuPswIrrcvAonW::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn usb20_rref_ext_en_aon(&mut self) -> Usb20RrefExtEnAonW<'_, PswIrrcvSpec> {
        Usb20RrefExtEnAonW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn en_por33_aon(&mut self) -> EnPor33AonW<'_, PswIrrcvSpec> {
        EnPor33AonW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb20_rref_hiz_aon(&mut self) -> Usb20RrefHizAonW<'_, PswIrrcvSpec> {
        Usb20RrefHizAonW::new(self, 21)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn usb20_rcal_code_aon(&mut self) -> Usb20RcalCodeAonW<'_, PswIrrcvSpec> {
        Usb20RcalCodeAonW::new(self, 24)
    }
}
#[doc = "PSW and USB Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`psw_irrcv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psw_irrcv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PswIrrcvSpec;
impl crate::RegisterSpec for PswIrrcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psw_irrcv::R`](R) reader structure"]
impl crate::Readable for PswIrrcvSpec {}
#[doc = "`write(|w| ..)` method takes [`psw_irrcv::W`](W) writer structure"]
impl crate::Writable for PswIrrcvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSW_IRRCV to value 0"]
impl crate::Resettable for PswIrrcvSpec {}
