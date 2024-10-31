use serde::Serialize;
use serde_json::to_string;
use partially_derive::Partial;

#[derive(Partial, Serialize)]
#[partially(derive(Serialize))]
struct Data {
    #[serde(rename = "foo")]
    #[partially(skip_attributes)]
    #[partially(attribute(serde(rename = "bar")))]
    value: String
}

#[test]
fn test_field_attrs() {
    let data = Data{
        value: "v1".to_string()
    };
    assert_eq!(
        to_string(&data).unwrap(),
        "{\"foo\":\"v1\"}"
    );
    let partial_data = PartialData{
        value: "v2".to_string().into()
    };
    assert_eq!(
        to_string(&partial_data).unwrap(),
        "{\"bar\":\"v2\"}"
    )
}
