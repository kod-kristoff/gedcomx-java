use gedcomx_model::conclusion::Date;
use gedcomx_model::conclusion::Identifier;
use gedcomx_model::conclusion::PlaceDescription;
use gedcomx_model::conclusion::PlaceReference;

// use gedcomx_model::common::Attribution;
use gedcomx_model::common::ResourceReference;
use gedcomx_model::common::TextValue;
use gedcomx_model::common::Uri;
use gedcomx_model::types::IdentifierType;
// use gedcomx_model::types::IdentifierType;

use std::error::Error;
// import org.junit.Test;

// import java.util.*;

// import static org.junit.Assert.assert_eq!;
// import static org.junit.Assert.assertNull;

#[test]
fn testPlaceDescription_Tikhvin() -> Result<(), Box<dyn Error>> {
    let mut tikhvinDesc = PlaceDescription::new();

    // assertNull(tikhvinDesc.get_names());
    // assertNull(tikhvinDesc.get_type());
    // assertNull(tikhvinDesc.get_temporal_description());
    // assertNull(tikhvinDesc.get_latitude());
    // assertNull(tikhvinDesc.get_longitude());
    // assertNull(tikhvinDesc.get_spatial_description());
    // assertNull(tikhvinDesc.get_attribution());
    // assertNull(tikhvinDesc.get_extension_elements());

    tikhvinDesc.get_names_mut().push(TextValue::new());
    tikhvinDesc
        .get_names_mut()
        .get_mut(0)
        .unwrap()
        .set_lang("ru-Cyrl".into());
    tikhvinDesc
        .get_names_mut()
        .get_mut(0)
        .unwrap()
        .set_value("Ти́хвин, Ленингра́дская о́бласть, Россия".into());
    tikhvinDesc.get_names_mut().push(
        TextValue::new()
            .lang("ru-Latn")
            .value("Tikhvin, Leningradskaya Oblast', Rossiya"),
    );
    tikhvinDesc.get_names_mut().push(
        TextValue::new()
            .lang("en-Latn")
            .value("Tikhvin, Leningrad Oblast, Russia"),
    );
    tikhvinDesc.set_type(Uri::create("urn:place-authority/city"));
    tikhvinDesc.set_temporal_description(Date::new());
    tikhvinDesc
        .get_temporal_description_mut()
        .set_formal("A+1383/".into());
    tikhvinDesc.set_latitude(59.6436111);
    tikhvinDesc.set_longitude(33.5094444);
    tikhvinDesc.set_spatial_description(ResourceReference::from_uri(Uri::create(
        "data:application/vnd.google-earth.kml+xml;base64,\
         PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4NCjxrbWwgeG1sbnM9Imh0dHA6\
         Ly93d3cub3Blbmdpcy5uZXQva21sLzIuMiIgeG1sbnM6Z3g9Imh0dHA6Ly93d3cuZ29vZ2xlLmNv\
         bS9rbWwvZXh0LzIuMiIgeG1sbnM6a21sPSJodHRwOi8vd3d3Lm9wZW5naXMubmV0L2ttbC8yLjIi\
         IHhtbG5zOmF0b209Imh0dHA6Ly93d3cudzMub3JnLzIwMDUvQXRvbSI+DQo8UGxhY2VtYXJrIGlk\
         PSIyMSI+DQoJPG5hbWU+VGlraHZpbiwgTGVuaW5ncmFkIE9ibGFzdCwgUnVzc2lhPC9uYW1lPg0K\
         CTxNdWx0aUdlb21ldHJ5Pg0KCQk8UG9pbnQ+DQoJCTxjb29yZGluYXRlcz4zMy41LDU5LjYzMzMz\
         MzAwMDAwMDAxLDA8L2Nvb3JkaW5hdGVzPg0KCQk8L1BvaW50Pg0KCQk8TGluZWFyUmluZz4NCgkJ\
         CTxjb29yZGluYXRlcz4NCgkJCQkzMy40NTA5Niw1OS42MTYxNTk0LDAgMzMuNDUwOTYsNTkuNjUw\
         NTA2NiwwIDMzLjU0OTA0LDU5LjY1MDUwNjYsMCAzMy41NDkwNCw1OS42MTYxNTk0LDAgMzMuNDUw\
         OTYsNTkuNjE2MTU5NCwwIA0KCQkJPC9jb29yZGluYXRlcz4NCgkJPC9MaW5lYXJSaW5nPg0KCTwv\
         TXVsdGlHZW9tZXRyeT4NCjwvUGxhY2VtYXJrPg0KPC9rbWw+DQo=",
    )));
    // tikhvinDesc.set_identifiers(new ArrayList<Identifier>());
    tikhvinDesc.get_identifiers().add(Identifier::new(
IdentifierType::Primary,
Uri::create("https://labs.familysearch.org/stdfinder/PlaceDetail.jsp?placeId=3262902#placeDescriptionId")));
    // tikhvinDesc.set_attribution(new Attribution());
    // tikhvinDesc.get_attribution().setContributor(new ResourceReference(Uri.create("urn:contributorId")));
    // tikhvinDesc.get_attribution().setModified(new java.util.Date(1321027871111L)); // 11 Nov 2011 11:11:11.111
    // tikhvinDesc.addExtension_element("tikhvinDesc-junkExtension_element");

    assert_eq!(
        tikhvinDesc.get_names().get(0).unwrap().get_lang(),
        "ru-Cyrl"
    );
    assert_eq!(
        tikhvinDesc.get_names().get(0).unwrap().get_value(),
        "Ти́хвин, Ленингра́дская о́бласть, Россия"
    );
    assert_eq!(
        tikhvinDesc.get_names().get(1).unwrap().get_lang(),
        "ru-Latn"
    );
    assert_eq!(
        tikhvinDesc.get_names().get(1).unwrap().get_value(),
        "Tikhvin, Leningradskaya Oblast', Rossiya"
    );
    assert_eq!(
        tikhvinDesc.get_names().get(2).unwrap().get_lang(),
        "en-Latn"
    );
    assert_eq!(
        tikhvinDesc.get_names().get(2).unwrap().get_value(),
        "Tikhvin, Leningrad Oblast, Russia"
    );
    assert_eq!(
        // tikhvinDesc.get_type().toUri().as_str(),
        tikhvinDesc.get_type().unwrap().as_str(),
        "urn:place-authority/city"
    );
    assert_eq!(
        tikhvinDesc.get_temporal_description().get_formal(),
        "A+1383/"
    );
    assert_eq!(tikhvinDesc.get_latitude().unwrap(), 59.6436111); //, 0d);
    assert_eq!(tikhvinDesc.get_longitude().unwrap(), 33.5094444); //, 0d);
    assert_eq!(
        tikhvinDesc
            .get_spatial_description()
            .unwrap()
            .get_resource()
            // .toUri()
            .as_str(),
        "data:application/vnd.google-earth.kml+xml;base64,\
        PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4NCjxrbWwgeG1sbnM9Imh0dHA6\
        Ly93d3cub3Blbmdpcy5uZXQva21sLzIuMiIgeG1sbnM6Z3g9Imh0dHA6Ly93d3cuZ29vZ2xlLmNv\
        bS9rbWwvZXh0LzIuMiIgeG1sbnM6a21sPSJodHRwOi8vd3d3Lm9wZW5naXMubmV0L2ttbC8yLjIi\
        IHhtbG5zOmF0b209Imh0dHA6Ly93d3cudzMub3JnLzIwMDUvQXRvbSI+DQo8UGxhY2VtYXJrIGlk\
        PSIyMSI+DQoJPG5hbWU+VGlraHZpbiwgTGVuaW5ncmFkIE9ibGFzdCwgUnVzc2lhPC9uYW1lPg0K\
        CTxNdWx0aUdlb21ldHJ5Pg0KCQk8UG9pbnQ+DQoJCTxjb29yZGluYXRlcz4zMy41LDU5LjYzMzMz\
        MzAwMDAwMDAxLDA8L2Nvb3JkaW5hdGVzPg0KCQk8L1BvaW50Pg0KCQk8TGluZWFyUmluZz4NCgkJ\
        CTxjb29yZGluYXRlcz4NCgkJCQkzMy40NTA5Niw1OS42MTYxNTk0LDAgMzMuNDUwOTYsNTkuNjUw\
        NTA2NiwwIDMzLjU0OTA0LDU5LjY1MDUwNjYsMCAzMy41NDkwNCw1OS42MTYxNTk0LDAgMzMuNDUw\
        OTYsNTkuNjE2MTU5NCwwIA0KCQkJPC9jb29yZGluYXRlcz4NCgkJPC9MaW5lYXJSaW5nPg0KCTwv\
        TXVsdGlHZW9tZXRyeT4NCjwvUGxhY2VtYXJrPg0KPC9rbWw+DQo="
    );
    // assert_eq!(tikhvinDesc.get_identifiers().get(0).getKnownType(), IdentifierType.Primary);
    // assert_eq!(tikhvinDesc.get_identifiers().get(0).getValue().toUri().toString(), "https://labs.familysearch.org/stdfinder/PlaceDetail.jsp?placeId=3262902#placeDescriptionId");
    // assert_eq!(tikhvinDesc.get_attribution().getContributor().get_resource().toUri().toString(), "urn:contributorId");
    // assert_eq!(tikhvinDesc.get_attribution().getModified().get_time(), 1321027871111L);
    // assert_eq!(tikhvinDesc.findExtensionOfType(String.class), "tikhvinDesc-junkExtension_element");
    Ok(())
}

// #[test]
// fn testPlaceReference_Tikhvin() -> Result<(), Box<dyn Error>> {
//   let mut tikhvinRef =  PlaceReference::new();

//   // assertNull(tikhvinRef.getOriginal());
//   // assertNull(tikhvinRef.getDescriptionRef());
//   // assertNull(tikhvinRef.get_extension_elements());

//   tikhvinRef.setOriginal("Tikhvin, Leningradskaya Oblast, Russia");
//   tikhvinRef.setDescriptionRef(Uri.create("#tikhvinDesc1"));
//   tikhvinRef.addExtension_element("tikhvinRef-junkExtension_element");

//   assert_eq!(tikhvinRef.getOriginal(), "Tikhvin, Leningradskaya Oblast, Russia");
//   assert_eq!(tikhvinRef.getDescriptionRef().toUri().toString(), "#tikhvinDesc1");
//   assert_eq!(tikhvinRef.findExtensionOfType(String.class), "tikhvinRef-junkExtension_element");
//   assert_eq!(tikhvinRef.toString(), "PlaceReference{original='Tikhvin, Leningradskaya Oblast, Russia', descriptionRef='#tikhvinDesc1'}");
// }

// #[test]
// fn testPlaceDescription_Luga() -> Result<(), Box<dyn Error>> {
//   let mut lugaDesc = PlaceDescription::new();

//   // assertNull(lugaDesc.get_names());
//   // assertNull(lugaDesc.get_type());
//   // assertNull(lugaDesc.get_temporal_description());
//   // assertNull(lugaDesc.get_latitude());
//   // assertNull(lugaDesc.get_longitude());
//   // assertNull(lugaDesc.get_spatial_description());
//   // assertNull(lugaDesc.get_attribution());
//   // assertNull(lugaDesc.get_extension_elements());

//   lugaDesc.setNames(new ArrayList<TextValue>());
//   lugaDesc.get_names().add(new TextValue());
//   lugaDesc.get_names().add(new TextValue());
//   lugaDesc.get_names().add(new TextValue());
//   lugaDesc.get_names().get(0).set_lang("ru-Cyrl");
//   lugaDesc.get_names().get(0).setValue("Лу́га, Новгоро́дская о́бласть, Россия");
//   lugaDesc.get_names().get(1).set_lang("ru-Latn");
//   lugaDesc.get_names().get(1).setValue("Luga, Leningradskaya Oblast', Rossiya");
//   lugaDesc.get_names().get(2).set_lang("en-Latn");
//   lugaDesc.get_names().get(2).setValue("Luga, Leningrad Oblast, Russia");
//   lugaDesc.setType(Uri.create("urn:place-authority/city"));
//   lugaDesc.set_temporal_description(new Date());
//   lugaDesc.get_temporal_description().set_formal("+1777-08-03/");
//   lugaDesc.set_latitude(58.7372222d);
//   lugaDesc.set_longitude(29.8452778d);
//   lugaDesc.set_spatial_description(new ResourceReference(Uri.create("data:application/vnd.google-earth.kml+xml;base64,\
//                                                                     "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4NCjxrbWwgeG1sbnM9Imh0dHA6\
//                                                                     "Ly93d3cub3Blbmdpcy5uZXQva21sLzIuMiIgeG1sbnM6Z3g9Imh0dHA6Ly93d3cuZ29vZ2xlLmNv\
//                                                                     "bS9rbWwvZXh0LzIuMiIgeG1sbnM6a21sPSJodHRwOi8vd3d3Lm9wZW5naXMubmV0L2ttbC8yLjIi\
//                                                                     "IHhtbG5zOmF0b209Imh0dHA6Ly93d3cudzMub3JnLzIwMDUvQXRvbSI+DQo8UGxhY2VtYXJrPg0K\
//                                                                     "CTxuYW1lPkx1Z2EsIExlbmluZ3JhZCBPYmxhc3QsIFJ1c3NpYTwvbmFtZT4NCgk8TXVsdGlHZW9t\
//                                                                     "ZXRyeT4NCgkJPFBvaW50Pg0KCQk8Y29vcmRpbmF0ZXM+MjkuODQ3OTY2LDU4LjczNTIxMywwPC9j\
//                                                                     "b29yZGluYXRlcz4NCgkJPC9Qb2ludD4NCgkJPExpbmVhclJpbmc+DQoJCQk8Y29vcmRpbmF0ZXM+\
//                                                                     "DQoJCQkJMjkuODA1NTQzMiw1OC43MDA4MjY2LDAgMjkuODA1NTQzMiw1OC43Njk1OTk0LDAgMjku\
//                                                                     "ODkwMzg4OCw1OC43Njk1OTk0LDAgMjkuODkwMzg4OCw1OC43MDA4MjY2LDAgMjkuODA1NTQzMiw1\
//                                                                     "OC43MDA4MjY2LDAgDQoJCQk8L2Nvb3JkaW5hdGVzPg0KCQk8L0xpbmVhclJpbmc+DQoJPC9NdWx0\
//                                                                     "aUdlb21ldHJ5Pg0KPC9QbGFjZW1hcms+DQo8L2ttbD4NCg==")));
//   lugaDesc.set_identifiers(new ArrayList<Identifier>());
//   lugaDesc.get_identifiers().add(new Identifier());
//   lugaDesc.get_identifiers().get(0).setKnownType(IdentifierType.Primary);
//   lugaDesc.get_identifiers().get(0).setValue(Uri.create("https://labs.familysearch.org/stdfinder/PlaceDetail.jsp?placeId=3314013#placeDescriptionId"));
//   lugaDesc.set_attribution(new Attribution());
//   lugaDesc.get_attribution().setContributor(new ResourceReference(Uri.create("urn:contributorId")));
//   lugaDesc.get_attribution().setModified(new java.util.Date(1321027871111L)); // 11 Nov 2011 11:11:11.111
//   lugaDesc.addExtension_element("lugaDesc-junkExtension_element");

//   assert_eq!(lugaDesc.get_names().get(0).get_lang(), "ru-Cyrl");
//   assert_eq!(lugaDesc.get_names().get(0).getValue(), "Лу́га, Новгоро́дская о́бласть, Россия");
//   assert_eq!(lugaDesc.get_names().get(1).get_lang(), "ru-Latn");
//   assert_eq!(lugaDesc.get_names().get(1).getValue(), "Luga, Leningradskaya Oblast', Rossiya");
//   assert_eq!(lugaDesc.get_names().get(2).get_lang(), "en-Latn");
//   assert_eq!(lugaDesc.get_names().get(2).getValue(), "Luga, Leningrad Oblast, Russia");
//   assert_eq!(lugaDesc.get_type().toUri().toString(), "urn:place-authority/city");
//   assert_eq!(lugaDesc.get_temporal_description().get_formal(), "+1777-08-03/");
//   assert_eq!(lugaDesc.get_latitude(), 58.7372222d, 0d);
//   assert_eq!(lugaDesc.get_longitude(), 29.8452778d, 0d);
//   assert_eq!(lugaDesc.get_spatial_description().get_resource().toUri().toString(), "data:application/vnd.google-earth.kml+xml;base64,\
//     "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4NCjxrbWwgeG1sbnM9Imh0dHA6\
//     "Ly93d3cub3Blbmdpcy5uZXQva21sLzIuMiIgeG1sbnM6Z3g9Imh0dHA6Ly93d3cuZ29vZ2xlLmNv\
//     "bS9rbWwvZXh0LzIuMiIgeG1sbnM6a21sPSJodHRwOi8vd3d3Lm9wZW5naXMubmV0L2ttbC8yLjIi\
//     "IHhtbG5zOmF0b209Imh0dHA6Ly93d3cudzMub3JnLzIwMDUvQXRvbSI+DQo8UGxhY2VtYXJrPg0K\
//     "CTxuYW1lPkx1Z2EsIExlbmluZ3JhZCBPYmxhc3QsIFJ1c3NpYTwvbmFtZT4NCgk8TXVsdGlHZW9t\
//     "ZXRyeT4NCgkJPFBvaW50Pg0KCQk8Y29vcmRpbmF0ZXM+MjkuODQ3OTY2LDU4LjczNTIxMywwPC9j\
//     "b29yZGluYXRlcz4NCgkJPC9Qb2ludD4NCgkJPExpbmVhclJpbmc+DQoJCQk8Y29vcmRpbmF0ZXM+\
//     "DQoJCQkJMjkuODA1NTQzMiw1OC43MDA4MjY2LDAgMjkuODA1NTQzMiw1OC43Njk1OTk0LDAgMjku\
//     "ODkwMzg4OCw1OC43Njk1OTk0LDAgMjkuODkwMzg4OCw1OC43MDA4MjY2LDAgMjkuODA1NTQzMiw1\
//     "OC43MDA4MjY2LDAgDQoJCQk8L2Nvb3JkaW5hdGVzPg0KCQk8L0xpbmVhclJpbmc+DQoJPC9NdWx0\
//     "aUdlb21ldHJ5Pg0KPC9QbGFjZW1hcms+DQo8L2ttbD4NCg==");
//   assert_eq!(lugaDesc.get_identifiers().get(0).getKnownType(), IdentifierType.Primary);
//   assert_eq!(lugaDesc.get_identifiers().get(0).getValue().toUri().toString(), "https://labs.familysearch.org/stdfinder/PlaceDetail.jsp?placeId=3314013#placeDescriptionId");
//   assert_eq!(lugaDesc.get_attribution().getContributor().get_resource().toUri().toString(), "urn:contributorId");
//   assert_eq!(lugaDesc.get_attribution().getModified().get_time(), 1321027871111L);
//   assert_eq!(lugaDesc.findExtensionOfType(String.class), "lugaDesc-junkExtension_element");
// }

// #[test]
// fn testPlaceReference_Luga() -> Result<(), Box<dyn Error>> {
//   let mut lugaRef = PlaceReference::new();

//   // assertNull(lugaRef.getOriginal());
//   // assertNull(lugaRef.getDescriptionRef());
//   // assertNull(lugaRef.get_extension_elements());

//   lugaRef.setOriginal("Luga, Leningradskaya Oblast', Russia");
//   lugaRef.setDescriptionRef(Uri.create("#lugaDesc1"));
//   lugaRef.addExtension_element("lugaRef-junkExtension_element");

//   assert_eq!(lugaRef.getOriginal(), "Luga, Leningradskaya Oblast', Russia");
//   assert_eq!(lugaRef.getDescriptionRef().toUri().toString(), "#lugaDesc1");
//   assert_eq!(lugaRef.findExtensionOfType(String.class), "lugaRef-junkExtension_element");
//   assert_eq!(lugaRef.toString(), "PlaceReference{original='Luga, Leningradskaya Oblast', Russia', descriptionRef='#lugaDesc1'}");
// }
