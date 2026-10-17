#[doc = "Register `DEV_FIBC7` reader"]
pub type R = crate::R<DevFibc7Spec>;
#[doc = "Register `DEV_FIBC7` writer"]
pub type W = crate::W<DevFibc7Spec>;
#[doc = "Field `BC_F7` reader - "]
pub type BcF7R = crate::FieldReader<u16>;
#[doc = "Field `BC_F7` writer - "]
pub type BcF7W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST7_HOV` reader - "]
pub type Ffrst7HovR = crate::BitReader;
#[doc = "Field `FFRST7_HOV` writer - "]
pub type Ffrst7HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f7(&self) -> BcF7R {
        BcF7R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst7_hov(&self) -> Ffrst7HovR {
        Ffrst7HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f7(&mut self) -> BcF7W<'_, DevFibc7Spec> {
        BcF7W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst7_hov(&mut self) -> Ffrst7HovW<'_, DevFibc7Spec> {
        Ffrst7HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 7\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc7Spec;
impl crate::RegisterSpec for DevFibc7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc7::R`](R) reader structure"]
impl crate::Readable for DevFibc7Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc7::W`](W) writer structure"]
impl crate::Writable for DevFibc7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC7 to value 0"]
impl crate::Resettable for DevFibc7Spec {}
