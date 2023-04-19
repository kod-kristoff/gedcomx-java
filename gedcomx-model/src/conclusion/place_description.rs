// /**
//  * Copyright Intellectual Reserve, Inc.
//  *
//  * Licensed under the Apache License, Version 2.0 (the "License");
//  * you may not use this file except in compliance with the License.
//  * You may obtain a copy of the License at
//  *
//  *   http://www.apache.org/licenses/LICENSE-2.0
//  *
//  * Unless required by applicable law or agreed to in writing, software
//  * distributed under the License is distributed on an "AS IS" BASIS,
//  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  * See the License for the specific language governing permissions and
//  * limitations under the License.
//  */
// package org.gedcomx.conclusion;
use crate::conclusion::Date;

// import com.fasterxml.jackson.annotation.JsonInclude;
// import com.fasterxml.jackson.annotation.JsonProperty;
// import com.webcohesion.enunciate.metadata.Facet;
// import org.gedcomx.common.*;
use crate::common::ResourceReference;
use crate::common::TextValue;
use crate::common::Uri;
// import org.gedcomx.links.Link;
// import org.gedcomx.rt.GedcomxConstants;
// import org.gedcomx.rt.GedcomxModelVisitor;
// import org.gedcomx.source.SourceDescription;
// import org.gedcomx.source.SourceReference;
// import org.gedcomx.types.ConfidenceLevel;

// import jakarta.xml.bind.annotation.XmlAttribute;
// import jakarta.xml.bind.annotation.XmlElement;
// import jakarta.xml.bind.annotation.XmlType;
// import java.util.ArrayList;
// import java.util.LinkedList;
// import java.util.List;
// import java.util.stream.Stream;

// /**
//  * A PlaceDescription is used to describe the details of a place in terms of its name
//  * and possibly its type, time period, and/or a geospatial description -- a description
//  * of a place as a snapshot in time.
//  */
// @XmlType ( name = "PlaceDescription", propOrder = { "names", "temporalDescription", "latitude", "longitude", "spatialDescription", "place", "jurisdiction", "displayExtension" } )
// @JsonInclude ( JsonInclude.Include.NON_NULL )
pub struct PlaceDescription {
    // extends Subject {
    names: Vec<TextValue>,
    r#type: Option<Uri>,
    temporalDescription: Date,
    latitude: Option<f64>,
    longitude: Option<f64>,
    //   private ResourceReference place;
    spatialDescription: Option<ResourceReference>,
    //   private ResourceReference jurisdiction;
    //   private PlaceDisplayProperties display;
}

impl Default for PlaceDescription {
    fn default() -> Self {
        Self {
            names: Vec::default(),
            r#type: None,
            temporalDescription: Date::default(),
            latitude: None,
            longitude: None,
            spatialDescription: None,
        }
    }
}

impl PlaceDescription {
    pub fn new() -> Self {
        Self::default()
    }
    //   @Override
    //   pub fn PlaceDescription id(String id) {
    //     return (PlaceDescription) super.id(id);
    //   }

    //   @Override
    //   pub fn PlaceDescription extensionElement(Object element) {
    //     return (PlaceDescription) super.extensionElement(element);
    //   }

    //   @Override
    //   pub fn PlaceDescription link(String rel, URI href) {
    //     return (PlaceDescription) super.link(rel, href);
    //   }

    //   @Override
    //   pub fn PlaceDescription link(Link link) {
    //     return (PlaceDescription) super.link(link);
    //   }

    //   @Override
    //   pub fn PlaceDescription lang(String lang) {
    //     return (PlaceDescription) super.lang(lang);
    //   }

    //   @Override
    //   pub fn PlaceDescription confidence(URI confidence) {
    //     return (PlaceDescription) super.confidence(confidence);
    //   }

    //   @Override
    //   pub fn PlaceDescription confidence(ConfidenceLevel confidence) {
    //     return (PlaceDescription) super.confidence(confidence);
    //   }

    //   @Override
    //   pub fn PlaceDescription source(SourceReference sourceReference) {
    //     return (PlaceDescription) super.source(sourceReference);
    //   }

    //   @Override
    //   pub fn PlaceDescription source(SourceDescription source) {
    //     return (PlaceDescription) super.source(source);
    //   }

    //   @Override
    //   pub fn PlaceDescription note(Note note) {
    //     return (PlaceDescription) super.note(note);
    //   }

    //   @Override
    //   pub fn PlaceDescription analysis(ResourceReference analysis) {
    //     return (PlaceDescription) super.analysis(analysis);
    //   }

    //   @Override
    //   pub fn PlaceDescription attribution(Attribution attribution) {
    //     return (PlaceDescription) super.attribution(attribution);
    //   }

    //   @Override
    //   pub fn PlaceDescription analysis(Document analysis) {
    //     return (PlaceDescription) super.analysis(analysis);
    //   }

    //   @Override
    //   pub fn PlaceDescription analysis(URI analysis) {
    //     return (PlaceDescription) super.analysis(analysis);
    //   }

    //   @Override
    //   pub fn PlaceDescription extracted(Boolean extracted) {
    //     return (PlaceDescription) super.extracted(extracted);
    //   }

    //   @Override
    //   pub fn PlaceDescription identifier(Identifier identifier) {
    //     return (PlaceDescription) super.identifier(identifier);
    //   }

    //   @Override
    //   pub fn PlaceDescription evidence(EvidenceReference evidence) {
    //     return (PlaceDescription) super.evidence(evidence);
    //   }

    //   pub fn PlaceDescription evidence(PlaceDescription evidence) {
    //     if (evidence.getId() == null) {
    //       throw new IllegalArgumentException("Unable to add event as evidence: no id.");
    //     }
    //     return (PlaceDescription) super.evidence(new EvidenceReference(URI.create("#" + evidence.getId())));
    //   }

    //   @Override
    //   pub fn PlaceDescription media(SourceReference media) {
    //     return (PlaceDescription) super.media(media);
    //   }

    //   @Override
    //   pub fn PlaceDescription media(SourceDescription media) {
    //     return (PlaceDescription) super.media(media);
    //   }

    //   @Override
    //   pub fn PlaceDescription sortKey(String sortKey) {
    //     return (PlaceDescription) super.sortKey(sortKey);
    //   }

    //   /**
    //    * Create a stream for the names.
    //    *
    //    * @return a stream for the names.
    //    */
    //   pub fn Stream<TextValue> names() {
    //     return self.names == null ? Stream.empty() : self.names.stream();
    //   }

    /**
     * An ordered list of standardized (or normalized), fully-qualified (in terms of what is known of the applicable jurisdictional hierarchy) names for this place that are applicable to this description of this place.
     *
     * The list MUST include at least one value. It is RECOMMENDED that instances include a single name and any equivalents from other cultural contexts;
     * name variants should more typically be described in separate PlaceDescription instances.  The list is assumed to be given in order of preference,
     * with the most preferred value in the first position in the list.
     *
     * @return An ordered list of standardized (or normalized), fully-qualified (in terms of what is known of the applicable jurisdictional hierarchy) names for this place that are applicable to this description of this place.
     */
    // @XmlElement (name = "name")
    // @JsonProperty ("names")
    pub fn get_names(&self) -> &Vec<TextValue> {
        &self.names
    }

    pub fn get_names_mut(&mut self) -> &mut Vec<TextValue> {
        &mut self.names
    }
    //   /**
    //    * An ordered list of standardized (or normalized), fully-qualified (in terms of what is known of the applicable jurisdictional hierarchy) names for this place that are applicable to this description of this place.
    //    *
    //    * The list MUST include at least one value. It is RECOMMENDED that instances include a single name and any equivalents from other cultural contexts;
    //    * name variants should more typically be described in separate PlaceDescription instances.  The list is assumed to be given in order of preference,
    //    * with the most preferred value in the first position in the list.
    //    *
    //    * @param names An ordered list of standardized (or normalized), fully-qualified (in terms of what is known of the applicable jurisdictional hierarchy) names for this place that are applicable to this description of this place.
    //    */
    //   @JsonProperty ("names")
    //   pub fn setNames(List<TextValue> names) {
    //     self.names = names;
    //   }

    //   /**
    //    * Build out this description with a name.
    //    *
    //    * @param name The name.
    //    * @return self.
    //    */
    //   pub fn PlaceDescription name(TextValue name) {
    //     addName(name);
    //     return this;
    //   }

    //   /**
    //    * Build out this description with a name.
    //    *
    //    * @param name The name.
    //    * @return self.
    //    */
    //   pub fn PlaceDescription name(String name) {
    //     addName(new TextValue(name));
    //     return this;
    //   }

    //   /**
    //    * Add a name to the list of standardized names.
    //    *
    //    * @param name The name to be added.
    //    */
    //   pub fn addName(TextValue name) {
    //     if (name != null) {
    //       if (names == null) {
    //         names = new LinkedList<TextValue>();
    //       }
    //       names.add(name);
    //     }
    //   }

    /**
     * An implementation-specific uniform resource identifier (URI) used to identify the type of a place (e.g., address, city, county, province, state, country, etc.).
     *
     * @return An implementation-specific uniform resource identifier (URI) used to identify the type of a place (e.g., address, city, county, province, state, country, etc.).
     */
    //   @XmlAttribute
    //   @Facet ( GedcomxConstants.FACET_FS_FT_UNSUPPORTED )
    pub fn get_type(&self) -> Option<&Uri> {
        self.r#type.as_ref()
    }

    /**
     * An implementation-specific uniform resource identifier (URI) used to identify the type of a place (e.g., address, city, county, province, state, country, etc.).
     *
     * @param type An implementation-specific uniform resource identifier (URI) used to identify the type of a place (e.g., address, city, county, province, state, country, etc.).
     */
    pub fn set_type(&mut self, r#type: Uri) {
        self.r#type = Some(r#type);
    }

    //   /**
    //    * Build out this place description with a type.
    //    * @param type The type.
    //    * @return self.
    //    */
    //   pub fn PlaceDescription type(URI type) {
    //     set_type(type);
    //     return this;
    //   }

    /**
     * A description of the time period to which this place description is relevant.
     *
     * @return A description of the time period to which this place description is relevant.
     */
    //   @Facet ( GedcomxConstants.FACET_FS_FT_UNSUPPORTED )
    pub fn get_temporal_description(&self) -> &Date {
        &self.temporalDescription
    }

    pub fn get_temporal_description_mut(&mut self) -> &mut Date {
        &mut self.temporalDescription
    }

    /**
     *  A description of the time period to which this place description is relevant.
     *
     * @param temporalDescription A description of the time period to which this place description is relevant.
     */
    pub fn set_temporal_description(&mut self, temporalDescription: Date) {
        self.temporalDescription = temporalDescription;
    }

    //   /**
    //    * Build out this place description with a temporal description.
    //    * @param temporalDescription the temporal description.
    //    * @return self.
    //    */
    //   pub fn PlaceDescription temporal_description(Date temporalDescription) {
    //     set_temporal_description(temporalDescription);
    //     return this;
    //   }

    /**
     * Degrees north or south of the Equator (0.0 degrees).   Values range from −90.0 degrees (south) to 90.0 degrees (north).
     *
     * @return Degrees north or south of the Equator.
     */
    pub fn get_latitude(&self) -> Option<f64> {
        self.latitude
    }

    /**
     * Degrees north or south of the Equator (0.0 degrees).   Values range from −90.0 degrees (south) to 90.0 degrees (north).
     *
     * @param latitude Degrees north or south of the Equator.
     */
    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = Some(latitude);
    }

    //   /**
    //    * Build out this place description with a latitude.
    //    *
    //    * @param latitude The latitude.
    //    * @return self.
    //    */
    //   pub fn PlaceDescription latitude(Double latitude) {
    //     set_latitude(latitude);
    //     return this;
    //   }

    /**
     * Angular distance in degrees, relative to the Prime Meridian.   Values range from −180.0 degrees (west of the Meridian) to 180.0 degrees (east of the Meridian).
     *
     * @return Angular distance in degrees, relative to the Prime Meridian.
     */
    pub fn get_longitude(&self) -> Option<f64> {
        self.longitude
    }

    /**
     * Angular distance in degrees, relative to the Prime Meridian.   Values range from −180.0 degrees (west of the Meridian) to 180.0 degrees (east of the Meridian).
     *
     * @param longitude Angular distance in degrees, relative to the Prime Meridian.
     */
    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = Some(longitude);
    }

    //   /**
    //    * Build out this place description with a longitude.
    //    * @param longitude The longitude.
    //    * @return self.
    //    */
    //   pub fn PlaceDescription longitude(Double longitude) {
    //     set_longitude(longitude);
    //     return this;
    //   }

    /**
     * A reference to a geospatial description of this place.
     *
     * It is RECOMMENDED that this description resolve to a KML document.
     *
     * @return  A reference to a geospatial description of this place.
     */
    //   @Facet ( GedcomxConstants.FACET_FS_FT_UNSUPPORTED )
    pub fn get_spatial_description(&self) -> Option<&ResourceReference> {
        self.spatialDescription.as_ref()
    }

    /**
     *  A reference to a geospatial description of this place.
     *
     *  It is RECOMMENDED that this description resolve to a KML document.
     *
     * @param spatialDescription A reference to a geospatial description of this place.
     */
    pub fn set_spatial_description(&mut self, spatialDescription: ResourceReference) {
        self.spatialDescription = Some(spatialDescription);
    }

    //   /**
    //    * Build out this place description with a spacial description.
    //    * @param spatialDescription The spatial description.
    //    * @return this
    //    */
    //   pub fn PlaceDescription spatial_description(ResourceReference spatialDescription) {
    //     set_spatial_description(spatialDescription);
    //     return this;
    //   }

    //   /**
    //    * A reference to a description of the jurisdiction this place.
    //    *
    //    * @return A reference to a description of the jurisdiction this place.
    //    */
    //   pub fn ResourceReference getJurisdiction() {
    //     return jurisdiction;
    //   }

    //   /**
    //    *  A reference to a description of the jurisdiction this place.
    //    *
    //    * @param jurisdiction A reference to a description of the jurisdiction this place.
    //    */
    //   pub fn setJurisdiction(ResourceReference jurisdiction) {
    //     self.jurisdiction = jurisdiction;
    //   }

    //   /**
    //    * Build out this place description with a jurisdiction.
    //    * @param jurisdiction The reference to the jurisdiction.
    //    * @return this
    //    */
    //   pub fn PlaceDescription jurisdiction(ResourceReference jurisdiction) {
    //     setJurisdiction(jurisdiction);
    //     return this;
    //   }

    //   /**
    //    * A reference to the place being described.
    //    *
    //    * @return A reference to the place being described.
    //    */
    //   pub fn ResourceReference getPlace() {
    //     return place;
    //   }

    //   /**
    //    *  A reference to the place being described.
    //    *
    //    * @param place A reference to the place being described.
    //    */
    //   pub fn setPlace(ResourceReference place) {
    //     self.place = place;
    //   }

    //   /**
    //    * Build out this place description with a place.
    //    *
    //    * @param place The reference to the place.
    //    * @return this
    //    */
    //   pub fn PlaceDescription place(ResourceReference place) {
    //     setPlace(place);
    //     return this;
    //   }

    //   /**
    //    * Display properties for the place. Display properties are not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    *
    //    * @return Display properties for the place. Display properties are not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    */
    //   @XmlElement(name = "display")
    //   @JsonProperty("display")
    //   @Facet ( GedcomxConstants.FACET_GEDCOMX_RS )
    //   pub fn PlaceDisplayProperties getDisplayExtension() {
    //     return display;
    //   }

    //   /**
    //    * Display properties for the place. Display properties are not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    *
    //    * @param display Display properties for the place. Display properties are not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    */
    //   @JsonProperty("display")
    //   pub fn setDisplayExtension(PlaceDisplayProperties display) {
    //     self.display = display;
    //   }

    //   /**
    //    * Build out this place with a display exension.
    //    *
    //    * @param display the display.
    //    * @return this
    //    */
    //   pub fn PlaceDescription displayExtension(PlaceDisplayProperties display) {
    //     setDisplayExtension(display);
    //     return this;
    //   }

    //   /**
    //    * Accept a visitor.
    //    *
    //    * @param visitor The visitor.
    //    */
    //   pub fn accept(GedcomxModelVisitor visitor) {
    //     visitor.visitPlace_description(this);
    //   }

    //   /**
    //    * Embed another place.
    //    *
    //    * @param place The place to embed.
    //    */
    //   pub fn embed(PlaceDescription place) {
    //     if (place.names != null) {
    //       self.names = self.names == null ? new ArrayList<TextValue>() : self.names;
    //       self.names.addAll(place.names);
    //     }
    //     self.type = self.type == null ? place.type : self.type;
    //     self.temporalDescription = self.temporalDescription == null ? place.temporalDescription : self.temporalDescription;
    //     self.latitude = self.latitude == null ? place.latitude : self.latitude;
    //     self.longitude = self.longitude == null ? place.longitude : self.longitude;
    //     self.spatialDescription = self.spatialDescription == null ? place.spatialDescription : self.spatialDescription;
    //     self.jurisdiction = self.jurisdiction == null ? place.jurisdiction : self.jurisdiction;
    //     if (self.display != null && place.display != null) {
    //       self.display.embed(place.display);
    //     }
    //     super.embed(place);
    //   }
}
