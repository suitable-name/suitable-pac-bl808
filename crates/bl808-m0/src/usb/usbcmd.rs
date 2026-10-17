#[doc = "Register `USBCMD` reader"]
pub type R = crate::R<UsbcmdSpec>;
#[doc = "Register `USBCMD` writer"]
pub type W = crate::W<UsbcmdSpec>;
#[doc = "Field `RS` reader - "]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - "]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HC_RESET` reader - "]
pub type HcResetR = crate::BitReader;
#[doc = "Field `HC_RESET` writer - "]
pub type HcResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRL_SIZE` reader - Frame List Size"]
pub type FrlSizeR = crate::FieldReader;
#[doc = "Field `FRL_SIZE` writer - Frame List Size"]
pub type FrlSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSCH_EN` reader - Periodic Schedule Enable"]
pub type PschEnR = crate::BitReader;
#[doc = "Field `PSCH_EN` writer - Periodic Schedule Enable"]
pub type PschEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASCH_EN` reader - Asynchronous Schedule Enable"]
pub type AschEnR = crate::BitReader;
#[doc = "Field `ASCH_EN` writer - Asynchronous Schedule Enable"]
pub type AschEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_OAAD` reader - Interrupt On Asynchronous Advance Doorbell"]
pub type IntOaadR = crate::BitReader;
#[doc = "Field `INT_OAAD` writer - Interrupt On Asynchronous Advance Doorbell"]
pub type IntOaadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYN_PK_CNT` reader - Asynchronous Park Count"]
pub type AsynPkCntR = crate::FieldReader;
#[doc = "Field `ASYN_PK_CNT` writer - Asynchronous Park Count"]
pub type AsynPkCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ASYN_PK_EN` reader - Asynchronous Park Enable"]
pub type AsynPkEnR = crate::BitReader;
#[doc = "Field `ASYN_PK_EN` writer - Asynchronous Park Enable"]
pub type AsynPkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_THRC` reader - Interrupt Threshold Control"]
pub type IntThrcR = crate::FieldReader;
#[doc = "Field `INT_THRC` writer - Interrupt Threshold Control"]
pub type IntThrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hc_reset(&self) -> HcResetR {
        HcResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size"]
    #[inline(always)]
    pub fn frl_size(&self) -> FrlSizeR {
        FrlSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn psch_en(&self) -> PschEnR {
        PschEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn asch_en(&self) -> AschEnR {
        AschEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt On Asynchronous Advance Doorbell"]
    #[inline(always)]
    pub fn int_oaad(&self) -> IntOaadR {
        IntOaadR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronous Park Count"]
    #[inline(always)]
    pub fn asyn_pk_cnt(&self) -> AsynPkCntR {
        AsynPkCntR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Asynchronous Park Enable"]
    #[inline(always)]
    pub fn asyn_pk_en(&self) -> AsynPkEnR {
        AsynPkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn int_thrc(&self) -> IntThrcR {
        IntThrcR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, UsbcmdSpec> {
        RsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hc_reset(&mut self) -> HcResetW<'_, UsbcmdSpec> {
        HcResetW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Frame List Size"]
    #[inline(always)]
    pub fn frl_size(&mut self) -> FrlSizeW<'_, UsbcmdSpec> {
        FrlSizeW::new(self, 2)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn psch_en(&mut self) -> PschEnW<'_, UsbcmdSpec> {
        PschEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn asch_en(&mut self) -> AschEnW<'_, UsbcmdSpec> {
        AschEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt On Asynchronous Advance Doorbell"]
    #[inline(always)]
    pub fn int_oaad(&mut self) -> IntOaadW<'_, UsbcmdSpec> {
        IntOaadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Asynchronous Park Count"]
    #[inline(always)]
    pub fn asyn_pk_cnt(&mut self) -> AsynPkCntW<'_, UsbcmdSpec> {
        AsynPkCntW::new(self, 8)
    }
    #[doc = "Bit 11 - Asynchronous Park Enable"]
    #[inline(always)]
    pub fn asyn_pk_en(&mut self) -> AsynPkEnW<'_, UsbcmdSpec> {
        AsynPkEnW::new(self, 11)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn int_thrc(&mut self) -> IntThrcW<'_, UsbcmdSpec> {
        IntThrcW::new(self, 16)
    }
}
#[doc = "USB Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbcmdSpec;
impl crate::RegisterSpec for UsbcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcmd::R`](R) reader structure"]
impl crate::Readable for UsbcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`usbcmd::W`](W) writer structure"]
impl crate::Writable for UsbcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCMD to value 0"]
impl crate::Resettable for UsbcmdSpec {}
