#[doc = "Register `VDO_H264_ROI%s` reader"]
pub type R = crate::R<VdoH264RoiSpec>;
#[doc = "Register `VDO_H264_ROI%s` writer"]
pub type W = crate::W<VdoH264RoiSpec>;
#[doc = "Field `mbx_s` reader - Start macroblock column (X)"]
pub type MbxSR = crate::FieldReader;
#[doc = "Field `mbx_s` writer - Start macroblock column (X)"]
pub type MbxSW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `mbx_e` reader - End macroblock column (X)"]
pub type MbxER = crate::FieldReader;
#[doc = "Field `mbx_e` writer - End macroblock column (X)"]
pub type MbxEW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `mby_s` reader - Start macroblock row (Y)"]
pub type MbySR = crate::FieldReader;
#[doc = "Field `mby_s` writer - Start macroblock row (Y)"]
pub type MbySW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `mby_e` reader - End macroblock row (Y)"]
pub type MbyER = crate::FieldReader;
#[doc = "Field `mby_e` writer - End macroblock row (Y)"]
pub type MbyEW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Start macroblock column (X)"]
    #[inline(always)]
    pub fn mbx_s(&self) -> MbxSR {
        MbxSR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - End macroblock column (X)"]
    #[inline(always)]
    pub fn mbx_e(&self) -> MbxER {
        MbxER::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Start macroblock row (Y)"]
    #[inline(always)]
    pub fn mby_s(&self) -> MbySR {
        MbySR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - End macroblock row (Y)"]
    #[inline(always)]
    pub fn mby_e(&self) -> MbyER {
        MbyER::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Start macroblock column (X)"]
    #[inline(always)]
    pub fn mbx_s(&mut self) -> MbxSW<'_, VdoH264RoiSpec> {
        MbxSW::new(self, 0)
    }
    #[doc = "Bits 8:14 - End macroblock column (X)"]
    #[inline(always)]
    pub fn mbx_e(&mut self) -> MbxEW<'_, VdoH264RoiSpec> {
        MbxEW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Start macroblock row (Y)"]
    #[inline(always)]
    pub fn mby_s(&mut self) -> MbySW<'_, VdoH264RoiSpec> {
        MbySW::new(self, 16)
    }
    #[doc = "Bits 24:30 - End macroblock row (Y)"]
    #[inline(always)]
    pub fn mby_e(&mut self) -> MbyEW<'_, VdoH264RoiSpec> {
        MbyEW::new(self, 24)
    }
}
#[doc = "ROI %s macroblock coordinate bounds (MBX_S/E, MBY_S/E)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_roi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_roi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264RoiSpec;
impl crate::RegisterSpec for VdoH264RoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_roi::R`](R) reader structure"]
impl crate::Readable for VdoH264RoiSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_roi::W`](W) writer structure"]
impl crate::Writable for VdoH264RoiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_ROI%s to value 0"]
impl crate::Resettable for VdoH264RoiSpec {}
