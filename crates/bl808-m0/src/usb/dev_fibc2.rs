#[doc = "Register `DEV_FIBC2` reader"]
pub type R = crate::R<DevFibc2Spec>;
#[doc = "Register `DEV_FIBC2` writer"]
pub type W = crate::W<DevFibc2Spec>;
#[doc = "Field `BC_F2` reader - "]
pub type BcF2R = crate::FieldReader<u16>;
#[doc = "Field `BC_F2` writer - "]
pub type BcF2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST2_HOV` reader - "]
pub type Ffrst2HovR = crate::BitReader;
#[doc = "Field `FFRST2_HOV` writer - "]
pub type Ffrst2HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f2(&self) -> BcF2R {
        BcF2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst2_hov(&self) -> Ffrst2HovR {
        Ffrst2HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn bc_f2(&mut self) -> BcF2W<'_, DevFibc2Spec> {
        BcF2W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ffrst2_hov(&mut self) -> Ffrst2HovW<'_, DevFibc2Spec> {
        Ffrst2HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc2Spec;
impl crate::RegisterSpec for DevFibc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc2::R`](R) reader structure"]
impl crate::Readable for DevFibc2Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc2::W`](W) writer structure"]
impl crate::Writable for DevFibc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC2 to value 0"]
impl crate::Resettable for DevFibc2Spec {}
