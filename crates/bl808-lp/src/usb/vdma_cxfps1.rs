#[doc = "Register `VDMA_CXFPS1` reader"]
pub type R = crate::R<VdmaCxfps1Spec>;
#[doc = "Register `VDMA_CXFPS1` writer"]
pub type W = crate::W<VdmaCxfps1Spec>;
#[doc = "Field `VDMA_START_CXF` reader - "]
pub type VdmaStartCxfR = crate::BitReader;
#[doc = "Field `VDMA_START_CXF` writer - "]
pub type VdmaStartCxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_TYPE_CXF` reader - "]
pub type VdmaTypeCxfR = crate::BitReader;
#[doc = "Field `VDMA_TYPE_CXF` writer - "]
pub type VdmaTypeCxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_IO_CXF` reader - VDMA Direction (0:OUT/Write, 1:IN/Read)"]
pub type VdmaIoCxfR = crate::BitReader;
#[doc = "Field `VDMA_IO_CXF` writer - VDMA Direction (0:OUT/Write, 1:IN/Read)"]
pub type VdmaIoCxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_LEN_CXF` reader - "]
pub type VdmaLenCxfR = crate::FieldReader<u32>;
#[doc = "Field `VDMA_LEN_CXF` writer - "]
pub type VdmaLenCxfW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_cxf(&self) -> VdmaStartCxfR {
        VdmaStartCxfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_cxf(&self) -> VdmaTypeCxfR {
        VdmaTypeCxfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDMA Direction (0:OUT/Write, 1:IN/Read)"]
    #[inline(always)]
    pub fn vdma_io_cxf(&self) -> VdmaIoCxfR {
        VdmaIoCxfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_cxf(&self) -> VdmaLenCxfR {
        VdmaLenCxfR::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_cxf(&mut self) -> VdmaStartCxfW<'_, VdmaCxfps1Spec> {
        VdmaStartCxfW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_cxf(&mut self) -> VdmaTypeCxfW<'_, VdmaCxfps1Spec> {
        VdmaTypeCxfW::new(self, 1)
    }
    #[doc = "Bit 2 - VDMA Direction (0:OUT/Write, 1:IN/Read)"]
    #[inline(always)]
    pub fn vdma_io_cxf(&mut self) -> VdmaIoCxfW<'_, VdmaCxfps1Spec> {
        VdmaIoCxfW::new(self, 2)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_cxf(&mut self) -> VdmaLenCxfW<'_, VdmaCxfps1Spec> {
        VdmaLenCxfW::new(self, 8)
    }
}
#[doc = "VDMA Control Endpoint FIFO Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_cxfps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_cxfps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaCxfps1Spec;
impl crate::RegisterSpec for VdmaCxfps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_cxfps1::R`](R) reader structure"]
impl crate::Readable for VdmaCxfps1Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_cxfps1::W`](W) writer structure"]
impl crate::Writable for VdmaCxfps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_CXFPS1 to value 0"]
impl crate::Resettable for VdmaCxfps1Spec {}
