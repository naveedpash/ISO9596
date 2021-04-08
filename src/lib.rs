#![allow(dead_code)]

use rasn::types::Class;
pub use rasn::types::{BitString, InstanceOf, ObjectIdentifier};
use rasn::{AsnType, Decode, Decoder, Encode, Encoder, Tag};

const CMIPUSERINFO_OID: [u32; 4] = [9, 1, 0, 1];

pub fn cmip_userinfo_oid() -> ObjectIdentifier {
    ObjectIdentifier::new(Vec::from(CMIPUSERINFO_OID)).unwrap()
}

#[derive(Debug, Clone)]
pub struct CMIPUserInfo<A, U> {
    // common_name: ObjectIdentifier,
    functional_units: BitString,
    protocol_version: BitString,
    access_control: Option<InstanceOf<A>>,
    user_info: Option<InstanceOf<U>>,
}

impl<A, U> CMIPUserInfo<A, U> {
    fn new(
        functional_units: u8,
        protocol_version: u8,
        access_control: Option<InstanceOf<A>>,
        user_info: Option<InstanceOf<U>>,
    ) -> Self {
        CMIPUserInfo {
            // common_name: ObjectIdentifier::new(Vec::from(CMIPUSERINFO_OID)).unwrap(),
            functional_units: BitString::from_vec(vec![functional_units]),
            protocol_version: BitString::from_vec(vec![protocol_version]),
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
            // common_name: ObjectIdentifier::new(Vec::from(CMIPUSERINFO_OID)).unwrap(),
            functional_units: BitString::default(),
            protocol_version: BitString::from_vec(vec![0]),
            access_control: None,
            user_info: None,
        }
    }
}

impl<A: Encode, U: Encode> Encode for CMIPUserInfo<A, U> {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_sequence(tag, |sequence| {
            // self.common_name.encode(sequence)?;
            self.functional_units
                .encode_with_tag(sequence, Tag::new(Class::Context, 0))?;
            self.protocol_version
                .encode_with_tag(sequence, Tag::new(Class::Context, 1))?;
            match &self.access_control {
                None => (),
                Some(a) => a.encode_with_tag(sequence, Tag::new(Class::Context, 2))?,
            }
            match &self.user_info {
                None => (),
                Some(a) => a.encode_with_tag(sequence, Tag::new(Class::Context, 3))?,
            }
            Ok(())
        })?;

        Ok(())
    }
}

impl<A: Decode, U: Decode> Decode for CMIPUserInfo<A, U> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        let mut sequence = decoder.decode_sequence(tag)?;
        // let common_name = ObjectIdentifier::decode(&mut sequence)?;
        let functional_units = BitString::decode(&mut sequence)?;
        let protocol_version = BitString::decode(&mut sequence)?;
        let access_control: Option<InstanceOf<A>>;
        let user_info: Option<InstanceOf<U>>;
        match sequence.peek_tag() {
            Ok(tag) => {
                if tag == Tag::new(Class::Context, 2) {
                    access_control = Some(InstanceOf::decode(&mut sequence)?);
                    match sequence.peek_tag() {
                        Ok(_) => {
                            user_info = Some(InstanceOf::decode(&mut sequence)?);
                        }
                        Err(_) => {
                            user_info = None;
                        }
                    }
                } else {
                    access_control = None;
                    user_info = Some(InstanceOf::decode(&mut sequence)?);
                }
            }
            Err(_) => {
                access_control = None;
                user_info = None;
            }
        }

        Ok(Self {
            // common_name,
            functional_units,
            protocol_version,
            access_control,
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
