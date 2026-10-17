#[doc = "Register `DEV_FIBC6` reader"]
pub type R = crate::R<DevFibc6Spec>;
#[doc = "Register `DEV_FIBC6` writer"]
pub type W = crate::W<DevFibc6Spec>;
#[doc = "Field `BC_F6` reader - "]
pub type BcF6R = crate::FieldReader<u16>;
#[doc = "Field `BC_F6` writer - "]
pub type BcF6W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST6_HOV` reader - "]
pub type Ffrst6HovR = crate::BitReader;
#[doc = "Field `FFRST6_HOV` writer - "]
pub type Ffrst6HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f6(&self) -> BcF6R {
        BcF6R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst6_hov(&self) -> Ffrst6HovR {
        Ffrst6HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f6(&mut self) -> BcF6W<'_, DevFibc6Spec> {
        BcF6W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst6_hov(&mut self) -> Ffrst6HovW<'_, DevFibc6Spec> {
        Ffrst6HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 6\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc6Spec;
impl crate::RegisterSpec for DevFibc6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc6::R`](R) reader structure"]
impl crate::Readable for DevFibc6Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc6::W`](W) writer structure"]
impl crate::Writable for DevFibc6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC6 to value 0"]
impl crate::Resettable for DevFibc6Spec {}
