#[doc = "Register `VDO_H264_BS_SIZE` reader"]
pub type R = crate::R<VdoH264BsSizeSpec>;
#[doc = "Register `VDO_H264_BS_SIZE` writer"]
pub type W = crate::W<VdoH264BsSizeSpec>;
#[doc = "Field `cfg_bs_size` reader - "]
pub type CfgBsSizeR = crate::FieldReader<u32>;
#[doc = "Field `cfg_bs_size` writer - "]
pub type CfgBsSizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_bs_size(&self) -> CfgBsSizeR {
        CfgBsSizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_bs_size(&mut self) -> CfgBsSizeW<'_, VdoH264BsSizeSpec> {
        CfgBsSizeW::new(self, 0)
    }
}
#[doc = "Main-stream bitstream output buffer size (bytes)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_bs_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_bs_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264BsSizeSpec;
impl crate::RegisterSpec for VdoH264BsSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_bs_size::R`](R) reader structure"]
impl crate::Readable for VdoH264BsSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_bs_size::W`](W) writer structure"]
impl crate::Writable for VdoH264BsSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_BS_SIZE to value 0"]
impl crate::Resettable for VdoH264BsSizeSpec {}
