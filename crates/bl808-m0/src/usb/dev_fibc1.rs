#[doc = "Register `DEV_FIBC1` reader"]
pub type R = crate::R<DevFibc1Spec>;
#[doc = "Register `DEV_FIBC1` writer"]
pub type W = crate::W<DevFibc1Spec>;
#[doc = "Field `BC_F1` reader - Byte count for FIFO 1"]
pub type BcF1R = crate::FieldReader<u16>;
#[doc = "Field `BC_F1` writer - Byte count for FIFO 1"]
pub type BcF1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST1_HOV` reader - Force FIFO Reset 1 (Host Overwrite)"]
pub type Ffrst1HovR = crate::BitReader;
#[doc = "Field `FFRST1_HOV` writer - Force FIFO Reset 1 (Host Overwrite)"]
pub type Ffrst1HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Byte count for FIFO 1"]
    #[inline(always)]
    pub fn bc_f1(&self) -> BcF1R {
        BcF1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Force FIFO Reset 1 (Host Overwrite)"]
    #[inline(always)]
    pub fn ffrst1_hov(&self) -> Ffrst1HovR {
        Ffrst1HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Byte count for FIFO 1"]
    #[inline(always)]
    pub fn bc_f1(&mut self) -> BcF1W<'_, DevFibc1Spec> {
        BcF1W::new(self, 0)
    }
    #[doc = "Bit 12 - Force FIFO Reset 1 (Host Overwrite)"]
    #[inline(always)]
    pub fn ffrst1_hov(&mut self) -> Ffrst1HovW<'_, DevFibc1Spec> {
        Ffrst1HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc1Spec;
impl crate::RegisterSpec for DevFibc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc1::R`](R) reader structure"]
impl crate::Readable for DevFibc1Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc1::W`](W) writer structure"]
impl crate::Writable for DevFibc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC1 to value 0"]
impl crate::Resettable for DevFibc1Spec {}
