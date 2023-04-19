use gedcomx_model::conclusion::FamilyView;

use gedcomx_model::common::ResourceReference;
use gedcomx_model::common::Uri;
// import org.junit.Test;
// import static org.junit.Assert.*;

/**
 * Class for testing the Family class.
 * User: Randy Wilson
 * Date: 15 May 2015
 */
#[test]
fn test_family() {
    let mut family = FamilyView::new();

    // Test parents and children
    family.set_parent1(ResourceReference::new(Uri::new("#parent1"), "parent1"));
    family.set_parent2(ResourceReference::new(Uri::new("#parent2"), "parent2"));
    family.add_child(ResourceReference::new(Uri::new("#child1"), "child1"));
    family.add_child(ResourceReference::new(Uri::new("#child2"), "child2"));
    assert_eq!("parent1", family.get_parent1().unwrap().get_resource_id());
    assert_eq!(
        "#parent1",
        family.get_parent1().unwrap().get_resource().as_str()
    );
    assert_eq!(
        "#parent2",
        family.get_parent2().unwrap().get_resource().as_str()
    );
    assert_eq!(2, family.get_children().len());
    assert_eq!(
        "#child1",
        family
            .get_children()
            .get(0)
            .unwrap()
            .get_resource()
            .as_str()
    );
    assert_eq!(
        "#child2",
        family
            .get_children()
            .get(1)
            .unwrap()
            .get_resource()
            .as_str()
    );
}
