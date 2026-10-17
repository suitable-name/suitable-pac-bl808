#[doc = "Register `DEV_FIBC3` reader"]
pub type R = crate::R<DevFibc3Spec>;
#[doc = "Register `DEV_FIBC3` writer"]
pub type W = crate::W<DevFibc3Spec>;
#[doc = "Field `BC_F3` reader - "]
pub type BcF3R = crate::FieldReader<u16>;
#[doc = "Field `BC_F3` writer - "]
pub type BcF3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST3_HOV` reader - "]
pub type Ffrst3HovR = crate::BitReader;
#[doc = "Field `FFRST3_HOV` writer - "]
pub type Ffrst3HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f3(&self) -> BcF3R {
        BcF3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst3_hov(&self) -> Ffrst3HovR {
        Ffrst3HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f3(&mut self) -> BcF3W<'_, DevFibc3Spec> {
        BcF3W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst3_hov(&mut self) -> Ffrst3HovW<'_, DevFibc3Spec> {
        Ffrst3HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc3Spec;
impl crate::RegisterSpec for DevFibc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc3::R`](R) reader structure"]
impl crate::Readable for DevFibc3Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc3::W`](W) writer structure"]
impl crate::Writable for DevFibc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC3 to value 0"]
impl crate::Resettable for DevFibc3Spec {}
