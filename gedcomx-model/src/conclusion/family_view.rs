/**
 * Copyright Intellectual Reserve, Inc.
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
 */
// package org.gedcomx.conclusion;

// import com.fasterxml.jackson.annotation.Js3xnInclude;
// import com.webcohesion.enunciate.metadata.Facet;
// import com.fasterxml.jackson.annotation.JsonProperty;
use crate::common::ResourceReference;
// use crate::links.HypermediaEnabledData;
// use crate::rt.GedcomxConstants;
// use crate::rt.json.JsonElementWrapper;

// import jakarta.xml.bind.annotation.XmlElement;
// import jakarta.xml.bind.annotation.XmlRootElement;
// import jakarta.xml.bind.annotation.XmlType;
// import java.util.ArrayList;
// import java.util.List;

/**
 * A family view, meaning up to two parents and a list of children who have those parents in common.
 * Relationships carry the canonical information for this view, and the relationships must be used
 * to get Facts (lineage types, marriages, etc.) about the relationships covered by a Family.
 * The Family data type provides a convenient way to see the typical family views without having to do
 * the calculations to derive them. There should only be one family for each unique set of parents,
 * and only one for each single-parent family with a particular parent.
 */
// @XmlRootElement( name = "family" )
// @Facet (GedcomxConstants.FACET_GEDCOMX_RS)
// @JsonElementWrapper( name = "families" )
// @XmlType( name = "FamilyView", propOrder = { "parent1", "parent2", "children"} )
// @JsonInclude ( JsonInclude.Include.NON_NULL )
pub struct FamilyView {
    // extends HypermediaEnabledData {

    // First parent
    parent1: Option<ResourceReference>,
    // Second parent
    parent2: Option<ResourceReference>,
    // List of children
    children: Vec<ResourceReference>,
}

impl Default for FamilyView {
    fn default() -> Self {
        Self {
            parent1: None,
            parent2: None,
            children: Vec::default(),
        }
    }
}
impl FamilyView {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * A reference to a parent in the family. The name "parent1" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     *
     * @return A reference to a parent in the family. The name "parent1" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     */
    pub fn get_parent1(&self) -> Option<&ResourceReference> {
        self.parent1.as_ref()
    }

    /**
     * A reference to a parent in the family. The name "parent1" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     *
     * @param parent1 A reference to a parent in the family. The name "parent1" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     */
    pub fn set_parent1(&mut self, parent1: ResourceReference) {
        self.parent1 = Some(parent1);
    }

    // /**
    //  * Build out this family with a reference to parent1.
    //  *
    //  * @param parent1 _parent 1.
    //  * @return this.
    //  */
    // pub fn FamilyView parent1(ResourceReference parent1) {
    //   set_parent1(parent1);
    //   return this;
    // }

    /**
     * A reference to a parent in the family. The name "parent2" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     *
     * @return A reference to a parent in the family. The name "parent2" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     */
    pub fn get_parent2(&self) -> Option<&ResourceReference> {
        self.parent2.as_ref()
    }

    /**
     * A reference to a parent in the family. The name "parent2" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     *
     * @param parent2 A reference to a parent in the family. The name "parent2" is used only to distinguish it from
     * the other parent in this family and implies neither order nor role.
     */
    pub fn set_parent2(&mut self, parent2: ResourceReference) {
        self.parent2 = Some(parent2);
    }

    // /**
    //  * Build out this family with a reference to parent2.
    //  *
    //  * @param parent2 _parent 2.
    //  * @return this.
    //  */
    // pub fn FamilyView parent2(ResourceReference parent2) {
    //   set_parent2(parent2);
    //   return this;
    // }

    /**
     * A list of references to the children of this family.
     *
     * @return A list of references to the children of this family.
     */
    // @XmlElement(name="child")
    // @JsonProperty("children")
    pub fn get_children(&self) -> &[ResourceReference] {
        self.children.as_slice()
    }

    // /**
    //  * A list of references to the children of this family.
    //  *
    //  * @param children A list of references to the children of this family.
    //  */
    // @JsonProperty("children")
    // pub fn void set_children(List<ResourceReference> children) {
    //   this.children = children;
    // }

    // /**
    //  * Build out this family by adding a child.
    //  *
    //  * @param child The child to add.
    //  * @return this.
    //  */
    // pub fn FamilyView child(ResourceReference child) {
    //   add_child(child);
    //   return this;
    // }

    /**
     * Add a child.
     *
     * @param child The child to add.
     */
    pub fn add_child(&mut self, child: ResourceReference) {
        self.children.push(child);
    }

    // pub fn void embed(FamilyView family) {
    //   this.parent1 = this.parent1 == null ? family.parent1 : this.parent1;
    //   this.parent2 = this.parent2 == null ? family.parent2 : this.parent2;
    //   if (family.children != null) {
    //     if (children == null) {
    //       children = new ArrayList<ResourceReference>();
    //     }
    //     children.addAll(family.children);
    //   }
    // }
}
