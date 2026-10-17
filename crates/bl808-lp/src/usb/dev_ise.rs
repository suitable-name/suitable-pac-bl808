#[doc = "Register `DEV_ISE` reader"]
pub type R = crate::R<DevIseSpec>;
#[doc = "Field `ISO_ABT_ERR_EP1` reader - "]
pub type IsoAbtErrEp1R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP2` reader - "]
pub type IsoAbtErrEp2R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP3` reader - "]
pub type IsoAbtErrEp3R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP4` reader - "]
pub type IsoAbtErrEp4R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP5` reader - "]
pub type IsoAbtErrEp5R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP6` reader - "]
pub type IsoAbtErrEp6R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP7` reader - "]
pub type IsoAbtErrEp7R = crate::BitReader;
#[doc = "Field `ISO_ABT_ERR_EP8` reader - "]
pub type IsoAbtErrEp8R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP1` reader - "]
pub type IsoSeqErrEp1R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP2` reader - "]
pub type IsoSeqErrEp2R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP3` reader - "]
pub type IsoSeqErrEp3R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP4` reader - "]
pub type IsoSeqErrEp4R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP5` reader - "]
pub type IsoSeqErrEp5R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP6` reader - "]
pub type IsoSeqErrEp6R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP7` reader - "]
pub type IsoSeqErrEp7R = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_EP8` reader - "]
pub type IsoSeqErrEp8R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn iso_abt_err_ep1(&self) -> IsoAbtErrEp1R {
        IsoAbtErrEp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn iso_abt_err_ep2(&self) -> IsoAbtErrEp2R {
        IsoAbtErrEp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn iso_abt_err_ep3(&self) -> IsoAbtErrEp3R {
        IsoAbtErrEp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iso_abt_err_ep4(&self) -> IsoAbtErrEp4R {
        IsoAbtErrEp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iso_abt_err_ep5(&self) -> IsoAbtErrEp5R {
        IsoAbtErrEp5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iso_abt_err_ep6(&self) -> IsoAbtErrEp6R {
        IsoAbtErrEp6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iso_abt_err_ep7(&self) -> IsoAbtErrEp7R {
        IsoAbtErrEp7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iso_abt_err_ep8(&self) -> IsoAbtErrEp8R {
        IsoAbtErrEp8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn iso_seq_err_ep1(&self) -> IsoSeqErrEp1R {
        IsoSeqErrEp1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn iso_seq_err_ep2(&self) -> IsoSeqErrEp2R {
        IsoSeqErrEp2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn iso_seq_err_ep3(&self) -> IsoSeqErrEp3R {
        IsoSeqErrEp3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn iso_seq_err_ep4(&self) -> IsoSeqErrEp4R {
        IsoSeqErrEp4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn iso_seq_err_ep5(&self) -> IsoSeqErrEp5R {
        IsoSeqErrEp5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn iso_seq_err_ep6(&self) -> IsoSeqErrEp6R {
        IsoSeqErrEp6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn iso_seq_err_ep7(&self) -> IsoSeqErrEp7R {
        IsoSeqErrEp7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn iso_seq_err_ep8(&self) -> IsoSeqErrEp8R {
        IsoSeqErrEp8R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Device Isochronous Sequence Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ise::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIseSpec;
impl crate::RegisterSpec for DevIseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_ise::R`](R) reader structure"]
impl crate::Readable for DevIseSpec {}
#[doc = "`reset()` method sets DEV_ISE to value 0"]
impl crate::Resettable for DevIseSpec {}
