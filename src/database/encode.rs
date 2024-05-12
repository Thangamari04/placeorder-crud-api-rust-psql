// src/database/encode.rs

use sqlx::{Encode, TypeInfo};

impl<'q, DB: sqlx::Database> Encode<'q, DB> for String
where
    DB: sqlx::database::HasArguments<'q>,
    &'q str: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        <&str as Encode<DB>>::encode(&self, buf)
    }

    fn size_hint(&self) -> usize {
        self.len()
    }

    fn type_info(&self) -> DB::TypeInfo {
        DB::TypeInfo::TEXT
    }
}
