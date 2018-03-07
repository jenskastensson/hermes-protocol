use ::Result;
use failure::ResultExt;
use ffi_utils::{AsRust, CArrayString, CReprOf, RawBorrow, RawPointerConverter};
use hermes;
use libc;
use snips_nlu_ontology_ffi::{CIntentClassifierResult, CSlot, CSlotList};
use std::ffi::CString;
use std::ptr::null;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct CSiteMessage {
    pub site_id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CSiteMessage {}

impl CSiteMessage {
    pub fn from(input: hermes::SiteMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SiteMessage> for CSiteMessage {
    fn c_repr_of(input: hermes::SiteMessage) -> Result<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SiteMessage> for CSiteMessage {
    fn as_rust(&self) -> Result<hermes::SiteMessage> {
        Ok(hermes::SiteMessage {
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CSiteMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CHotwordDetectedMessage {
    pub site_id: *const libc::c_char,
    pub model_id: *const libc::c_char,
}

unsafe impl Sync for CHotwordDetectedMessage {}

impl CReprOf<hermes::HotwordDetectedMessage> for CHotwordDetectedMessage {
    fn c_repr_of(input: hermes::HotwordDetectedMessage) -> Result<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            model_id: convert_to_c_string!(input.model_id),
        })
    }
}

impl AsRust<hermes::HotwordDetectedMessage> for CHotwordDetectedMessage {
    fn as_rust(&self) -> Result<hermes::HotwordDetectedMessage> {
        Ok(hermes::HotwordDetectedMessage {
            site_id: create_rust_string_from!(self.site_id),
            model_id: create_rust_string_from!(self.model_id),
        })
    }
}

impl Drop for CHotwordDetectedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.model_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CTextCapturedMessage {
    pub text: *const libc::c_char,
    pub likelihood: f32,
    pub seconds: f32,
    pub site_id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CTextCapturedMessage {}

impl CTextCapturedMessage {
    pub fn from(input: hermes::TextCapturedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::TextCapturedMessage> for CTextCapturedMessage {
    fn c_repr_of(input: hermes::TextCapturedMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            likelihood: input.likelihood,
            seconds: input.seconds,
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::TextCapturedMessage> for CTextCapturedMessage {
    fn as_rust(&self) -> Result<hermes::TextCapturedMessage> {
        Ok(hermes::TextCapturedMessage {
            text: create_rust_string_from!(self.text),
            likelihood: self.likelihood,
            seconds: self.seconds,
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CTextCapturedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluQueryMessage {
    pub input: *const libc::c_char,
    // Nullable
    pub intent_filter: *const CArrayString,
    // Nullable
    pub id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluQueryMessage {}

impl CNluQueryMessage {
    pub fn from(input: hermes::NluQueryMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluQueryMessage> for CNluQueryMessage {
    fn c_repr_of(input: hermes::NluQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            intent_filter: convert_to_nullable_c_array_string!(input.intent_filter),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluQueryMessage> for CNluQueryMessage {
    fn as_rust(&self) -> Result<hermes::NluQueryMessage> {
        Ok(hermes::NluQueryMessage {
            input: create_rust_string_from!(self.input),
            intent_filter: create_optional_rust_vec_string_from!(self.intent_filter),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}


impl Drop for CNluQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_array_string!(self.intent_filter);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotQueryMessage {
    pub input: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    pub slot_name: *const libc::c_char,
    // Nullable
    pub id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotQueryMessage {}

impl CNluSlotQueryMessage {
    pub fn from(input: hermes::NluSlotQueryMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluSlotQueryMessage> for CNluSlotQueryMessage {
    fn c_repr_of(input: hermes::NluSlotQueryMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            intent_name: convert_to_c_string!(input.intent_name),
            slot_name: convert_to_c_string!(input.slot_name),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluSlotQueryMessage> for CNluSlotQueryMessage {
    fn as_rust(&self) -> Result<hermes::NluSlotQueryMessage> {
        Ok(hermes::NluSlotQueryMessage {
            input: create_rust_string_from!(self.input),
            intent_name: create_rust_string_from!(self.intent_name),
            slot_name: create_rust_string_from!(self.slot_name),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}


impl Drop for CNluSlotQueryMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_c_string!(self.intent_name);
        take_back_c_string!(self.slot_name);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayBytesMessage {
    pub id: *const libc::c_char,
    pub wav_bytes: *const u8,
    // Note: we can't use `libc::size_t` because it's not supported by JNA
    pub wav_bytes_len: libc::c_int,
    pub site_id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CPlayBytesMessage {}

impl CPlayBytesMessage {
    pub fn from(input: hermes::PlayBytesMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::PlayBytesMessage> for CPlayBytesMessage {
    fn c_repr_of(input: hermes::PlayBytesMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            wav_bytes_len: input.wav_bytes.len() as libc::c_int,
            wav_bytes: Box::into_raw(input.wav_bytes.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::PlayBytesMessage> for CPlayBytesMessage {
    fn as_rust(&self) -> Result<hermes::PlayBytesMessage> {
        Ok(hermes::PlayBytesMessage {
            id: create_rust_string_from!(self.id),
            wav_bytes: unsafe {
                slice::from_raw_parts(self.wav_bytes as *const u8,
                                      self.wav_bytes_len as usize)
            }.to_vec(),
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CPlayBytesMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.wav_bytes as *mut u8,
                self.wav_bytes_len as usize,
            ))
        };
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAudioFrameMessage {
    pub wav_frame: *const u8,
    // Note: we can't use `libc::size_t` because it's not supported by JNA
    pub wav_frame_len: libc::c_int,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CAudioFrameMessage {}

impl CAudioFrameMessage {
    pub fn from(input: hermes::AudioFrameMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::AudioFrameMessage> for CAudioFrameMessage {
    fn c_repr_of(input: hermes::AudioFrameMessage) -> Result<Self> {
        Ok(Self {
            wav_frame_len: input.wav_frame.len() as libc::c_int,
            wav_frame: Box::into_raw(input.wav_frame.into_boxed_slice()) as *const u8,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::AudioFrameMessage> for CAudioFrameMessage {
    fn as_rust(&self) -> Result<hermes::AudioFrameMessage> {
        Ok(hermes::AudioFrameMessage {
            wav_frame: unsafe {
                slice::from_raw_parts(self.wav_frame as *const u8,
                                      self.wav_frame_len as usize)
            }.to_vec(),
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CAudioFrameMessage {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.wav_frame as *mut u8,
                self.wav_frame_len as usize,
            ))
        };
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CPlayFinishedMessage {
    pub id: *const libc::c_char,
    pub site_id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CPlayFinishedMessage {}

impl CPlayFinishedMessage {
    pub fn from(input: hermes::PlayFinishedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::PlayFinishedMessage> for CPlayFinishedMessage {
    fn c_repr_of(input: hermes::PlayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::PlayFinishedMessage> for CPlayFinishedMessage {
    fn as_rust(&self) -> Result<hermes::PlayFinishedMessage> {
        Ok(hermes::PlayFinishedMessage {
            id: create_rust_string_from!(self.id),
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CPlayFinishedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.id);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayMessage {
    pub text: *const libc::c_char,
    // Nullable
    pub lang: *const libc::c_char,
    // Nullable
    pub id: *const libc::c_char,
    pub site_id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

impl CReprOf<hermes::SayMessage> for CSayMessage {
    fn c_repr_of(input: hermes::SayMessage) -> Result<Self> {
        Ok(Self {
            text: convert_to_c_string!(input.text),
            lang: convert_to_nullable_c_string!(input.lang),
            id: convert_to_nullable_c_string!(input.id),
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SayMessage> for CSayMessage {
    fn as_rust(&self) -> Result<hermes::SayMessage> {
        Ok(hermes::SayMessage {
            text: create_rust_string_from!(self.text),
            lang: create_optional_rust_string_from!(self.lang),
            id: create_optional_rust_string_from!(self.id),
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}


impl CSayMessage {
    pub fn from(input: hermes::SayMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_say_message(&self) -> Result<hermes::SayMessage> {
        self.as_rust()
    }
}

unsafe impl Sync for CSayMessage {}

impl Drop for CSayMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.text);
        take_back_nullable_c_string!(self.lang);
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSayFinishedMessage {
    // Nullable
    pub id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CSayFinishedMessage {}

impl CSayFinishedMessage {
    pub fn from(input: hermes::SayFinishedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_say_finished_message(&self) -> Result<hermes::SayFinishedMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::SayFinishedMessage> for CSayFinishedMessage {
    fn c_repr_of(input: hermes::SayFinishedMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::SayFinishedMessage> for CSayFinishedMessage {
    fn as_rust(&self) -> Result<hermes::SayFinishedMessage> {
        Ok(hermes::SayFinishedMessage {
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CSayFinishedMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluSlotMessage {
    // Nullable
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent_name: *const libc::c_char,
    // Nullable
    pub slot: *const CSlot,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluSlotMessage {}

impl CNluSlotMessage {
    pub fn from(input: hermes::NluSlotMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluSlotMessage> for CNluSlotMessage {
    fn c_repr_of(input: hermes::NluSlotMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent_name: convert_to_c_string!(input.intent_name),
            slot: if let Some(s) = input.slot {
                CSlot::from(s).into_raw_pointer()
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluSlotMessage> for CNluSlotMessage {
    fn as_rust(&self) -> Result<hermes::NluSlotMessage> {
        /*Ok(hermes::NluSlotMessage {
            id: create_optional_rust_string_from!(self.id),
            input: create_rust_string_from!(self.input),
            intent_name: create_rust_string_from!(self.intent_name),
            session_id: create_optional_rust_string_from!(self.session_id),
            slot: unsafe { CSlot::raw_borrow(self.slot) } ?.as_rust()?, // TODO impl in snips-nlu-ontology
        })*/
        bail!("Missing converter for CSlot, if you need this feature, please tell us !")
    }
}

impl Drop for CNluSlotMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        take_back_c_string!(self.intent_name);
        if !self.slot.is_null() {
            let _ = unsafe { CSlot::from_raw_pointer(self.slot) };
        }
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentNotRecognizedMessage {
    pub input: *const libc::c_char,
    // Nullable
    pub id: *const libc::c_char,
    // Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluIntentNotRecognizedMessage {}

impl CNluIntentNotRecognizedMessage {
    pub fn from(input: hermes::NluIntentNotRecognizedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluIntentNotRecognizedMessage> for CNluIntentNotRecognizedMessage {
    fn c_repr_of(input: hermes::NluIntentNotRecognizedMessage) -> Result<Self> {
        Ok(Self {
            input: convert_to_c_string!(input.input),
            id: convert_to_nullable_c_string!(input.id),
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluIntentNotRecognizedMessage> for CNluIntentNotRecognizedMessage {
    fn as_rust(&self) -> Result<hermes::NluIntentNotRecognizedMessage> {
        Ok(hermes::NluIntentNotRecognizedMessage {
            input: create_rust_string_from!(self.input),
            id: create_optional_rust_string_from!(self.id),
            session_id: create_optional_rust_string_from!(self.session_id),
        })
    }
}

impl Drop for CNluIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        take_back_nullable_c_string!(self.id);
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNluIntentMessage {
    // Nullable
    pub id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    // Nullable
    pub slots: *const CSlotList,
    //Nullable
    pub session_id: *const libc::c_char,
}

unsafe impl Sync for CNluIntentMessage {}

impl CNluIntentMessage {
    pub fn from(input: hermes::NluIntentMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::NluIntentMessage> for CNluIntentMessage {
    fn c_repr_of(input: hermes::NluIntentMessage) -> Result<Self> {
        Ok(Self {
            id: convert_to_nullable_c_string!(input.id),
            input: convert_to_c_string!(input.input),
            intent: CIntentClassifierResult::from(input.intent).into_raw_pointer(),
            slots: if let Some(slots) = input.slots {
                CSlotList::from(slots).into_raw_pointer()
            } else {
                null()
            },
            session_id: convert_to_nullable_c_string!(input.session_id),
        })
    }
}

impl AsRust<hermes::NluIntentMessage> for CNluIntentMessage {
    fn as_rust(&self) -> Result<hermes::NluIntentMessage> {
        /*Ok(hermes::NluIntentMessage {
            id: create_optional_rust_string_from!(self.id),
            input: create_rust_string_from!(self.input),
            intent: unsafe {CIntentClassifierResult::raw_borrow(self.intent) }?.as_rust()?, // TODO impl in snips-nlu-ontology
            slots: if self.slots.is_null() { None }  else { unsafe {CSlotList::raw_borrow(self.slots)}?.as_rust()? }, // TODO impl in snips-nlu-ontology
            session_id: create_optional_rust_string_from!(self.session_id),
        })*/
        bail!("Missing converter for CIntentClassifierResult and CSlotList, if you need this feature, please tell us !")
    }
}

impl Drop for CNluIntentMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.id);
        take_back_c_string!(self.input);
        let _ = unsafe { CIntentClassifierResult::from_raw_pointer(self.intent) };
        if !self.slots.is_null() {
            let _ = unsafe { CSlotList::from_raw_pointer(self.slots) };
        }
        take_back_nullable_c_string!(self.session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentMessage {
    pub session_id: *const libc::c_char,
    // Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,

    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    // Nullable
    pub slots: *const CSlotList,
}

unsafe impl Sync for CIntentMessage {}

impl CIntentMessage {
    pub fn from(input: hermes::IntentMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::IntentMessage> for CIntentMessage {
    fn c_repr_of(input: hermes::IntentMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            input: convert_to_c_string!(input.input),
            intent: Box::into_raw(Box::new(CIntentClassifierResult::from(input.intent))),
            slots: if let Some(slots) = input.slots {
                Box::into_raw(Box::new(CSlotList::from(slots))) as *const CSlotList
            } else {
                null()
            },
        })
    }
}

impl Drop for CIntentMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.input);
        let _ = unsafe { Box::from_raw(self.intent as *mut CIntentClassifierResult) };
        if !self.slots.is_null() {
            let _ = unsafe { Box::from_raw(self.slots as *mut CSlotList) };
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum CSessionInitType {
    Action = 1,
    Notification = 2,
}

impl CSessionInitType {
    pub fn from(slot_value: &hermes::SessionInit) -> Self {
        match *slot_value {
            hermes::SessionInit::Notification { .. } => CSessionInitType::Notification,
            hermes::SessionInit::Action { .. } => CSessionInitType::Action,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct CActionSessionInit {
    // Nullable
    text: *const libc::c_char,
    // Nullable
    intent_filter: *const CArrayString,
    can_be_enqueued: libc::c_uchar,
}

impl CActionSessionInit {
    pub fn new(
        text: Option<String>,
        intent_filter: Option<Vec<String>>,
        can_be_enqueued: bool,
    ) -> Result<Self> {
        Ok(Self {
            text: convert_to_nullable_c_string!(text),
            intent_filter: convert_to_nullable_c_array_string!(intent_filter),
            can_be_enqueued: if can_be_enqueued { 1 } else { 0 },
        })
    }

    pub fn to_action_session_init(&self) -> Result<hermes::SessionInit> {
        Ok(hermes::SessionInit::Action {
            text: create_optional_rust_string_from!(self.text),
            intent_filter: match unsafe { self.intent_filter.as_ref() } {
                Some(it) => Some(it.as_rust()?),
                None => None,
            },
            can_be_enqueued: self.can_be_enqueued == 1,
        })
    }
}

impl Drop for CActionSessionInit {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.text);
        take_back_nullable_c_array_string!(self.intent_filter);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionInit {
    init_type: CSessionInitType,
    /**
     * Points to either a *const char, a *const CActionSessionInit
     */
    value: *const libc::c_void,
}

impl CSessionInit {
    fn from(init: hermes::SessionInit) -> Result<Self> {
        let init_type = CSessionInitType::from(&init);
        let value: *const libc::c_void = match init {
            hermes::SessionInit::Action {
                text,
                intent_filter,
                can_be_enqueued,
            } => Box::into_raw(Box::new(CActionSessionInit::new(
                text,
                intent_filter,
                can_be_enqueued,
            )?)) as _,
            hermes::SessionInit::Notification { text } => convert_to_c_string!(text) as _,
        };
        Ok(Self { init_type, value })
    }

    fn to_session_init(&self) -> Result<hermes::SessionInit> {
        match self.init_type {
            CSessionInitType::Action => {
                unsafe { (self.value as *const CActionSessionInit).as_ref() }
                    .ok_or_else(|| format_err!("unexpected null pointer in SessionInit value"))?
                    .to_action_session_init()
            }
            CSessionInitType::Notification => Ok(hermes::SessionInit::Notification {
                text: create_rust_string_from!((self.value as *const libc::c_char)
                    .as_ref()
                    .ok_or_else(|| format_err!("unexpected null pointer in SessionInit value"))?),
            }),
        }
    }
}

impl Drop for CSessionInit {
    fn drop(&mut self) {
        match self.init_type {
            CSessionInitType::Action => unsafe {
                let _ = CActionSessionInit::from_raw_pointer(self.value as _);
            }
            CSessionInitType::Notification => {
                take_back_c_string!(self.value as *const libc::c_char);
            }
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CStartSessionMessage {
    pub init: CSessionInit,
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

impl CStartSessionMessage {
    pub fn from(input: hermes::StartSessionMessage) -> Result<Self> {
        Ok(Self {
            init: CSessionInit::from(input.init)?,
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_nullable_c_string!(input.site_id),
        })
    }

    pub fn to_start_session_message(&self) -> Result<hermes::StartSessionMessage> {
        self.as_rust()
    }
}

impl AsRust<hermes::StartSessionMessage> for CStartSessionMessage {
    fn as_rust(&self) -> Result<hermes::StartSessionMessage> {
        Ok(hermes::StartSessionMessage {
            init: self.init.to_session_init()?,
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_optional_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CStartSessionMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.custom_data);
        take_back_nullable_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionStartedMessage {
    pub session_id: *const libc::c_char,
    // Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
    // Nullable
    pub reactivated_from_session_id: *const libc::c_char,
}

unsafe impl Sync for CSessionStartedMessage {}

impl CSessionStartedMessage {
    pub fn from(input: hermes::SessionStartedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionStartedMessage> for CSessionStartedMessage {
    fn c_repr_of(input: hermes::SessionStartedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            reactivated_from_session_id: convert_to_nullable_c_string!(
                input.reactivated_from_session_id
            ),
        })
    }
}

impl Drop for CSessionStartedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.reactivated_from_session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionQueuedMessage {
    pub session_id: *const libc::c_char,
    // Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionQueuedMessage {}

impl CSessionQueuedMessage {
    pub fn from(input: hermes::SessionQueuedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionQueuedMessage> for CSessionQueuedMessage {
    fn c_repr_of(input: hermes::SessionQueuedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl Drop for CSessionQueuedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CContinueSessionMessage {
    pub session_id: *const libc::c_char,
    pub text: *const libc::c_char,
    // Nullable
    pub intent_filter: *const CArrayString,
}

impl CContinueSessionMessage {
    pub fn from(input: hermes::ContinueSessionMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_c_string!(input.text),
            intent_filter: convert_to_nullable_c_array_string!(input.intent_filter),
        })
    }

    pub fn to_continue_session_message(&self) -> Result<hermes::ContinueSessionMessage> {
        self.as_rust()
    }
}

impl AsRust<hermes::ContinueSessionMessage> for CContinueSessionMessage {
    fn as_rust(&self) -> Result<hermes::ContinueSessionMessage> {
        Ok(hermes::ContinueSessionMessage {
            session_id: create_rust_string_from!(self.session_id),
            text: create_rust_string_from!(self.text),
            intent_filter: match unsafe { self.intent_filter.as_ref() } {
                Some(it) => Some(it.as_rust()?),
                None => None,
            },
        })
    }
}

impl Drop for CContinueSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_c_string!(self.text);
        take_back_nullable_c_array_string!(self.intent_filter);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CEndSessionMessage {
    pub session_id: *const libc::c_char,
    // Nullable
    pub text: *const libc::c_char,
}

impl CEndSessionMessage {
    pub fn from(input: hermes::EndSessionMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_nullable_c_string!(input.text),
        })
    }

    pub fn to_end_session_message(&self) -> Result<hermes::EndSessionMessage> {
        self.as_rust()
    }
}

impl AsRust<hermes::EndSessionMessage> for CEndSessionMessage {
    fn as_rust(&self) -> Result<hermes::EndSessionMessage> {
        Ok(hermes::EndSessionMessage {
            session_id: create_rust_string_from!(self.session_id),
            text: create_optional_rust_string_from!(self.text),
        })
    }
}

impl Drop for CEndSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.text);
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum CSessionTerminationType {
    Nominal = 1,
    SiteUnavailable = 2,
    AbortedByUser = 3,
    IntentNotRecognized = 4,
    Timeout = 5,
    Error = 6,
}

impl CSessionTerminationType {
    fn from(termination_type: &hermes::SessionTerminationType) -> CSessionTerminationType {
        match *termination_type {
            hermes::SessionTerminationType::Nominal => CSessionTerminationType::Nominal,
            hermes::SessionTerminationType::SiteUnavailable => {
                CSessionTerminationType::SiteUnavailable
            }
            hermes::SessionTerminationType::AbortedByUser => CSessionTerminationType::AbortedByUser,
            hermes::SessionTerminationType::IntentNotRecognized => {
                CSessionTerminationType::IntentNotRecognized
            }
            hermes::SessionTerminationType::Timeout => CSessionTerminationType::Timeout,
            hermes::SessionTerminationType::Error { .. } => CSessionTerminationType::Error,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionTermination {
    termination_type: CSessionTerminationType,
    // Nullable,
    data: *const libc::c_char,
}

impl CSessionTermination {
    fn from(termination: ::hermes::SessionTerminationType) -> Result<Self> {
        let termination_type = CSessionTerminationType::from(&termination);
        let data: *const libc::c_char = match termination {
            ::hermes::SessionTerminationType::Error { error } => convert_to_c_string!(error),
            _ => null(),
        };
        Ok(Self {
            termination_type,
            data,
        })
    }
}

impl Drop for CSessionTermination {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.data);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionEndedMessage {
    pub session_id: *const libc::c_char,
    // Nullable
    pub custom_data: *const libc::c_char,
    pub termination: CSessionTermination,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionEndedMessage {}

impl CSessionEndedMessage {
    pub fn from(input: hermes::SessionEndedMessage) -> Result<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionEndedMessage> for CSessionEndedMessage {
    fn c_repr_of(input: hermes::SessionEndedMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            termination: CSessionTermination::from(input.termination)?,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl Drop for CSessionEndedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CVersionMessage {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl CVersionMessage {
    pub fn from(input: &hermes::VersionMessage) -> Result<Self> {
        Ok(Self {
            major: input.version.major,
            minor: input.version.minor,
            patch: input.version.patch,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CErrorMessage {
    // Nullable
    pub session_id: *const libc::c_char,
    pub error: *const libc::c_char,
    // Nullable
    pub context: *const libc::c_char,
}

impl CErrorMessage {
    pub fn from(input: hermes::ErrorMessage) -> Result<Self> {
        Ok(Self {
            session_id: convert_to_nullable_c_string!(input.session_id),
            error: convert_to_c_string!(input.error),
            context: convert_to_nullable_c_string!(input.context),
        })
    }
}

impl Drop for CErrorMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.session_id);
        take_back_c_string!(self.error);
        take_back_nullable_c_string!(self.context);
    }
}