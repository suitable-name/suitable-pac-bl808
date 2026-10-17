#[doc = "Register `DEV_FIBC5` reader"]
pub type R = crate::R<DevFibc5Spec>;
#[doc = "Register `DEV_FIBC5` writer"]
pub type W = crate::W<DevFibc5Spec>;
#[doc = "Field `BC_F5` reader - "]
pub type BcF5R = crate::FieldReader<u16>;
#[doc = "Field `BC_F5` writer - "]
pub type BcF5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST5_HOV` reader - "]
pub type Ffrst5HovR = crate::BitReader;
#[doc = "Field `FFRST5_HOV` writer - "]
pub type Ffrst5HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f5(&self) -> BcF5R {
        BcF5R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst5_hov(&self) -> Ffrst5HovR {
        Ffrst5HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f5(&mut self) -> BcF5W<'_, DevFibc5Spec> {
        BcF5W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst5_hov(&mut self) -> Ffrst5HovW<'_, DevFibc5Spec> {
        Ffrst5HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc5Spec;
impl crate::RegisterSpec for DevFibc5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc5::R`](R) reader structure"]
impl crate::Readable for DevFibc5Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc5::W`](W) writer structure"]
impl crate::Writable for DevFibc5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC5 to value 0"]
impl crate::Resettable for DevFibc5Spec {}
