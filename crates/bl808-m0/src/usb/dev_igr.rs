#[doc = "Register `DEV_IGR` reader"]
pub type R = crate::R<DevIgrSpec>;
#[doc = "Field `INT_G0` reader - "]
pub type IntG0R = crate::BitReader;
#[doc = "Field `INT_G1` reader - "]
pub type IntG1R = crate::BitReader;
#[doc = "Field `INT_G2_HOV` reader - "]
pub type IntG2HovR = crate::BitReader;
#[doc = "Field `INT_G3` reader - "]
pub type IntG3R = crate::BitReader;
#[doc = "Field `INT_G4` reader - "]
pub type IntG4R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int_g0(&self) -> IntG0R {
        IntG0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int_g1(&self) -> IntG1R {
        IntG1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn int_g2_hov(&self) -> IntG2HovR {
        IntG2HovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int_g3(&self) -> IntG3R {
        IntG3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn int_g4(&self) -> IntG4R {
        IntG4R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Device Interrupt Group Register (Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_igr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIgrSpec;
impl crate::RegisterSpec for DevIgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_igr::R`](R) reader structure"]
impl crate::Readable for DevIgrSpec {}
#[doc = "`reset()` method sets DEV_IGR to value 0"]
impl crate::Resettable for DevIgrSpec {}
