#[doc = "Register `CODEC_BUS_THRE` reader"]
pub type R = crate::R<CodecBusThreSpec>;
#[doc = "Register `CODEC_BUS_THRE` writer"]
pub type W = crate::W<CodecBusThreSpec>;
#[doc = "Field `REG_X_WTHRE_BLAI2SYSRAM` reader - Write threshold for NPU to System RAM"]
pub type RegXWthreBlai2sysramR = crate::FieldReader;
#[doc = "Field `REG_X_WTHRE_BLAI2SYSRAM` writer - Write threshold for NPU to System RAM"]
pub type RegXWthreBlai2sysramW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_X_WTHRE_BLAI2EXT` reader - Write threshold for NPU to External Memory"]
pub type RegXWthreBlai2extR = crate::FieldReader;
#[doc = "Field `REG_X_WTHRE_BLAI2EXT` writer - Write threshold for NPU to External Memory"]
pub type RegXWthreBlai2extW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_X_WTHRE_VDO2PB` reader - Write threshold for Video to Port B"]
pub type RegXWthreVdo2pbR = crate::FieldReader;
#[doc = "Field `REG_X_WTHRE_VDO2PB` writer - Write threshold for Video to Port B"]
pub type RegXWthreVdo2pbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_X_WTHRE_VDO2PA` reader - Write threshold for Video to Port A"]
pub type RegXWthreVdo2paR = crate::FieldReader;
#[doc = "Field `REG_X_WTHRE_VDO2PA` writer - Write threshold for Video to Port A"]
pub type RegXWthreVdo2paW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_X_WTHRE_VDO2SYSRAM` reader - Write threshold for Video to System RAM"]
pub type RegXWthreVdo2sysramR = crate::FieldReader;
#[doc = "Field `REG_X_WTHRE_VDO2SYSRAM` writer - Write threshold for Video to System RAM"]
pub type RegXWthreVdo2sysramW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Write threshold for NPU to System RAM"]
    #[inline(always)]
    pub fn reg_x_wthre_blai2sysram(&self) -> RegXWthreBlai2sysramR {
        RegXWthreBlai2sysramR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Write threshold for NPU to External Memory"]
    #[inline(always)]
    pub fn reg_x_wthre_blai2ext(&self) -> RegXWthreBlai2extR {
        RegXWthreBlai2extR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Write threshold for Video to Port B"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2pb(&self) -> RegXWthreVdo2pbR {
        RegXWthreVdo2pbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write threshold for Video to Port A"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2pa(&self) -> RegXWthreVdo2paR {
        RegXWthreVdo2paR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Write threshold for Video to System RAM"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2sysram(&self) -> RegXWthreVdo2sysramR {
        RegXWthreVdo2sysramR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write threshold for NPU to System RAM"]
    #[inline(always)]
    pub fn reg_x_wthre_blai2sysram(&mut self) -> RegXWthreBlai2sysramW<'_, CodecBusThreSpec> {
        RegXWthreBlai2sysramW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Write threshold for NPU to External Memory"]
    #[inline(always)]
    pub fn reg_x_wthre_blai2ext(&mut self) -> RegXWthreBlai2extW<'_, CodecBusThreSpec> {
        RegXWthreBlai2extW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Write threshold for Video to Port B"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2pb(&mut self) -> RegXWthreVdo2pbW<'_, CodecBusThreSpec> {
        RegXWthreVdo2pbW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Write threshold for Video to Port A"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2pa(&mut self) -> RegXWthreVdo2paW<'_, CodecBusThreSpec> {
        RegXWthreVdo2paW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Write threshold for Video to System RAM"]
    #[inline(always)]
    pub fn reg_x_wthre_vdo2sysram(&mut self) -> RegXWthreVdo2sysramW<'_, CodecBusThreSpec> {
        RegXWthreVdo2sysramW::new(self, 8)
    }
}
#[doc = "Codec Bus Thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_thre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_bus_thre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecBusThreSpec;
impl crate::RegisterSpec for CodecBusThreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_bus_thre::R`](R) reader structure"]
impl crate::Readable for CodecBusThreSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_bus_thre::W`](W) writer structure"]
impl crate::Writable for CodecBusThreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEC_BUS_THRE to value 0"]
impl crate::Resettable for CodecBusThreSpec {}
