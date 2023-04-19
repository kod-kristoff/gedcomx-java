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

// import com.fasterxml.jackson.annotation.JsonCreator;
// import com.fasterxml.jackson.annotation.JsonIgnore;
// import com.fasterxml.jackson.annotation.JsonInclude;
// import com.fasterxml.jackson.annotation.JsonValue;
// import com.webcohesion.enunciate.metadata.qname.XmlQNameEnumRef;
// import org.gedcomx.common.URI;
// import org.gedcomx.rt.json.HasJsonKey;
use crate::types::IdentifierType;

// import jakarta.xml.bind.annotation.XmlAttribute;
// import jakarta.xml.bind.annotation.XmlTransient;
// import jakarta.xml.bind.annotation.XmlType;
// import jakarta.xml.bind.annotation.XmlValue;

use crate::common::Uri;

// /**
//  * An identifier for a resource.
//  *
//  * @author Ryan Heaton
//  */
// @XmlType ( name = "Identifier" )
// @JsonInclude ( JsonInclude.Include.NON_NULL )
pub struct Identifier {
    // implements HasJsonKey {

    //   private boolean hasUniqueKey = false;
    //   private URI
    value: Uri,

    /**
     * @see org.gedcomx.types.IdentifierType
     */
    known_type: IdentifierType,
    //   private URI type;
}

impl Identifier {
    //   pub fn Identifier() {
    //   }

    //   @JsonCreator
    //   pub fn Identifier(URI value) {
    //     this.value = value;
    //   }

    pub fn new(known_type: IdentifierType, value: Uri) -> Self {
        Self { value, known_type }
    }
    //   /**
    //    * The id value.
    //    *
    //    * @return The id value.
    //    */
    //   @XmlValue
    //   @JsonValue
    //   pub fn URI getValue() {
    //     return value;
    //   }

    //   /**
    //    * The id value.
    //    *
    //    * @param value The id value.
    //    */
    //   @JsonValue
    //   pub fn void setValue(URI value) {
    //     this.value = value;
    //   }

    //   /**
    //    * Build up this identifier with a value.
    //    *
    //    * @param value The value.
    //    * @return this.
    //    */
    //   pub fn Identifier value(URI value) {
    //     setValue(value);
    //     return this;
    //   }

    //   /**
    //    * The type of the id.
    //    *
    //    * @return The type of the id.
    //    */
    //   @XmlAttribute
    //   @JsonIgnore
    //   @XmlQNameEnumRef (IdentifierType.class)
    //   pub fn URI getType() {
    //     return type;
    //   }

    //   /**
    //    * The type of the id.
    //    *
    //    * @param type The type of the id.
    //    */
    //   @JsonIgnore
    //   pub fn void setType(URI type) {
    //     this.type = type;
    //   }

    //   /**
    //    * Build up this identifier with a type.
    //    * @param type The type.
    //    * @return this.
    //    */
    //   pub fn Identifier type(URI type) {
    //     setType(type);
    //     return this;
    //   }

    //   /**
    //    * Build up this identifier with a type.
    //    * @param type The type.
    //    * @return this.
    //    */
    //   pub fn Identifier type(IdentifierType type) {
    //     setKnownType(type);
    //     return this;
    //   }

    //   /**
    //    * The type of the id.
    //    *
    //    * @param type The type of the id.
    //    * @param unique Whether the type of this identifier implies that the value is unique among all other identifiers of the same type.
    //    */
    //   pub fn void setType(URI type, boolean unique) {
    //     this.type = type;
    //     this.hasUniqueKey = unique;
    //   }

    //   /**
    //    * The enum referencing a known identifier type.
    //    *
    //    * @return The enum referencing a known identifier type, or {@link org.gedcomx.types.IdentifierType#OTHER} if not known.
    //    */
    //   @XmlTransient
    //   @JsonIgnore
    //   pub fn IdentifierType getKnownType() {
    //     return getType() == null ? null : IdentifierType.fromQNameURI(getType());
    //   }

    //   /**
    //    * Set the value of the id type from a known identifier type.
    //    *
    //    * @param knownType The known identifier type.
    //    */
    //   @JsonIgnore
    //   pub fn void setKnownType(IdentifierType knownType) {
    //     setType(knownType == null ? null : knownType.toQNameURI());
    //   }

    //   @XmlTransient
    //   @JsonIgnore
    //   @Override
    //   pub fn boolean isHasUniqueKey() {
    //     return this.hasUniqueKey;
    //   }

    //   @XmlTransient
    //   @JsonIgnore
    //   @Override
    //   pub fn String getJsonKey() {
    //     return this.type == null ? null : this.type.toString();
    //   }

    //   @JsonIgnore
    //   @Override
    //   pub fn void setJsonKey(String jsonKey) {
    //     this.type = new URI(jsonKey);
    //   }

    //   /**
    //    * Provide a simple toString() method.
    //    */
    //   @Override
    //   pub fn String toString() {
    //     return (value == null) ? "" : value.toString();
    //   }
}
