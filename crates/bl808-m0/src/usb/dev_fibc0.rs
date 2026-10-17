#[doc = "Register `DEV_FIBC0` reader"]
pub type R = crate::R<DevFibc0Spec>;
#[doc = "Register `DEV_FIBC0` writer"]
pub type W = crate::W<DevFibc0Spec>;
#[doc = "Field `BC_F0` reader - Byte count for FIFO 0"]
pub type BcF0R = crate::FieldReader<u16>;
#[doc = "Field `BC_F0` writer - Byte count for FIFO 0"]
pub type BcF0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FFRST0_HOV` reader - Force FIFO Reset 0 (Host Overwrite)"]
pub type Ffrst0HovR = crate::BitReader;
#[doc = "Field `FFRST0_HOV` writer - Force FIFO Reset 0 (Host Overwrite)"]
pub type Ffrst0HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Byte count for FIFO 0"]
    #[inline(always)]
    pub fn bc_f0(&self) -> BcF0R {
        BcF0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Force FIFO Reset 0 (Host Overwrite)"]
    #[inline(always)]
    pub fn ffrst0_hov(&self) -> Ffrst0HovR {
        Ffrst0HovR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Byte count for FIFO 0"]
    #[inline(always)]
    pub fn bc_f0(&mut self) -> BcF0W<'_, DevFibc0Spec> {
        BcF0W::new(self, 0)
    }
    #[doc = "Bit 12 - Force FIFO Reset 0 (Host Overwrite)"]
    #[inline(always)]
    pub fn ffrst0_hov(&mut self) -> Ffrst0HovW<'_, DevFibc0Spec> {
        Ffrst0HovW::new(self, 12)
    }
}
#[doc = "Device FIFO Byte Count 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFibc0Spec;
impl crate::RegisterSpec for DevFibc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fibc0::R`](R) reader structure"]
impl crate::Readable for DevFibc0Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fibc0::W`](W) writer structure"]
impl crate::Writable for DevFibc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FIBC0 to value 0"]
impl crate::Resettable for DevFibc0Spec {}
