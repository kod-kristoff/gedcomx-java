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
// package org.gedcomx.common;

// import com.fasterxml.jackson.annotation.JsonInclude;
// import com.webcohesion.enunciate.metadata.Facet;
// import org.gedcomx.rt.GedcomxConstants;

// import javax.xml.XMLConstants;
// import jakarta.xml.bind.annotation.XmlAttribute;
// import jakarta.xml.bind.annotation.XmlType;
// import jakarta.xml.bind.annotation.XmlValue;

// /**
//  * An element representing a text value that may be in a specific language.
//  */
// @XmlType ( name = "TextValue" )
// @JsonInclude ( JsonInclude.Include.NON_NULL )
pub struct TextValue {
    lang: String,
    value: String,
}

impl Default for TextValue {
    fn default() -> Self {
        Self {
            lang: String::default(),
            value: String::default(),
        }
    }
}
impl TextValue {
    pub fn new() -> Self {
        Self::default()
    }
    //   }

    //   pub TextValue(String value) {
    //     this.value = value;
    //   }

    /**
     * The language of the text value. See <a href="http://www.w3.org/International/articles/language-tags/">http://www.w3.org/International/articles/language-tags/</a>
     *
     * @return The language of the text value.
     */
    //   @XmlAttribute( namespace = XMLConstants.XML_NS_URI )
    //   @Facet ( GedcomxConstants.FACET_FS_FT_READ_ONLY )
    pub fn get_lang(&self) -> &str {
        self.lang.as_str()
    }

    /**
     * The language of the text value. See <a href="http://www.w3.org/International/articles/language-tags/">http://www.w3.org/International/articles/language-tags/</a>
     *
     * @param lang The language of the text value.
     */
    pub fn set_lang(&mut self, lang: String) {
        self.lang = lang;
    }

    /**
     * Build up this text value with a lang.
     *
     * @param lang The lang.
     * @return this.
     */
    pub fn lang<S: Into<String>>(mut self, lang: S) -> Self {
        self.set_lang(lang.into());
        self
    }

    /**
     * The text value.
     *
     * @return The text value.
     */
    //   @XmlValue
    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    /**
     * The text value.
     *
     * @param value The text value.
     */
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /**
     * Build up this text value with a value.
     *
     * @param value The value.
     * @return this.
     */
    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.set_value(value.into());
        self
    }

    //   @Override
    //   pub boolean equals( Object o ) {
    //     if (this == o) {
    //       return true;
    //     }
    //     if (o == null || getClass() != o.getClass()) {
    //       return false;
    //     }

    //     TextValue textValue = (TextValue) o;

    //     if (lang != null ? !lang.equals( textValue.lang ) : textValue.lang != null) {
    //       return false;
    //     }
    //     if (value != null ? !value.equals( textValue.value ) : textValue.value != null) {
    //       return false;
    //     }

    //     return true;
    //   }

    //   @Override
    //   pub int hashCode() {
    //     int result = lang != null ? lang.hashCode() : 0;
    //     result = 31 * result + (value != null ? value.hashCode() : 0);
    //     return result;
    //   }

    //   @Override
    //   pub String toString() {
    //     return "TextValue{" +
    //       "value='" + value + '\'' +
    //       ", lang='" + lang + '\'' +
    //       '}';
    //   }
}
