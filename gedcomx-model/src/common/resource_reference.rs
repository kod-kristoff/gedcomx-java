/**
 * Copyright Intellectual Reserve, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package org.gedcomx.common;
use crate::common::Uri;
// import com.fasterxml.jackson.annotation.JsonInclude;
// import com.webcohesion.enunciate.metadata.Facet;
// import com.webcohesion.enunciate.metadata.Facets;
// import org.gedcomx.rt.GedcomxConstants;
// import org.gedcomx.rt.json.JsonElementWrapper;

// import javax.xml.XMLConstants;
// import jakarta.xml.bind.annotation.XmlAttribute;
// import jakarta.xml.bind.annotation.XmlRootElement;
// import jakarta.xml.bind.annotation.XmlSchemaType;
// import jakarta.xml.bind.annotation.XmlType;

/**
 * A generic reference to a resource.
 *
 * @author Ryan Heaton
 */
// @XmlRootElement
// @XmlType ( name = "ResourceReference" )
// @JsonElementWrapper ( name = "resourceReference" )
// @JsonInclude ( JsonInclude.Include.NON_NULL )
pub struct ResourceReference {
    resource: Uri,
    resource_id: String,
}

impl ResourceReference {
    // pub fn ResourceReference() {
    // }

    // pub fn ResourceReference(URI resource) {
    //   this.resource = resource;
    // }

    // pub fn ResourceReference(java.net.URI resource) {
    //   this.resource = URI.create(resource.toString());
    // }

    pub fn new<U: Into<Uri>, S: Into<String>>(resource: U, resource_id: S) -> Self {
        Self::create(resource.into(), resource_id.into())
    }

    pub fn create(resource: Uri, resource_id: String) -> Self {
        Self {
            resource,
            resource_id,
        }
    }

    /**
     * The resource id of the resource being referenced. Used as an extension attribute when resolving the resource is inconvenient.
     *
     * @return The resource id of the resource being referenced.
     */
    // @XmlAttribute
    // @Facets ( {
    //   @Facet ( GedcomxConstants.FACET_GEDCOMX_RS ),
    //   @Facet ( GedcomxConstants.FACET_FS_FT_READ_ONLY )
    // } )
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    // /**
    //  * The resource id of the resource being referenced. Used as an extension attribute when resolving the resource is inconvenient.
    //  *
    //  * @param resourceId The resource id of the resource being referenced.
    //  */
    // pub fn void setResourceId(String resourceId) {
    //   this.resourceId = resourceId;
    // }

    // /**
    //  * Build up this resource reference with a resource id.
    //  *
    //  * @param resourceId The resource id.
    //  * @return this.
    //  */
    // pub fn ResourceReference resourceId(String resourceId) {
    //   setResourceId(resourceId);
    //   return this;
    // }

    /**
     * The URI to the resource. For more information, see <a href="http://www.w3.org/TR/webarch/#identification">Architecture of the World
     * Wide Web, Volume One, Section 2</a>
     *
     * @see <a href="http://www.w3.org/TR/webarch/#identification">http://www.w3.org/TR/webarch/#identification</a>
     * @return The URI to the resource.
     */
    // @XmlAttribute
    // @XmlSchemaType ( name = "anyURI", namespace = XMLConstants.W3C_XML_SCHEMA_NS_URI )
    pub fn get_resource(&self) -> &Uri {
        &self.resource
    }

    // /**
    //  * The URI to the resource. For more information, see <a href="http://www.w3.org/TR/webarch/#identification">Architecture of the World
    //  * Wide Web, Volume One, Section 2</a>
    //  *
    //  * @see <a href="http://www.w3.org/TR/webarch/#identification">http://www.w3.org/TR/webarch/#identification</a>
    //  * @param resource The URI to the resource.
    //  */
    // pub fn void setResource(URI resource) {
    //   this.resource = resource;
    // }

    // /**
    //  * Build up this resource reference with a resource.
    //  *
    //  * @param resource The resource.
    //  * @return this.
    //  */
    // pub fn ResourceReference resource(URI resource) {
    //   setResource(resource);
    //   return this;
    // }

    // /**
    //  * Provide a simple toString() method.
    //  */
    // @Override
    // pub fn String toString() {
    //   return (resource == null) ? "" : resource.toString();
    // }
}
