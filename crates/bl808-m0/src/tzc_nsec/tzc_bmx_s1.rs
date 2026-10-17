#[doc = "Register `TZC_BMX_S1` reader"]
pub type R = crate::R<TzcBmxS1Spec>;
#[doc = "Field `tzc_bmx_s10_tzsid_en` reader - AHB Slave 0x10 TZSID (UART0)"]
pub type TzcBmxS10TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s11_tzsid_en` reader - AHB Slave 0x11 TZSID (UART1)"]
pub type TzcBmxS11TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s12_tzsid_en` reader - AHB Slave 0x12 TZSID (SPI)"]
pub type TzcBmxS12TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s13_tzsid_en` reader - AHB Slave 0x13 TZSID (I2C0)"]
pub type TzcBmxS13TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s14_tzsid_en` reader - AHB Slave 0x14 TZSID (PWM)"]
pub type TzcBmxS14TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s15_tzsid_en` reader - AHB Slave 0x15 TZSID (TIMER0)"]
pub type TzcBmxS15TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s16_tzsid_en` reader - AHB Slave 0x16 TZSID (IR)"]
pub type TzcBmxS16TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s17_tzsid_en` reader - AHB Slave 0x17 TZSID (CKS)"]
pub type TzcBmxS17TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s18_tzsid_en` reader - AHB Slave 0x18 TZSID (QDEC)"]
pub type TzcBmxS18TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s19_tzsid_en` reader - AHB Slave 0x19 TZSID (KYS)"]
pub type TzcBmxS19TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s1a_tzsid_en` reader - AHB Slave 0x1A TZSID (UART2)"]
pub type TzcBmxS1aTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s1b_tzsid_en` reader - "]
pub type TzcBmxS1bTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s1c_tzsid_en` reader - "]
pub type TzcBmxS1cTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s1d_tzsid_en` reader - "]
pub type TzcBmxS1dTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s1e_tzsid_en` reader - "]
pub type TzcBmxS1eTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s1f_tzsid_en` reader - "]
pub type TzcBmxS1fTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - AHB Slave 0x10 TZSID (UART0)"]
    #[inline(always)]
    pub fn tzc_bmx_s10_tzsid_en(&self) -> TzcBmxS10TzsidEnR {
        TzcBmxS10TzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AHB Slave 0x11 TZSID (UART1)"]
    #[inline(always)]
    pub fn tzc_bmx_s11_tzsid_en(&self) -> TzcBmxS11TzsidEnR {
        TzcBmxS11TzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - AHB Slave 0x12 TZSID (SPI)"]
    #[inline(always)]
    pub fn tzc_bmx_s12_tzsid_en(&self) -> TzcBmxS12TzsidEnR {
        TzcBmxS12TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - AHB Slave 0x13 TZSID (I2C0)"]
    #[inline(always)]
    pub fn tzc_bmx_s13_tzsid_en(&self) -> TzcBmxS13TzsidEnR {
        TzcBmxS13TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - AHB Slave 0x14 TZSID (PWM)"]
    #[inline(always)]
    pub fn tzc_bmx_s14_tzsid_en(&self) -> TzcBmxS14TzsidEnR {
        TzcBmxS14TzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - AHB Slave 0x15 TZSID (TIMER0)"]
    #[inline(always)]
    pub fn tzc_bmx_s15_tzsid_en(&self) -> TzcBmxS15TzsidEnR {
        TzcBmxS15TzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - AHB Slave 0x16 TZSID (IR)"]
    #[inline(always)]
    pub fn tzc_bmx_s16_tzsid_en(&self) -> TzcBmxS16TzsidEnR {
        TzcBmxS16TzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - AHB Slave 0x17 TZSID (CKS)"]
    #[inline(always)]
    pub fn tzc_bmx_s17_tzsid_en(&self) -> TzcBmxS17TzsidEnR {
        TzcBmxS17TzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - AHB Slave 0x18 TZSID (QDEC)"]
    #[inline(always)]
    pub fn tzc_bmx_s18_tzsid_en(&self) -> TzcBmxS18TzsidEnR {
        TzcBmxS18TzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - AHB Slave 0x19 TZSID (KYS)"]
    #[inline(always)]
    pub fn tzc_bmx_s19_tzsid_en(&self) -> TzcBmxS19TzsidEnR {
        TzcBmxS19TzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - AHB Slave 0x1A TZSID (UART2)"]
    #[inline(always)]
    pub fn tzc_bmx_s1a_tzsid_en(&self) -> TzcBmxS1aTzsidEnR {
        TzcBmxS1aTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_bmx_s1b_tzsid_en(&self) -> TzcBmxS1bTzsidEnR {
        TzcBmxS1bTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_bmx_s1c_tzsid_en(&self) -> TzcBmxS1cTzsidEnR {
        TzcBmxS1cTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn tzc_bmx_s1d_tzsid_en(&self) -> TzcBmxS1dTzsidEnR {
        TzcBmxS1dTzsidEnR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tzc_bmx_s1e_tzsid_en(&self) -> TzcBmxS1eTzsidEnR {
        TzcBmxS1eTzsidEnR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn tzc_bmx_s1f_tzsid_en(&self) -> TzcBmxS1fTzsidEnR {
        TzcBmxS1fTzsidEnR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "BMX Slave 1 Security ID Enable (TZSID for AHB Slaves 0x10 - 0x1F)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxS1Spec;
impl crate::RegisterSpec for TzcBmxS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_s1::R`](R) reader structure"]
impl crate::Readable for TzcBmxS1Spec {}
#[doc = "`reset()` method sets TZC_BMX_S1 to value 0"]
impl crate::Resettable for TzcBmxS1Spec {}
