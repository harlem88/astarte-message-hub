/*
 * This file is part of Astarte.
 *
 * Copyright 2022 SECO Mind Srl
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use std::time::SystemTime;

use crate::error::AstarteMessageHubError;
use crate::error::AstarteMessageHubError::TypeConversionError;
use chrono::{DateTime, Utc};

use crate::proto_message_hub::astarte_data_type::Data::AstarteIndividual;
use crate::proto_message_hub::astarte_data_type_individual::IndividualData;
use crate::proto_message_hub::AstarteDataType;
use crate::proto_message_hub::AstarteDataTypeIndividual;
use crate::proto_message_hub::{
    AstarteBinaryBlobArray, AstarteBooleanArray, AstarteDateTimeArray, AstarteDoubleArray,
    AstarteIntegerArray, AstarteLongIntegerArray, AstarteStringArray,
};

macro_rules! impl_type_conversion_traits {
    ( {$( ($typ:ty, $astartedatatype:ident) ,)*}) => {

        $(
               impl From<$typ> for AstarteDataType {
                    fn from(d: $typ) -> Self {
                        AstarteDataType {
                            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                            individual_data: Some(IndividualData::$astartedatatype(d.into())),
                            }))
                        }
                    }
                }

                impl From<&$typ> for AstarteDataType {
                    fn from(d: &$typ) -> Self {
                        AstarteDataType {
                            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                            individual_data: Some(IndividualData::$astartedatatype(d.clone().into())),
                            }))
                        }
                    }
                }
        )*
    };
}

impl_type_conversion_traits!({
    (i32, AstarteInteger),
    (bool, AstarteBoolean),
    (i64, AstarteLongInteger),
    (&str, AstarteString),
    (String, AstarteString),
    (Vec<u8>, AstarteBinaryBlob),
});

impl TryFrom<f64> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() || value.is_infinite() || value.is_subnormal() {
            return Err(TypeConversionError);
        }
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteDouble(value)),
            })),
        })
    }
}

impl TryFrom<DateTime<Utc>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: DateTime<Utc>) -> Result<Self, Self::Error> {
        let system_time: SystemTime = value.try_into().unwrap();

        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteDateTime(system_time.into())),
            })),
        })
    }
}

impl TryFrom<Vec<f64>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteDoubleArray(AstarteDoubleArray {
                    astarte_double: value,
                })),
            })),
        })
    }
}

impl TryFrom<Vec<i32>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteIntegerArray(AstarteIntegerArray {
                    astarte_integer: value,
                })),
            })),
        })
    }
}

impl TryFrom<Vec<i64>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteLongIntegerArray(
                    AstarteLongIntegerArray {
                        astarte_long_integer: value,
                    },
                )),
            })),
        })
    }
}

impl TryFrom<Vec<bool>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<bool>) -> Result<Self, Self::Error> {
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteBooleanArray(AstarteBooleanArray {
                    astarte_boolean: value,
                })),
            })),
        })
    }
}

impl TryFrom<Vec<String>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteStringArray(AstarteStringArray {
                    astarte_string: value,
                })),
            })),
        })
    }
}

impl TryFrom<Vec<Vec<u8>>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteBinaryBlobArray(
                    AstarteBinaryBlobArray {
                        astarte_binary_blob: value,
                    },
                )),
            })),
        })
    }
}

impl TryFrom<Vec<DateTime<Utc>>> for AstarteDataType {
    type Error = AstarteMessageHubError;
    fn try_from(value: Vec<DateTime<Utc>>) -> Result<Self, Self::Error> {
        use prost_types::Timestamp;
        Ok(AstarteDataType {
            data: Some(AstarteIndividual(AstarteDataTypeIndividual {
                individual_data: Some(IndividualData::AstarteDateTimeArray(AstarteDateTimeArray {
                    astarte_date_time: value
                        .iter()
                        .map(|x| {
                            let system_time: SystemTime = x.clone().into();
                            system_time.into()
                        })
                        .collect::<Vec<Timestamp>>(),
                })),
            })),
        })
    }
}

impl TryFrom<AstarteDataType> for DateTime<Utc> {
    type Error = AstarteMessageHubError;

    fn try_from(value: AstarteDataType) -> Result<Self, Self::Error> {
        return if let Some(AstarteIndividual(data)) = value.data {
            if let Some(IndividualData::AstarteDateTime(datetime_value)) = data.individual_data {
                let system_time: SystemTime = datetime_value.try_into()?;
                let sas: DateTime<Utc> = system_time.try_into()?;
                Ok(sas)
            } else {
                Err(TypeConversionError)
            }
        } else {
            Err(TypeConversionError)
        };
    }
}

#[cfg(test)]
mod test {
    use crate::proto_message_hub::astarte_data_type::Data::AstarteIndividual;
    use crate::proto_message_hub::astarte_data_type_individual::IndividualData;
    use crate::proto_message_hub::AstarteDataType;

    #[test]
    fn double_into_astarte_data_type_success() {
        let expected_double_value: f64 = 15.5;
        let d_astarte_data_type: AstarteDataType = expected_double_value.try_into().unwrap();

        if let AstarteIndividual(data) = d_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteDouble(double_value) = data.individual_data.unwrap() {
                assert_eq!(expected_double_value, double_value);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn integer_into_astarte_data_type_success() {
        let expected_integer_value: i32 = 15;
        let i32_astarte_data_type: AstarteDataType = expected_integer_value.try_into().unwrap();

        if let AstarteIndividual(data) = i32_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteInteger(i32_value) = data.individual_data.unwrap() {
                assert_eq!(expected_integer_value, i32_value);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn bool_into_astarte_data_type_success() {
        let expected_bool_value: bool = true;
        let bool_astarte_data_type: AstarteDataType = expected_bool_value.try_into().unwrap();

        if let AstarteIndividual(data) = bool_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteBoolean(bool_value) = data.individual_data.unwrap() {
                assert_eq!(expected_bool_value, bool_value);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn longinteger_into_astarte_data_type_success() {
        let expected_longinteger_value: i64 = 15;
        let i64_astarte_data_type: AstarteDataType = expected_longinteger_value.try_into().unwrap();

        if let AstarteIndividual(data) = i64_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteLongInteger(i64_value) = data.individual_data.unwrap() {
                assert_eq!(expected_longinteger_value, i64_value);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn string_into_astarte_data_type_success() {
        let expected_string_value: String = "15".to_owned();
        let string_astarte_data_type: AstarteDataType =
            expected_string_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = string_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteString(string_value) = data.individual_data.unwrap() {
                assert_eq!(expected_string_value, string_value);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn u8_array_into_astarte_data_type_success() {
        let expected_vec_u8_value: Vec<u8> = vec![10, 44];
        let vec_u8_astarte_data_type: AstarteDataType =
            expected_vec_u8_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_u8_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteBinaryBlob(vec_u8_values) = data.individual_data.unwrap()
            {
                assert_eq!(expected_vec_u8_value, vec_u8_values);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn datetime_into_astarte_data_type_success() {
        use chrono::{DateTime, Utc};

        let expected_datetime_value = Utc::now();
        let datetime_astarte_data_type: AstarteDataType =
            expected_datetime_value.clone().try_into().unwrap();

        let date_time: DateTime<Utc> = datetime_astarte_data_type.try_into().unwrap();
        assert_eq!(expected_datetime_value, date_time);
    }

    #[test]
    fn double_array_into_astarte_data_type_success() {
        let expected_vec_double_value: Vec<f64> = vec![10.54, 44.99];
        let vec_double_astarte_data_type: AstarteDataType =
            expected_vec_double_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_double_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteDoubleArray(vec_double_values) =
                data.individual_data.unwrap()
            {
                assert_eq!(expected_vec_double_value, vec_double_values.astarte_double);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn integer_array_into_astarte_data_type_success() {
        let expected_vec_i32_value: Vec<i32> = vec![10, 44];
        let vec_i32_astarte_data_type: AstarteDataType =
            expected_vec_i32_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_i32_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteIntegerArray(vec_i32_values) =
                data.individual_data.unwrap()
            {
                assert_eq!(expected_vec_i32_value, vec_i32_values.astarte_integer);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn long_integer_array_into_astarte_data_type_success() {
        let expected_vec_i64_value: Vec<i64> = vec![10, 44];
        let vec_i64_astarte_data_type: AstarteDataType =
            expected_vec_i64_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_i64_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteLongIntegerArray(vec_i64_values) =
                data.individual_data.unwrap()
            {
                assert_eq!(expected_vec_i64_value, vec_i64_values.astarte_long_integer);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn bool_array_into_astarte_data_type_success() {
        let expected_vec_bool_value: Vec<bool> = vec![false, true];
        let vec_bool_astarte_data_type: AstarteDataType =
            expected_vec_bool_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_bool_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteBooleanArray(vec_bool_values) =
                data.individual_data.unwrap()
            {
                assert_eq!(expected_vec_bool_value, vec_bool_values.astarte_boolean);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn string_array_into_astarte_data_type_success() {
        let expected_vec_string_value: Vec<String> = vec!["test1".to_owned(), "test2".to_owned()];
        let vec_string_astarte_data_type: AstarteDataType =
            expected_vec_string_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_string_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteStringArray(vec_string_values) =
                data.individual_data.unwrap()
            {
                assert_eq!(expected_vec_string_value, vec_string_values.astarte_string);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn binary_blob_array_into_astarte_data_type_success() {
        let expected_vec_binary_blob_value: Vec<Vec<u8>> = vec![vec![12, 245], vec![78, 11, 128]];
        let vec_binary_blob_astarte_data_type: AstarteDataType =
            expected_vec_binary_blob_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_binary_blob_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteBinaryBlobArray(vec_binary_blob_values) =
                data.individual_data.unwrap()
            {
                assert_eq!(
                    expected_vec_binary_blob_value,
                    vec_binary_blob_values.astarte_binary_blob
                );
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn datetime_array_into_astarte_data_type_success() {
        use chrono::{DateTime, Utc};
        use std::time::SystemTime;

        let expected_vec_datetime_value = vec![Utc::now(), Utc::now()];
        let vec_datetime_astarte_data_type: AstarteDataType =
            expected_vec_datetime_value.clone().try_into().unwrap();

        if let AstarteIndividual(data) = vec_datetime_astarte_data_type.data.unwrap() {
            if let IndividualData::AstarteDateTimeArray(vec_datetime_value) =
                data.individual_data.unwrap()
            {
                for i in 0..expected_vec_datetime_value.len() {
                    let system_time: SystemTime = vec_datetime_value
                        .astarte_date_time
                        .get(i)
                        .unwrap()
                        .clone()
                        .try_into()
                        .unwrap();

                    let date_time: DateTime<Utc> = system_time.try_into().unwrap();
                    assert_eq!(expected_vec_datetime_value.get(i).unwrap(), &date_time);
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }
}
