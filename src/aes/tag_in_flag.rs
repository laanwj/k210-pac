#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAG_IN_FLAG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `tag_in_flag`"]
pub type TAG_IN_FLAGR = super::data_in_flag::DATA_IN_FLAGR;
#[doc = "Values that can be written to the field `tag_in_flag`"]
pub type TAG_IN_FLAGW = super::data_in_flag::DATA_IN_FLAGW;
#[doc = r" Proxy"]
pub struct _TAG_IN_FLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TAG_IN_FLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAG_IN_FLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cannot input"]
    #[inline]
    pub fn cannot_input(self) -> &'a mut W {
        self.variant(super::data_in_flag::DATA_IN_FLAGW::CANNOT_INPUT)
    }
    #[doc = "Can input"]
    #[inline]
    pub fn can_input(self) -> &'a mut W {
        self.variant(super::data_in_flag::DATA_IN_FLAGW::CAN_INPUT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - GCM tag can be written to gcm_in_tag when this flag is set"]
    #[inline]
    pub fn tag_in_flag(&self) -> TAG_IN_FLAGR {
        TAG_IN_FLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GCM tag can be written to gcm_in_tag when this flag is set"]
    #[inline]
    pub fn tag_in_flag(&mut self) -> _TAG_IN_FLAGW {
        _TAG_IN_FLAGW { w: self }
    }
}
