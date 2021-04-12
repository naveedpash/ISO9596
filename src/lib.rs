#![allow(dead_code)]

use rasn::{types::*, *};

// Object Identifiers
const CMIPUSERINFO_OID: [u32; 4] = [9, 1, 0, 1];
const CMIPABORTINFO_OID: [u32; 4] = [9, 1, 0, 2];

pub fn cmip_userinfo_oid() -> ObjectIdentifier {
    ObjectIdentifier::new(Vec::from(CMIPUSERINFO_OID)).unwrap()
}

pub fn cmip_abortinfo_oid() -> ObjectIdentifier {
    ObjectIdentifier::new(Vec::from(CMIPABORTINFO_OID)).unwrap()
}

// Context Tags
const TAG_CMIPUSERINFO_0: Tag = Tag::new(Class::Context, 0);
const TAG_CMIPUSERINFO_1: Tag = Tag::new(Class::Context, 1);
const TAG_CMIPUSERINFO_2: Tag = Tag::new(Class::Context, 2);
const TAG_CMIPUSERINFO_3: Tag = Tag::new(Class::Context, 3);

#[derive(Debug, Clone)]
pub(crate) struct CMIPUserInfo<A, U> {
    functional_units: BitString,
    protocol_version: BitString,
    access_control: Option<InstanceOf<A>>,
    user_info: Option<InstanceOf<U>>,
}

impl<A, U> CMIPUserInfo<A, U> {
    fn new(
        functional_units: [u8; 4],
        protocol_version: [u8; 2],
        access_control: Option<InstanceOf<A>>,
        user_info: Option<InstanceOf<U>>,
    ) -> Self {
        CMIPUserInfo {
            functional_units: BitString::from_vec(functional_units.to_vec()),
            protocol_version: BitString::from_vec(protocol_version.to_vec()),
            access_control,
            user_info,
        }
    }
}

impl<A, U> AsnType for CMIPUserInfo<A, U> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<A, U> Default for CMIPUserInfo<A, U> {
    fn default() -> Self {
        CMIPUserInfo {
            functional_units: BitString::from_vec(vec![0, 0, 0, 0]),
            protocol_version: BitString::from_vec(vec![1, 0]),
            access_control: None,
            user_info: None,
        }
    }
}

impl<A: Encode, U: Encode> Encode for CMIPUserInfo<A, U> {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_sequence(tag, |sequence| {
            self.functional_units
                .encode_with_tag(sequence, TAG_CMIPUSERINFO_0)?;
            self.protocol_version
                .encode_with_tag(sequence, TAG_CMIPUSERINFO_1)?;
            match &self.access_control {
                None => (),
                Some(a) => a.encode_with_tag(sequence, TAG_CMIPUSERINFO_2)?,
            }
            match &self.user_info {
                None => (),
                Some(u) => u.encode_with_tag(sequence, TAG_CMIPUSERINFO_3)?,
            }
            Ok(())
        })?;

        Ok(())
    }
}

impl<A: Decode, U: Decode> Decode for CMIPUserInfo<A, U> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        let mut sequence = decoder.decode_sequence(tag)?;
        let functional_units = BitString::decode(&mut sequence)?;
        let protocol_version = BitString::decode(&mut sequence)?;
        let mut access_control: Option<InstanceOf<A>> = None;
        let mut user_info: Option<InstanceOf<U>> = None;
        match decoder.peek_tag() {
            Ok(tag) => match tag {
                TAG_CMIPUSERINFO_2 => access_control = Some(InstanceOf::decode(&mut sequence)?),
                TAG_CMIPUSERINFO_3 => user_info = Some(InstanceOf::decode(&mut sequence)?),
                _ => (),
            },
            Err(_) => (),
        }
        match decoder.peek_tag() {
            Ok(tag) => match tag {
                TAG_CMIPUSERINFO_2 => access_control = Some(InstanceOf::decode(&mut sequence)?),
                TAG_CMIPUSERINFO_3 => user_info = Some(InstanceOf::decode(&mut sequence)?),
                _ => (),
            },
            Err(_) => (),
        }

        Ok(Self {
            functional_units,
            protocol_version,
            access_control,
            user_info,
        })
    }
}

#[derive(AsnType, Clone, Copy, Debug, Encode, Decode, PartialEq)]
#[rasn(enumerated)]
pub enum CMIPAbortSource {
    CMISEServiceUser = 0,
    CMISEServiceProvider = 1,
}

// Context Tags
const TAG_CMIPABORTINFO_0: Tag = Tag::new(Class::Context, 0);
const TAG_CMIPABORTINFO_1: Tag = Tag::new(Class::Context, 1);

pub(crate) struct CMIPAbortInfo<U> {
    abort_source: CMIPAbortSource,
    user_info: Option<InstanceOf<U>>,
}

impl<U> CMIPAbortInfo<U> {
    fn new(abort_source: CMIPAbortSource, user_info: Option<InstanceOf<U>>) -> Self {
        CMIPAbortInfo {
            abort_source,
            user_info,
        }
    }
}

impl<U> AsnType for CMIPAbortInfo<U> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<U: Encode> Encode for CMIPAbortInfo<U> {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_sequence(tag, |sequence| {
            self.abort_source
                .encode_with_tag(sequence, TAG_CMIPABORTINFO_0);
            match &self.user_info {
                None => (),
                Some(u) => u.encode_with_tag(sequence, TAG_CMIPABORTINFO_1)?,
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl<U: Decode> Decode for CMIPAbortInfo<U> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        let mut sequence = decoder.decode_sequence(tag)?;
        let enumerated = sequence.decode_enumerated(TAG_CMIPABORTINFO_0)?;
        let abort_source = match enumerated.to_string().as_str() {
            "0" => CMIPAbortSource::CMISEServiceUser,
            "1" => CMIPAbortSource::CMISEServiceProvider,
            _ => return,
        };
        let mut user_info: Option<InstanceOf<U>> = None;
        match sequence.peek_tag() {
            Err(_) => (),
            Ok(_) => {
                user_info = Some(InstanceOf::decode_with_tag(
                    &mut sequence,
                    TAG_CMIPABORTINFO_1,
                )?)
            }
        }

        Ok(Self {
            abort_source,
            user_info,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
