#[doc = "Register `VDO_H264_BS_FULL_THR` reader"]
pub type R = crate::R<VdoH264BsFullThrSpec>;
#[doc = "Register `VDO_H264_BS_FULL_THR` writer"]
pub type W = crate::W<VdoH264BsFullThrSpec>;
#[doc = "Field `cfg_bs_full_thr` reader - "]
pub type CfgBsFullThrR = crate::FieldReader<u32>;
#[doc = "Field `cfg_bs_full_thr` writer - "]
pub type CfgBsFullThrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_bs_full_thr(&self) -> CfgBsFullThrR {
        CfgBsFullThrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_bs_full_thr(&mut self) -> CfgBsFullThrW<'_, VdoH264BsFullThrSpec> {
        CfgBsFullThrW::new(self, 0)
    }
}
#[doc = "Main-stream bitstream buffer full threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_bs_full_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_bs_full_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264BsFullThrSpec;
impl crate::RegisterSpec for VdoH264BsFullThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_bs_full_thr::R`](R) reader structure"]
impl crate::Readable for VdoH264BsFullThrSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_bs_full_thr::W`](W) writer structure"]
impl crate::Writable for VdoH264BsFullThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_BS_FULL_THR to value 0"]
impl crate::Resettable for VdoH264BsFullThrSpec {}
