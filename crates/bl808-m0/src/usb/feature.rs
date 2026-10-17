#[doc = "Register `FEATURE` reader"]
pub type R = crate::R<FeatureSpec>;
#[doc = "Field `DMABUFSIZE` reader - DMA buffer size capability"]
pub type DmabufsizeR = crate::FieldReader;
#[doc = "Field `FIFO_NUM` reader - Number of dedicated FIFOs"]
pub type FifoNumR = crate::FieldReader;
#[doc = "Field `EP_NUM` reader - Number of endpoints supported (minus EP0)"]
pub type EpNumR = crate::FieldReader;
#[doc = "Field `DEV_ONLY` reader - Device Only mode capability"]
pub type DevOnlyR = crate::BitReader;
#[doc = "Field `HOST_ONLY` reader - Host Only mode capability"]
pub type HostOnlyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - DMA buffer size capability"]
    #[inline(always)]
    pub fn dmabufsize(&self) -> DmabufsizeR {
        DmabufsizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of dedicated FIFOs"]
    #[inline(always)]
    pub fn fifo_num(&self) -> FifoNumR {
        FifoNumR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of endpoints supported (minus EP0)"]
    #[inline(always)]
    pub fn ep_num(&self) -> EpNumR {
        EpNumR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Device Only mode capability"]
    #[inline(always)]
    pub fn dev_only(&self) -> DevOnlyR {
        DevOnlyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Host Only mode capability"]
    #[inline(always)]
    pub fn host_only(&self) -> HostOnlyR {
        HostOnlyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "USB Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`feature::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeatureSpec;
impl crate::RegisterSpec for FeatureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feature::R`](R) reader structure"]
impl crate::Readable for FeatureSpec {}
#[doc = "`reset()` method sets FEATURE to value 0"]
impl crate::Resettable for FeatureSpec {}
