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

// import com.fasterxml.jackson.annotation.JsonIgnore;
// import com.fasterxml.jackson.annotation.JsonInclude;
// import com.fasterxml.jackson.annotation.JsonProperty;
// import com.webcohesion.enunciate.metadata.ClientName;
// import com.webcohesion.enunciate.metadata.Facet;
// import com.webcohesion.enunciate.metadata.qname.XmlQNameEnumRef;
// import org.gedcomx.common.ExtensibleData;
// import org.gedcomx.common.TextValue;
// import org.gedcomx.common.URI;
// import org.gedcomx.date.GedcomxDate;
// import org.gedcomx.records.Field;
// import org.gedcomx.records.HasFields;
// import org.gedcomx.rt.GedcomxConstants;
// import org.gedcomx.rt.GedcomxModelVisitor;
// import org.gedcomx.types.ConfidenceLevel;

// import jakarta.xml.bind.annotation.XmlAttribute;
// import jakarta.xml.bind.annotation.XmlElement;
// import jakarta.xml.bind.annotation.XmlTransient;
// import jakarta.xml.bind.annotation.XmlType;
// import java.util.LinkedList;
// import java.util.List;
// import java.util.stream.Stream;

// /**
//  * A concluded genealogical date.
//  */
// @ClientName ("DateInfo")
// @XmlType ( name = "Date", propOrder = { "original", "formal", "normalizedExtensions", "fields"})
// @JsonInclude ( JsonInclude.Include.NON_NULL )
pub struct Date {
    // extends ExtensibleData implements HasFields {

    //   private String original;
    formal: String,
    //   private URI confidence;
    //   private List<TextValue> normalized;
    //   private List<Field> fields;
}

impl Default for Date {
    fn default() -> Self {
        Self {
            formal: String::default(),
        }
    }
}

impl Date {
    pub fn new() -> Self {
        Self::default()
    }

    //   @Override
    //   pub fn Date id(String id) {
    //     return (Date) super.id(id);
    //   }

    //   @Override
    //   pub fn Date extensionElement(Object element) {
    //     return (Date) super.extensionElement(element);
    //   }

    //   /**
    //    * The original text as supplied by the user.
    //    *
    //    * @return The original text as supplied by the user.
    //    */
    //   pub fn String getOriginal() {
    //     return original;
    //   }

    //   /**
    //    * The original text as supplied by the user.
    //    *
    //    * @param original The original text as supplied by the user.
    //    */
    //   pub fn void setOriginal(String original) {
    //     self.original = original;
    //   }

    //   /**
    //    * Build up this date with original text as supplied by the user.
    //    *
    //    * @param original the original text.
    //    * @return self.
    //    */
    //   pub fn Date original(String original) {
    //     setOriginal(original);
    //     return this;
    //   }

    /**
     * The standardized and/or normalized formal value.
     *
     * @return The formal value.
     */
    pub fn get_formal(&self) -> &str {
        self.formal.as_str()
    }

    /**
     * The standardized and/or normalized formal value.
     *
     * @param formal The formal value.
     */
    pub fn set_formal(&mut self, formal: String) {
        self.formal = formal;
    }

    //   /**
    //    * The standardized and/or normalized formal value.
    //    *
    //    * @param formal The formal value.
    //    */
    //   @XmlTransient
    //   @JsonIgnore
    //   pub fn void set_formalDate(GedcomxDate formal) {
    //     if (formal != null) {
    //       set_formal(formal.toFormalString());
    //     }
    //   }

    //   /**
    //    * Build up this date with a formal representation of the date.
    //    *
    //    * @param formal The formal date.
    //    * @return self.
    //    */
    //   pub fn Date formal(String formal) {
    //     set_formal(formal);
    //     return this;
    //   }

    //   /**
    //    * Build up this date with a formal representation of the date.
    //    *
    //    * @param formal The formal date.
    //    * @return self.
    //    */
    //   pub fn Date formal(GedcomxDate formal) {
    //     return formal(formal.toFormalString());
    //   }

    //   /**
    //    * The level of confidence the contributor has about the data.
    //    *
    //    * @return The level of confidence the contributor has about the data.
    //    */
    //   @XmlAttribute
    //   @XmlQNameEnumRef(ConfidenceLevel.class)
    //   pub fn URI getConfidence() {
    //     return confidence;
    //   }

    //   /**
    //    * The level of confidence the contributor has about the data.
    //    *
    //    * @param confidence The level of confidence the contributor has about the data.
    //    */
    //   pub fn void setConfidence(URI confidence) {
    //     self.confidence = confidence;
    //   }

    //   /**
    //    * Build up this conclusion with a confidence level.
    //    *
    //    * @param confidence The confidence level.
    //    * @return self.
    //    */
    //   pub fn Date confidence(URI confidence) {
    //     setConfidence(confidence);
    //     return this;
    //   }

    //   /**
    //    * Build up this conclusion with a confidence level.
    //    *
    //    * @param confidence The confidence level.
    //    * @return self.
    //    */
    //   pub fn Date confidence(ConfidenceLevel confidence) {
    //     setKnownConfidenceLevel(confidence);
    //     return this;
    //   }

    //   /**
    //    * The value of a the known confidence level, or {@link org.gedcomx.types.ConfidenceLevel#OTHER} if not known.
    //    *
    //    * @return The value of a the known confidence level, or {@link org.gedcomx.types.ConfidenceLevel#OTHER} if not known.
    //    */
    //   @XmlTransient
    //   @JsonIgnore
    //   pub fn ConfidenceLevel getKnownConfidenceLevel() {
    //     return getConfidence() == null ? null : ConfidenceLevel.fromQNameURI(getConfidence());
    //   }

    //   /**
    //    * Set the confidence level from a known enumeration of confidence levels.
    //    *
    //    * @param level The known level.
    //    */
    //   @JsonIgnore
    //   pub fn void setKnownConfidenceLevel(ConfidenceLevel level) {
    //     setConfidence(level == null ? null : level.toQNameURI());
    //   }

    //   /**
    //    * Create a stream for the normalized value extensions.
    //    *
    //    * @return a stream for the normalized value extensions.
    //    */
    //   pub fn Stream<TextValue> normalizedExtensions() {
    //     return self.normalized == null ? Stream.empty() : self.normalized.stream();
    //   }

    //   /**
    //    * The list of normalized values for the date, provided for display purposes by the application. Normalized values are
    //    * not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    *
    //    * @return The list of normalized values for the date, provided for display purposes by the application. Normalized values
    //    * are not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    */
    //   @XmlElement ( name = "normalized" )
    //   @JsonProperty ("normalized")
    //   @Facet ( GedcomxConstants.FACET_GEDCOMX_RS )
    //   pub fn List<TextValue> getNormalizedExtensions() {
    //     return normalized;
    //   }

    //   /**
    //    * The list of normalized values for the date, provided for display purposes by the application. Normalized values are
    //    * not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    *
    //    * @param normalized The list of normalized values for the date, provided for display purposes by the application. Normalized values are
    //    * not specified by GEDCOM X core, but as extension elements by GEDCOM X RS.
    //    */
    //   @JsonProperty ("normalized")
    //   pub fn void setNormalizedExtensions(List<TextValue> normalized) {
    //     self.normalized = normalized;
    //   }

    //   /**
    //    * Add a normalized extension to the list.
    //    *
    //    * @param normalizedExtension The normalizedExtension to be added.
    //    */
    //   pub fn void addNormalizedExtension(TextValue normalizedExtension) {
    //     if (normalizedExtension != null) {
    //       if (normalized == null) {
    //         normalized = new LinkedList<TextValue>();
    //       }
    //       normalized.add(normalizedExtension);
    //     }
    //   }

    //   /**
    //    * Get the fields being used as evidence.
    //    *
    //    * @return The references to the record fields being used as evidence.
    //    */
    //   @XmlElement( name = "field" )
    //   @JsonProperty( "fields" )
    //   @Facet ( GedcomxConstants.FACET_GEDCOMX_RECORD )
    //   pub fn List<Field> get_fields() {
    //     return fields;
    //   }

    //   /**
    //    * Set the list of fields being used as evidence.
    //    *
    //    * @param fields - List of fields
    //    */
    //   @JsonProperty( "fields" )
    //   pub fn void set_fields(List<Field> fields) {
    //     self.fields = fields;
    //   }

    //   /**
    //    * Build up this date with a field.
    //    *
    //    * @param field The field.
    //    * @return self.
    //    */
    //   pub fn Date field(Field field) {
    //     addField(field);
    //     return this;
    //   }

    //   /**
    //    * Add a reference to the record field values being used as evidence.
    //    *
    //    * @param field The field to be added.
    //    */
    //   pub fn void addField(Field field) {
    //     if (field != null) {
    //       if (fields == null) {
    //         fields = new LinkedList<Field>();
    //       }
    //       fields.add(field);
    //     }
    //   }

    //   @Override
    //   pub fn String toString() {
    //     return "Date{" + "original='" + original + '\'' + ", formal=" + formal + '}';
    //   }

    //   /**
    //    * Accept a visitor.
    //    *
    //    * @param visitor The visitor.
    //    */
    //   pub fn void accept(GedcomxModelVisitor visitor) {
    //     visitor.visitDate(this);
    //   }
}
