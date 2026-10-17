#[doc = "Register `DEV_FIBC4` reader"]
pub type R = crate::R<DevFibc4Spec>;
#[doc = "Register `DEV_FIBC4` writer"]
pub type W = crate::W<DevFibc4Spec>;
#[doc = "Field `BC_F4` reader - "]
pub type BcF4R = crate::FieldReader<u16>;
#[doc = "Field `BC_F4` writer - "]
pub type BcF4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST4_HOV` reader - "]
pub type Ffrst4HovR = crate::BitReader;
#[doc = "Field `FFRST4_HOV` writer - "]
pub type Ffrst4HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f4(&self) -> BcF4R {
        BcF4R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst4_hov(&self) -> Ffrst4HovR {
        Ffrst4HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f4(&mut self) -> BcF4W<'_, DevFibc4Spec> {
        BcF4W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst4_hov(&mut self) -> Ffrst4HovW<'_, DevFibc4Spec> {
        Ffrst4HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc4Spec;
impl crate::RegisterSpec for DevFibc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc4::R`](R) reader structure"]
impl crate::Readable for DevFibc4Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc4::W`](W) writer structure"]
impl crate::Writable for DevFibc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC4 to value 0"]
impl crate::Resettable for DevFibc4Spec {}
