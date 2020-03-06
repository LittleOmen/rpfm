//---------------------------------------------------------------------------//
// Copyright (c) 2017-2020 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module with the `Encoder` trait, to encode data to `Vec<u8>`..

This module contains the `Encoder` trait and his implementation for `Vec<u8>`. This trait allow us
to encode any type of data contained within a PackFile/PackedFile, so it can be saved to disk and
read by the games.

Note: If you change anything from here, remember to update the `encoder_test.rs` file for it.
!*/

use byteorder::{LittleEndian, WriteBytesExt};

use rpfm_error::{ErrorKind, Result};

//---------------------------------------------------------------------------//
//                      `Encoder` Trait Definition
//---------------------------------------------------------------------------//

/// This trair allow us to easely encode all kinds of data to a `Vec<u8>`.
pub trait Encoder {

    /// This function allows us to encode a boolean to a byte of a `Vec<u8>`.
    fn encode_bool(&mut self, boolean: bool);

    /// This function allows us to encode an u16 integer into the provided `Vec<u8>`.
    fn encode_integer_u16(&mut self, integer: u16);

    /// This function allows us to encode an u32 integer into the provided `Vec<u8>`.
    fn encode_integer_u32(&mut self, integer: u32);

    /// This function allows us to encode an u64 integer into the provided `Vec<u8>`.
    fn encode_integer_u64(&mut self, integer: u64);

    /// This function allows us to encode an i8 integer into the provided `Vec<u8>`.
    fn encode_integer_i8(&mut self, integer: i8);

    /// This function allows us to encode an i16 integer into the provided `Vec<u8>`.
    fn encode_integer_i16(&mut self, integer: i16);

    /// This function allows us to encode an i32 integer into the provided `Vec<u8>`.
    fn encode_integer_i32(&mut self, integer: i32);

    /// This function allows us to encode an i64 integer into the provided `Vec<u8>`.
    fn encode_integer_i64(&mut self, integer: i64);

    /// This function allows us to encode a f32 float into the provided `Vec<u8>`.
    fn encode_float_f32(&mut self, float: f32);

    /// This function allows us to encode an UTF-8 String into the provided `Vec<u8>`.
    fn encode_string_u8(&mut self, string: &str);

    /// This function allows us to encode a 00-Padded UTF-8 String into the provided `Vec<u8>`.
    ///
    /// This one is a bit special. It's uses a tuple with the String to encode and the total size of the encoded string.
    /// So... we just encode the String as a normal string, then add 0 until we reach the desired size. If the String is
    /// longer than the provided size, we throw an error.
    fn encode_string_u8_0padded(&mut self, string: &(String, usize)) -> Result<()>;

    /// This function allows us to encode an UTF-16 String into the provided `Vec<u8>`.
    fn encode_string_u16(&mut self, string: &str);

    /// This function allows us to encode an UTF-8 String with his lenght (u16) before the String into the provided `Vec<u8>`..
    fn encode_packedfile_string_u8(&mut self, string: &str);

    /// This function allows us to encode an UTF-16 String with his lenght (u16) before the String into the provided `Vec<u8>`..
    fn encode_packedfile_string_u16(&mut self, string: &str);

    /// This function allows us to encode an UTF-8 Optional String into the provided `Vec<u8>`.
    fn encode_packedfile_optional_string_u8(&mut self, string: &str);

    /// This function allows us to encode an UTF-16 Optional String into the provided `Vec<u8>`.
    fn encode_packedfile_optional_string_u16(&mut self, string: &str);
}

/// Implementation of trait `Encoder` for `Vec<u8>`.
impl Encoder for Vec<u8> {

    //---------------------------------------------------------------------------//
    //                          Normal Encoders
    //---------------------------------------------------------------------------//

    fn encode_bool(&mut self, boolean: bool) {
        self.push(if boolean { 1 } else { 0 });
    }

    fn encode_integer_u16(&mut self, integer: u16) {
        self.write_u16::<LittleEndian>(integer).unwrap();
    }

    fn encode_integer_u32(&mut self, integer: u32) {
        self.write_u32::<LittleEndian>(integer).unwrap();
    }

    fn encode_integer_u64(&mut self, integer: u64) {
        self.write_u64::<LittleEndian>(integer).unwrap();
    }

    fn encode_integer_i8(&mut self, integer: i8) {
        self.push(integer as u8);
    }

    fn encode_integer_i16(&mut self, integer: i16) {
        self.write_i16::<LittleEndian>(integer).unwrap();
    }

    fn encode_integer_i32(&mut self, integer: i32) {
        self.write_i32::<LittleEndian>(integer).unwrap();
    }

    fn encode_integer_i64(&mut self, integer: i64) {
        self.write_i64::<LittleEndian>(integer).unwrap();
    }

    fn encode_float_f32(&mut self, float: f32) {
        self.write_f32::<LittleEndian>(float).unwrap();
    }

    fn encode_string_u8(&mut self, string: &str) {
        self.extend_from_slice(string.as_bytes());
    }

    fn encode_string_u8_0padded(&mut self, (string, size): &(String, usize)) -> Result<()> {
        if string.len() <= *size {
            self.extend_from_slice(string.as_bytes());
            self.extend_from_slice(&vec![0; size - string.len()]);
            Ok(())
        } else {
            Err(ErrorKind::HelperDecodingEncodingError(format!("Error trying to encode an UTF-8 0-Padded String: \"{}\" has a lenght of {} chars, but his length should be less or equal than {}.", string, string.len(), size)).into())
        }
    }

    fn encode_string_u16(&mut self, string: &str) {
        string.encode_utf16().for_each(|character| self.encode_integer_u16(character));
    }

    //---------------------------------------------------------------------------//
    //                          Indexed Encoders
    //---------------------------------------------------------------------------//

    fn encode_packedfile_string_u8(&mut self, string: &str) {
        self.encode_integer_u16(string.as_bytes().len() as u16);
        self.encode_string_u8(string);
    }

    fn encode_packedfile_string_u16(&mut self, string: &str) {
        self.encode_integer_u16(string.as_bytes().len() as u16);
        self.encode_string_u16(string);
    }

    fn encode_packedfile_optional_string_u8(&mut self, string: &str) {
        if string.is_empty() {
            self.encode_bool(false);
        }
        else {
            self.encode_bool(true);
            self.encode_integer_u16(string.as_bytes().len() as u16);
            self.encode_string_u8(string);
        }
    }

    fn encode_packedfile_optional_string_u16(&mut self, string: &str) {
        if string.is_empty() {
            self.encode_bool(false);
        }
        else {
            self.encode_bool(true);
            self.encode_integer_u16(string.as_bytes().len() as u16);
            self.encode_string_u16(string);
        }
    }
}
